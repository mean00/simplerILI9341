#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
extern crate sdl2;

extern crate ili9341;

use ili9341::simpler9341 as simpler9341;
use ili9341::access::Ili9341Access as Ili9341Access;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::rect::Point;
use sdl2::EventPump;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;

//---

struct sdlAccess <'a>
{
    canvas :  &'a mut Canvas<Window>,
    x1 : usize,
    x2 : usize,
    y1 : usize,
    y2 : usize,
    x  : usize,
    y  : usize,
}
impl sdlAccess <'_>
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
        self.canvas.present();
    }
}
//-------
impl Ili9341Access for sdlAccess <'_>
{
    fn send_byte(&mut self,  b : u8)
    {
            self.canvas.present();
    }
    
    fn send_word(&mut self,  color : u16)
    {
        let r= color >> 11;
        let g = (color >> 6) & 0x1f;
        let b = color & 0x3f;
        let color = pixels::Color::RGB((r*8) as u8,(g*4) as u8 ,(b*8) as u8);
        self.canvas.set_draw_color(color);
        self.canvas.draw_point( Point::new(self.x as i32, self.y as i32));        
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
        self.flush();
    }
    fn data_begin(&mut self, )
    {
        self.flush();
    }   
}

fn exit_requested(events : &  mut EventPump) -> bool
{    
    for event in events.poll_iter() 
    {
        match event {
            Event::Quit { .. } => {return true;},
            Event::KeyDown {
                keycode: Some(keycode),
                ..
            } => {
                if keycode == Keycode::Escape 
                    {return true;}
            }
            _ => {}
        }        
    }
    return false;
}

//--

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "rust-sdl2_gfx: draw line & FPSManager",
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let  mut events = sdl_context.event_pump()?;

    let mut access = sdlAccess{ canvas : &mut canvas , x1: 0, x2: 0, y1: 0, y2  :0 , x : 0, y:0 };

    let ili = ili9341::simpler9341::Ili9341::new( SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &mut  access);
    'main: loop {
        ili.fill_screen(0x0);
        ili.draw_line(10,10,200,200,0x1f);
        ili.draw_line(10,200,200,10,0x1f);
        while true
        {
        if exit_requested(& mut events)        {            break 'main;        }
        }

        ili.fill_screen(0xf<<6);
        if exit_requested(& mut events)        {            break 'main;        }

        ili.fill_screen(0xf<<11);
        if exit_requested(& mut events)        {            break 'main;        }
    }
    



    
    Ok(())
}
/*
'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => {
                    if keycode == Keycode::Escape {
                        break 'main;
                    } else if keycode == Keycode::Space {
                        println!("space down");
                        for i in 0..400 {
                            canvas.pixel(i as i16, i as i16, 0xFF000FFu32)?;
                        }
                        canvas.present();
                    }
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    let _ = canvas.line(lastx, lasty, x as i16, y as i16, color);
                    lastx = x as i16;
                    lasty = y as i16;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
    }

    */
