
#![allow(dead_code)]
extern crate alloc;
//use alloc::boxed::Box;
//
use crate::util::unsafe_array_alloc as unsafe_array_alloc;
use crate::util::unsafe_box_allocate as unsafe_box_allocate;
//
use crate::glyph::{PFXfont,PFXglyph,FontInfo};
//
use crate::access::Ili9341Access;
//
const ST7735_BUFFER_SIZE_WORD : usize = 320;
mod geometry;
mod text;


enum FontFamily
{
    SmallFont=0,
    MediumFont=1,
    BigFont=2,
}
//-----------
pub struct Ili9341 <'a>
{
    physical_width   : usize,
    physical_height  : usize,
    width           : usize,
    height          : usize,
    rotation        : usize,
    physical_x_offset : usize,
    physical_y_offset : usize,
    x_offset         : usize,
    y_offset         : usize,  
    src_buf         : *mut u16,
    access          : &'a mut dyn Ili9341Access,
    current_font    : &'a FontInfo,
    font_infos      : [FontInfo;3],
    cursor_x        : usize,
    cursor_y        : usize,

    fg              : u16,
    bg              : u16,
}

impl <'a>Ili9341<'a>
{
    //-------------------------------------------------------------------------------
    fn _init(&'a mut self,w: usize, h:usize, access: &'a mut dyn Ili9341Access, smallfont :  &'static PFXfont, mediumfont:  &'static PFXfont, bigfont :  &'static PFXfont ) 
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
        self.src_buf            = unsafe_array_alloc(ST7735_BUFFER_SIZE_WORD);
        self.access             = access;

        
        self.font_infos[0].font       = smallfont;        
        self.font_infos[1].font       = mediumfont;
        self.font_infos[2].font       = bigfont;        
        self.current_font= &(self.font_infos[0]);
        for i in 0..3
        {
            // TODO TO DO
          //  self.check_font(  self.font_infos[0].font , &mut self.font_infos[i]  );
        }        
    }   
    //-------------------------------------------------------------------------------
    pub fn new (w: usize, h:usize, access: &'a mut dyn Ili9341Access, 
                smallfont :  &'static PFXfont, mediumfont:  &'static PFXfont, bigfont :  &'static PFXfont ) 
                -> &'a mut Ili9341 <'a>
    {
        // there is probably a better way to do this
        // we dont want to use the stack (even temporarily) as it will overflow
        unsafe {
            let  allocated :  *mut Ili9341   = unsafe_box_allocate();
            (*allocated)._init(w,h,access,smallfont,mediumfont,bigfont);        
        // We normally never free this, so a mem leak is a not a big deal            
            return &mut (*allocated);
        }
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
    fn color_map(&self, color : u16) -> u16
    {
        return color;

       // if(!IS_7789()) return d;
       // uint32_t r=(d>>11),b=d&0x1f,g=(d>>5)&0x3f;
       // return r+(g<<5)+(b<<11);
    
    }
}

// EOF
