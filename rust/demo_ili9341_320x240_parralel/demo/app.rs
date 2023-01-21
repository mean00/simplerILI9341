#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate alloc;

use alloc::boxed::Box;
use cty::c_char;
use rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnGpio::rnPin as rnPin;
use rn::rnExti as rnExti;
use rn::rnOsHelper::rnLogger as rnLogger;
use rnarduino::rnSpi::rnPin::{NoPin};
use rnarduino::rnSpi::rnSpiBitOrder::*;
use rnarduino::rnSpi::rnSPISettings;

use ili9341::ili9341::Ili9341;
use lnspi_ili9341::spi_ili9341 as spi_ili9341;


use crate::testfont::NotoSans_Bold20pt7b;

use ili9341::ili9341_init_sequence::{DSO_RESET,DSO_WAKEUP};

pub const ILI_PIN_DC         : rnPin =  rnPin::PA4 ;
pub const ILI_PIN_CS         : rnPin =  rnPin::PB1 ;
pub const ILI_PIN_RESET      : rnPin =  rnPin::PB0 ;



pub struct runTime
{
   
}

/**
 * 
 * 
 */
const FAST : u32 =1;
impl runTime
{
   /**
    *     
    */
   fn new() -> runTime
   {
      let t : runTime = runTime
         {
         };         
         t      
   }
   
   
   /**
    * 
    */
   fn run(&mut self) -> ()
   {          
      let transaction : rnSPISettings  = rnSPISettings{
         speed: FAST*36*1000*1000+(1-FAST)*10*1000, 
         bOrder : SPI_MSBFIRST, 
         dMode : 0, 
         pinCS : rnPin::NoPin};

      let mut spi = rn::rnSpi::rnSPI::new(0,36*1000*1000);
      spi.begin();
      spi.begin_transaction(&transaction);

      let mut ili_access = spi_ili9341::new(spi, ILI_PIN_CS, ILI_PIN_DC,ILI_PIN_RESET);
      // init low level
      ili_access.reset();
      ili_access.send_init_sequence(DSO_RESET);
      ili_access.send_init_sequence(DSO_WAKEUP);
      // Send it over to real ili
      let  ili = Ili9341::new(240,320, 
                     &mut ili_access, 
                     &NotoSans_Bold20pt7b, &NotoSans_Bold20pt7b ,&NotoSans_Bold20pt7b);

      let bitmap_width = 96;
      let bitmap_height = 96;
      let bitmap = include_bytes!("test_bitmap.bin");
                 
      ili.set_rotation(1);
      ili.fill_screen(0);  
       
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
      
      ili.select_font( ili9341::ili9341::FontFamily::BigFont);
      ili.print(5,95,"Some  text");
      ili.drawHSBitmap(bitmap_width, bitmap_height, 40,80, ili9341::colors::GREEN, ili9341::colors::BLACK, bitmap);


      loop
      {   
       
      }  
   }
}
/**
 * \fn rnLoop
 * 
 * 
 */
#[no_mangle]
pub extern "C" fn rnLoop() -> ()
{
      let r : runTime = runTime::new();
      let boxed : Box<runTime> = Box::new(r);
      let mut boxed2 : Box<runTime>;
        
      let ptr = Box::into_raw(boxed);
      unsafe {    

            boxed2 = Box::from_raw(ptr);
         }
      boxed2.run();
}

#[no_mangle]
pub extern "C" fn rnInit() -> ()
{
   rnLogger("Setuping up Power Supply...\n");
   
}
// EOF
