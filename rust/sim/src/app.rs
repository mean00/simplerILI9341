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


mod testfont;
mod testfont2C;
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
    loop {
    loops+=1;
    if loops > 5
    {
        break;
    }
    clear_background(macroquad::color::BLACK);

    let mut access = quadAccess{  x1: 0, x2: 0, y1: 0, y2  :0 , x : 0, y:0 };

    let ili = simpler9341::Ili9341::new( SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &mut  access,
            &DejaVuSans20pt7b, //NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b    
            );
    
    ili.fill_screen(0x0);
    //next_frame().await;
    ili.draw_line(10,10,200,200,ili9341::colors::BLUE); // \
    //next_frame().await;
    ili.draw_line(10,200,200,10,ili9341::colors::BLUE); // /
    //next_frame().await;
    ili.draw_line(10,200,10,10,ili9341::colors::BLUE);  // ^ Left
    //next_frame().await;
    ili.draw_line(200,10,200,200,ili9341::colors::BLUE);// | right
    //next_frame().await;
    ili.draw_line(200,200,10,200,ili9341::colors::BLUE);// _ Bottom
    //next_frame().await;
    ili.draw_line(10,10,200,10,ili9341::colors::BLUE);// - top
    //next_frame().await;
    ili.circle(60,60,24,ili9341::colors::RED);
    ili.disc(120,60,24,ili9341::colors::GREEN);

    ili.inverted_disc_corner(200,40, 30,4,ili9341::colors::BLUE);
    ili.inverted_disc_corner(80,120, 30,1,ili9341::colors::RED);
    
    
    ili.fill_round_rect( 20,220,100,16,4,ili9341::colors::RED,ili9341::colors::GREEN);
   
    ili.set_text_color(ili9341::colors::RED,ili9341::colors::BLUE);

    ili.print(5,35,"Some  text");
    ili.set_text_color(ili9341::colors::rgb(0xff,0xff,0xff), 0);
    ili.print(5,65,"Some  text");
    ili.set_text_color(ili9341::colors::RED,ili9341::colors::BLUE);
    
    ili.select_font( simpler9341::FontFamily::BigFont);
    ili.print(5,95,"Some  text");
    ili.drawHSBitmap(bitmap_width, bitmap_height, 40,80, ili9341::colors::GREEN, ili9341::colors::BLACK, bitmap);


    next_frame().await;
    }
    std::println!("Exiting....");
}
