#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
use macroquad::prelude::*;
extern crate ili9341;

use ili9341::ili9341 as simpler9341;
use ili9341::access::Ili9341Access as Ili9341Access;
use ili9341::colors::GREEN;
use ili9341::colors::BLUE;
use ili9341::colors::RED;
use ili9341::colors::BLACK;
use ili9341::ili9341::gauge_meter::Gauge;

extern crate std;
use std::thread;
use std::time;



mod testfont;
mod testfont2C;
mod rust_logo_nc;



use crate::testfont::NotoSans_Bold20pt7b;
use crate::testfont2C::DejaVuSans20pt7b;

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
//-------
fn full_to_unit( c : u16 , shift: usize, range : usize) -> f32
{
    let mut f= c;
    f=f>>shift;
    if range==6
    {
        f&=0x3f;
    }else {
        f&=0x1f;
    }
    f<<= 8-range;
    let mut m=f as f32;
    m=m/255.;
    if m>1.0
    {
        m=1.0;
    }
    m
}
//-------
impl Ili9341Access for quadAccess 
{
    fn send_byte(&mut self,  b : u8)
    {

    }
   
    fn send_word(&mut self,  color : u16)
    {
        let   r : f32=  full_to_unit( color, 11,5);
        let   g : f32 = full_to_unit( color, 5, 6);
        let   b : f32 = full_to_unit( color, 0, 5);

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
    let mut loops = 0;
    let bitmap_width = 96;
    let bitmap_height = 96;
    let bitmap = include_bytes!("test_bitmap.bin");

    let ten_millis = time::Duration::from_millis(1000);

    loop {
    loops+=1;
    if loops > 5000
    {
        break;
    }
    clear_background(macroquad::color::BLACK);

    let access = Box::new(quadAccess{  x1: 0, x2: 0, y1: 0, y2  :0 , x : 0, y:0 });

    let mut ili = simpler9341::Ili9341::new( SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, access,
            &DejaVuSans20pt7b, //NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b    
            );
    
    ili.fill_screen(0x0);
   
    let mut gauge=Gauge::new(40,64);
    let mut percent : isize =60;
    let mut increment : isize =2;
    loop
    {
        gauge.draw(percent as usize, &mut ili,100,100, ili9341::colors::RED);
        percent+=increment;
        next_frame().await;
        if percent<4 && increment < 0
        {
            increment=-increment;
        }
        if percent>95 && increment > 0
        {
            increment = -increment;
        }
        thread::sleep(ten_millis);
    }
    }
    std::println!("Exiting....");
}
