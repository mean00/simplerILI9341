#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use macroquad::prelude::*;
extern crate ili9341;

use ili9341::simpler9341 as simpler9341;
use ili9341::access::Ili9341Access as Ili9341Access;



mod testfont;
use crate::testfont::NotoSans_Bold12pt7b;

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
        if self.x > self.x2
        {
            self.x = self.x1;
            self.y +=1;
        }
        if self.y > self.y2
        {
            self.y = self.y1;        
        }
    }
    fn flush(&mut self)
    {
        
    }
}
const iRED :   u16  = (0x1f<<11);
const iGREEN : u16  = (0x3f<<5);
const iBLUE :  u16  = (0x1f<<0);
const iBLACK : u16  = 0;
const iWHITE : u16  = 0xffff;
//-------
impl Ili9341Access for quadAccess 
{
    fn send_byte(&mut self,  b : u8)
    {

    }
    
    fn send_word(&mut self,  color : u16)
    {
        let   r : f32= ((color >> 11)*8) as f32;
        let   g : f32 = (((color >> 5) & 0x1f)*4) as f32;
        let   b : f32 = ((color & 0x1f)*8) as f32;

        let ix= (self.x as i32)*2;
        let iy= (self.y as i32)*2;

        let color = Color::new(r,g,b,1.0);
        draw_rectangle(ix as f32, iy as f32, 2.,2.,color);
        self.next();
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

    let ili = ili9341::simpler9341::Ili9341::new( SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &mut  access,
    &NotoSans_Bold12pt7b,&NotoSans_Bold12pt7b,&NotoSans_Bold12pt7b    
    );
    
    ili.fill_screen(0x0);
    //next_frame().await;
    ili.draw_line(10,10,200,200,iBLUE); // \
    //next_frame().await;
    ili.draw_line(10,200,200,10,iBLUE); // /
    //next_frame().await;
    ili.draw_line(10,200,10,10,iBLUE);  // ^ Left
    //next_frame().await;
    ili.draw_line(200,10,200,200,iBLUE);// | right
    //next_frame().await;
    ili.draw_line(200,200,10,200,iBLUE);// _ Bottom
    //next_frame().await;
    ili.draw_line(10,10,200,10,iBLUE);// - top
    //next_frame().await;
    ili.circle(60,60,24,iRED);
    ili.disc(120,60,24,iGREEN);

    ili.inverted_disc_corner(200,40, 30,4,iBLUE);
    ili.inverted_disc_corner(80,120, 30,1,iRED);
    
    
    ili.fill_round_rect( 20,220,100,16,4,iRED,iGREEN);
   
    ili.set_text_color(iRED,iBLUE);

    
    ili.print(5,35,"Some  text");
    ili.print(5,65,"Some  text");
    ili.print(5,95,"Some  text");

    next_frame().await;
    }
}
