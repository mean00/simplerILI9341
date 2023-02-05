
#![allow(dead_code)]
extern crate alloc;
use alloc::boxed::Box;
//use crate::util::unsafe_array_alloc;
//
use simpler_gfx::{PFXfont,FontInfo};
//
use crate::access::Ili9341Access;
//
use crate::settings::ST7735_BUFFER_SIZE_WORD;
//
mod sin;
mod geometry;
pub mod gauge_meter;
mod gauge;
mod text;
mod text_1nc_r;
mod text_2nc_r;
mod bitmap;


#[cfg(feature = "hs")]
use heatshrink_byte as heatshrink;
#[cfg(feature = "hs")]
mod text_1c_r;
#[cfg(feature = "hs")]
mod text_2c_r;

#[cfg(not(feature = "hs"))]
mod text_nohs;

#[derive(Copy, Clone)]
pub enum FontFamily
{
    SmallFont=0,
    MediumFont=1,
    BigFont=2,
}
//-----------
pub struct Ili9341 <'a>
{
    physical_width      : usize,
    physical_height     : usize,
    width               : usize,
    height              : usize,
    rotation            : usize,
    physical_x_offset   : usize,
    physical_y_offset   : usize,
    x_offset            : usize,
    y_offset            :  usize,  
    src_buf             : &'a mut [u16],
    access              : Box< dyn Ili9341Access>,
    current_font_index  : FontFamily,
    font_infos          : [FontInfo;3],
    cursor_x            : usize,
    cursor_y            : usize,

    fg                  : u16,
    bg                  : u16,

    chip_id             : u16,

    #[cfg(feature = "hs")]
    hs              : heatshrink::HeatshrinkDecoder<'a>,
}
//-----------------
impl <'a>Ili9341<'a>
{
    fn current_font(&self) -> &FontInfo
    {        
        let ix = self.current_font_index as usize;
        return &self.font_infos[ix];
    }
    //-------------------------------------------------------------------------------
    fn _init(&'a mut self,w: usize, h:usize, access:  Box< dyn Ili9341Access>, 
            smallfont :  &'static PFXfont, mediumfont:  &'static PFXfont, bigfont :  &'static PFXfont ) 
    {
        self.physical_width     = w;
        self.physical_height    = h;
        self.width = w;
        self.height = h;
        self.rotation = h;
        self.physical_x_offset  = 0;
        self.physical_y_offset  = 0;
        self.x_offset           = 0;
        self.y_offset           = 0;     
        self.cursor_x           = 0;   
        self.cursor_y           = 0;
        self.fg                 = 0xffff;
        self.bg                 = 0;
        self.src_buf            =  crate::util::unsafe_slice_alloc::<u16>(ST7735_BUFFER_SIZE_WORD);
        self.access             = access;
        #[cfg(feature = "hs")]
        self.hs                 = heatshrink::HeatshrinkDecoder::new( &[], &(heatshrink::Config::new(7,4).unwrap()));
        
        self.font_infos[0].font       = smallfont;        
        self.font_infos[1].font       = mediumfont;
        self.font_infos[2].font       = bigfont;                
        self.check_font( );
        self.current_font_index       = FontFamily::SmallFont;
    }   
    //-------------------------------------------------------------------------------
    pub fn new (w: usize, h:usize, access:  Box< dyn Ili9341Access>, 
                smallfont :  &'static PFXfont, 
                mediumfont:  &'static PFXfont, 
                bigfont :  &'static PFXfont )                ->   Box<Ili9341 <'a>>
    {
        // there is probably a better way to do this
        // we dont want to use the stack (even temporarily) as it will overflow
        /*
        unsafe {
            let  allocated :  *mut Ili9341   = unsafe_box_allocate();
            (*allocated)._init(w,h,access,smallfont,mediumfont,bigfont);        
        // We normally never free this, so a mem leak is a not a big deal            
            return &mut (*allocated);
            */
            //let mut access = access;
            
            let  mut allocated  = Box::new(Ili9341{
                physical_width     : w,
                physical_height    : h,
                width : w,
                height : h,
                rotation : h,
                physical_x_offset  : 0,
                physical_y_offset  : 0,
                x_offset           : 0,
                y_offset           : 0,     
                cursor_x           : 0,   
                cursor_y           : 0,
                fg                 : 0xffff,
                bg                 : 0,
                src_buf            : crate::util::unsafe_slice_alloc::<u16>(ST7735_BUFFER_SIZE_WORD),
                access             : access,
                #[cfg(feature = "hs")]
                hs                 : heatshrink::HeatshrinkDecoder::new( &[], &(heatshrink::Config::new(7,4).unwrap())),
                
                chip_id             : 0x9341,

                current_font_index : FontFamily::SmallFont,
                font_infos          :[
                                    FontInfo{  
                                        max_height  : 0, 
                                        max_width   : 0, 
                                        font        : smallfont,
                                    },
                                    FontInfo{  
                                        max_height  : 0, 
                                        max_width   : 0, 
                                        font        : mediumfont,
                                    },
                                    FontInfo{  
                                        max_height  : 0, 
                                        max_width   : 0, 
                                        font        : bigfont,
                                    } ],
            });
            allocated.check_font( );
            return allocated;        
    }
    //-------------------------------------------------------------------------------
    pub fn fill_screen(&mut self, color : u16 ) 
    {
        const ONE_GO: usize = 320; // If we DMA too much, we may overflow, it should be elsewhere...
        if self.height > ONE_GO // bug ? int color, int x, int y, int w, int h
        {
            self.square(color,0,0,self.width,ONE_GO);
            self.square(color,0,ONE_GO,self.width,self.height-ONE_GO);
        }
        else
        {
            self.square(color,0,0,self.width,self.height);
        }
    }
    //-------------------------------------------------------------------------------
    pub fn set_rotation(&mut self,rotation : usize)
    {
        self.rotation=rotation;
        match rotation
        {
            0|2 => {
                self.x_offset    =self.physical_x_offset;
                self.y_offset    =self.physical_y_offset;
                self.width      =self.physical_width;
                self.height     =self.physical_height;
                },
            1|3 => {
                    self.x_offset    =self.physical_y_offset;
                    self.y_offset    =self.physical_x_offset;
                    self.width      =self.physical_height;
                    self.height     =self.physical_width;    
                }
            _ => panic!("oops")
        }
        self.access.update_hw_rotation(rotation);
    }
    //-----------------------
    // Warning: The colors must be correct !
    //-----------------------
    pub fn send_data(&mut self, x: usize, y : usize, data : &[u16])
    {
        self.access.set_address(x,y,data.len(),1);
        self.access.push_colors(data);
    }
    //-------------------------------------------------------------------------------
    pub fn square(&mut self, color : u16, x : usize, y : usize, w: usize, h:usize)
    {
        self.access.set_address(x,y,w,h);
        self.access.data_begin();
        let f=w*h;
        self.access.flood_words(f,color);
        self.access.data_end();
    }
    //-------------------------------------------------------------------------------   
    pub fn circle_advance(xx: &mut usize, yy: &mut usize, e: &mut isize) {
        if (*e) > 0 {
            (*xx) -= 1;
            *e = (*e) - 8 * (*xx as isize);
        }
        (*yy) += 1;
        (*e) += 8 * (*yy as isize) + 4;
    }
}

// EOF
