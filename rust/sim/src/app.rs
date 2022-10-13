#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use macroquad::prelude::*;
extern crate ili9341;

use ili9341::simpler9341 as simpler9341;
use ili9341::access::Ili9341Access as Ili9341Access;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;

struct quadAccess 
{    
    x1 : usize,
    x2 : usize,
    y1 : usize,
    y2 : usize,
    x  : usize,
    y  : usize,
}
impl quadAccess 
{
    fn next(&mut self)
    {
        self.x+=1;
        if(self.x > self.x2)
        {
            self.x = self.x1;
            self.y +=1;
        }
        if(self.y > self.y2)
        {
            self.y = self.y1;        
        }
    }
    fn flush(&mut self)
    {
        
    }
}
//-------
impl Ili9341Access for quadAccess 
{
    fn send_byte(&mut self,  b : u8)
    {

    }
    
    fn send_word(&mut self,  color : u16)
    {
        let mut r : f32= (color >> 11) as f32;
        let mut g : f32 = ((color >> 5) & 0x1f) as f32;
        let mut b : f32 = (color & 0x1f) as f32;

        let ix= (self.x as i32)*2;
        let iy= (self.y as i32)*2;

        let color = Color::new(r,g,b,1.0);
        draw_rectangle(ix as f32, iy as f32, 2.,2.,color);
    }
    fn update_hw_rotation(&mut self, rotation  : usize )
    {
        self.flush();
    }
    fn set_address(&mut self,  x: usize, y : usize, w: usize, h:usize)
    {
        self.x1=x;
        self.x2=x+w-1;
        self.y1=y;
        self.y2=y+h-1;
        self.x=self.x1;
        self.y=self.y1;
        self.flush();
    }
    fn data_end(&mut self, )
    {
       
    }
    fn data_begin(&mut self, )
    {
       
    }   
}

//---
#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
    clear_background(BLACK);

    let mut access = quadAccess{  x1: 0, x2: 0, y1: 0, y2  :0 , x : 0, y:0 };

    let ili = ili9341::simpler9341::Ili9341::new( SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &mut  access);
    
    ili.fill_screen(0x0);
    ili.draw_line(10,10,200,200,0x1f); // \
    ili.draw_line(10,200,200,10,0x1f); // /
    ili.draw_line(10,200,10,10,0x1f);  // ^ Left
    ili.draw_line(200,200,10,200,0x1f);// _ Bottom
    ili.circle(60,60,24,(0x3f)<<5);

    
    next_frame().await;
    }
}
