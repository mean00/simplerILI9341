#![allow(non_camel_case_types)]
#![allow(unused_imports)]
extern crate sdl2;

extern crate ili9341;

use ili9341::simpler9341 as simpler9341;
use ili9341::access::Ili9341Access as Ili9341Access;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::render::Canvas;
use sdl2::video::Window;

use sdl2::gfx::primitives::DrawRenderer;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;

//---

struct sdlAccess <'a>
{
    canvas :  &'a mut Canvas<Window>,
}
//-------
impl Ili9341Access for sdlAccess <'_>
{
    fn send_byte(&mut self,  b : u8)
    {
            self.canvas.present();
    }
    fn send_word(&mut self,  b : u16)
    {
        self.canvas.present();
    }
    fn update_hw_rotation(&mut self, rotation  : usize )
    {
        self.canvas.present();
    }
    fn set_address(&mut self,  x: usize, y : usize, w: usize, h:usize)
    {
        self.canvas.present();
    }
    fn data_end(&mut self, )
    {

    }
    fn data_begin(&mut self, )
    {

    }   
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

    let mut lastx = 0;
    let mut lasty = 0;

    let mut events = sdl_context.event_pump()?;

    let mut access = sdlAccess{ canvas : &mut canvas};

    let ili = ili9341::simpler9341::Ili9341::new( SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, &mut  access);
    loop {
        ili.fill_screen(0xf);
        ili.fill_screen(0xf<<6);
        ili.fill_screen(0xf<<11);
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
