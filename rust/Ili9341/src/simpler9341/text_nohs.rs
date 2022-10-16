use super :: Ili9341;
#[cfg(feature = "sim")]
extern crate std;


impl <'a>Ili9341<'a>
{
    pub fn innerLoop2C(&mut self, w: usize, h : usize, left: usize, line_size: usize, fg: u16, bg : u16, p : *const u8)
    {        
        panic!("No heatshrink!");
    }
    pub fn innerLoop1C(&mut self, w: usize, h : usize, left: usize, line_size: usize, fg: u16, bg : u16, p : *const u8)
    {        
        panic!("No heatshrink!");
    }

}
