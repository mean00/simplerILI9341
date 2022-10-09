
#![allow(dead_code)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::alloc::Layout as Layout;
use alloc::alloc::alloc as alloc;
//
use crate::access::Ili9341Access;
//
const ST7735_BUFFER_SIZE_WORD : usize = 320;
//
//https://stackoverflow.com/questions/59232877/how-to-allocate-structs-on-the-heap-without-taking-up-space-on-the-stack-in-stab
pub fn unsafe_box_allocate<T>() -> Box<T> 
{
    let instance: Box<T>;
    let layout = Layout::new::<T>();
    unsafe {           
        let ptr = alloc(layout) as *mut T;     
        instance = Box::from_raw(ptr);
    }
    instance
}
//-----------
pub fn unsafe_array_alloc<T>(count : usize ) -> *mut T 
{
    
        let layout = Layout::new::<T>();
        let unit = layout.size();
        let align = layout.align();
    unsafe {  
        let big_layout = Layout::from_size_align_unchecked(unit * count, align);

            
        let ptr = alloc(big_layout) as *mut T;     
        ptr
    }    
}
//-----------
struct Ili9341 <'a>
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
}

impl <'a>Ili9341<'a>
{
    //-------------------------------------------------------------------------------
    fn _init(&'a mut self,w: usize, h:usize, access: &'a mut dyn Ili9341Access)
    {
        self.physical_width     = w;
        self.physical_height    = h;
        self.width = w;
        self.height = h;
        self.rotation = h;
        self.physical_x_offset = 0;
        self.physical_y_offset = 0;
        self.x_offset         = 0;
        self.y_offset         = 0;        
        self.src_buf         = unsafe_array_alloc(ST7735_BUFFER_SIZE_WORD);
        self.access = access;
    }
    //-------------------------------------------------------------------------------
    fn new (w: usize, h:usize, access: &'a mut dyn Ili9341Access) -> Option<Box<Ili9341>>
    {
        let mut allocated :  Box<Ili9341>   = unsafe_box_allocate();
        allocated._init(w,h,access);        
        //Some(allocated)
        None
    }
    //-------------------------------------------------------------------------------
    fn fill_screen(&mut self, color : u16 ) 
    {
        const ONE_GO: usize = 320; 
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
    fn set_rotation(&mut self,rotation : usize)
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
    fn square(&mut self, color : u16, x : usize, y : usize, w: usize, h:usize)
    {
        self.access.set_address(x,y,w,h);
        let f=w*h;
        self.access.flood_words(f,color);
    }
    //-------------------------------------------------------------------------------
}

// EOF
