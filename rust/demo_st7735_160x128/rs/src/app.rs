#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate alloc;
use core::include_bytes;
use alloc::boxed::Box;
use cty::c_char;
use rnarduino as rn;

use rn::rn_gpio as rnGpio;
use rn::rn_gpio::rnPin as rnPin;
use rn::rn_exti as rnExti;
use rnarduino::rn_spi::rnSPI;
use rnarduino::rn_spi::rnPin::{NoPin};
use rnarduino::rn_spi::rnSpiBitOrder::*;
use rnarduino::rn_spi::rnSPISettings;

use ili9341::ili9341::Ili9341;
use lnspi_ili9341::spi_ili9341 as spi_ili9341;


use crate::testfont::NotoSans_Bold20pt7b;

use crate::st7735_init::{WW,HH,ST7735};

pub const ILI_PIN_DC         : rnPin =  rnPin::PB11 ;
pub const ILI_PIN_CS         : rnPin =  rnPin::PB8 ;
pub const ILI_PIN_RESET      : rnPin =  rnPin::PB9 ;

pub const SLOW_SPEED      : u32 =  50*1000;
pub const FAST_SPEED      : u32 =  36*1000*1000;

rn::lnLogger_init!();
use rn::lnLogger;

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
       let speed = FAST*FAST_SPEED +(1-FAST)*SLOW_SPEED; 
      let transaction : rnSPISettings  = rnSPISettings{
         speed ,
         bOrder : SPI_MSBFIRST, 
         dMode : 0, 
         pinCS : rnPin::NoPin};

      let mut spi = rnSPI::new(0,speed);
      spi.set(&transaction);

      let mut ili_access = spi_ili9341::new(spi, ILI_PIN_CS, ILI_PIN_DC,ILI_PIN_RESET);
      // init low level
      ili_access.reset();
      ili_access.send_init_sequence(ST7735);
      //ili_access.send_init_sequence(DSO_WAKEUP);
      // Send it over to real ili
      let  mut ili = Ili9341::new(HH,WW, 
                     Box::new(ili_access), 
                     &NotoSans_Bold20pt7b, &NotoSans_Bold20pt7b ,&NotoSans_Bold20pt7b);

      let bitmap_width = 96;
      let bitmap_height = 96;
      let bitmap = include_bytes!("test_bitmap.bin");
      let mut toggle = true;                 
      ili.set_rotation(1);
      loop
      {   
        if(toggle) {
            ili.fill_screen(0);  
            ili.draw_bitmap_hs(bitmap_width, bitmap_height, 4,4, ili9341::colors::GREEN, ili9341::colors::BLACK, bitmap);
            ili.circle(60,60,24,ili9341::colors::RED);
            ili.disc(120,60,24,ili9341::colors::BLUE);  
            ili.print(5,35,"grn/blk");
        }else {
            ili.fill_screen(0);  
            ili.draw_bitmap_hs(bitmap_width, bitmap_height, 4,4, ili9341::colors::RED, ili9341::colors::BLUE, bitmap);
            ili.circle(60,60,24,ili9341::colors::GREEN);
            ili.disc(120,60,24,ili9341::colors::BLACK);  
            ili.print(5,35,"red/blue");
            
        }
        toggle=!toggle;
        rn::rn_os_helper::delay_ms(1000);

       
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
   lnLogger!("Setuping up Power Supply...\n");
   
}
// EOF
