/*
 * Basic demo
 *
 */
extern crate alloc;
use alloc::boxed::Box;
use core::include_bytes;
use rnarduino as rn;

use rn::rn_gpio as rnGpio;
use rn::rn_gpio::rnPin;
use rnarduino::rn_spi::rnSPI;
use rnarduino::rn_spi::rnSPISettings;
use rnarduino::rn_spi::rnSpiBitOrder::*;

use ili9341::ili9341::Ili9341;
use lnspi_ili9341::SpiIli9341;
// Pick the right pinout
#[cfg(not(feature = "rp2040"))]
use crate::pinout_bp as pin;
#[cfg(feature = "rp2040")]
use crate::pinout_rp2040 as pin;

use pin::{ILI_PIN_CS, ILI_PIN_DC, ILI_PIN_RESET, ILI_PIN_SPI11, ILI_PIN_SPI12};
use ili9341::ili9341_init_sequence::{DSO_RESET,DSO_WAKEUP};
use crate::testfont::NotoSans_Bold20pt7b;

const HH : usize = 320;
const WW : usize = 240;
//
pub const SLOW_SPEED: u32 = 50 * 1000;
pub const FAST_SPEED: u32 = 36 * 1000 * 1000;
const FAST: u32 = 1;

rn::lnLogger_init!();
use rn::lnLogger;

/*
 * Dummy context
 */
pub struct runTime {}

/*
 *
 *
 */
impl runTime {
    /*
     *
     */
    fn new() -> runTime {
        let t: runTime = runTime {};
        t
    }

    /*
     *
     */
    fn run(&mut self) {
        // spi pins
        rnGpio::pinMode(ILI_PIN_SPI11, rnGpio::rnGpioMode::lnSPI_MODE);
        rnGpio::pinMode(ILI_PIN_SPI12, rnGpio::rnGpioMode::lnSPI_MODE);

        let speed = FAST * FAST_SPEED + (1 - FAST) * SLOW_SPEED;
        let transaction: rnSPISettings = rnSPISettings {
            speed,
            bOrder: SPI_MSBFIRST,
            dMode: 0,
            pinCS: rnPin::NoPin,
        };

        // init low level
        let mut spi = rnSPI::new(0, speed);
        spi.set(&transaction);

        // init access
        let mut ili_access = SpiIli9341::new(spi, ILI_PIN_CS, ILI_PIN_DC, ILI_PIN_RESET);
        ili_access.reset();
        ili_access.send_init_sequence(DSO_RESET);
        ili_access.send_init_sequence(DSO_WAKEUP);
        ili_access.set_chip_id(0x9341);
        // init ILI9341/ST7735
        let mut ili = Ili9341::new(
            WW,
            HH,
            Box::new(ili_access),
            &NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b,
            &NotoSans_Bold20pt7b,
        );

        ili.set_rotation(1);

        let bitmap_width = 96;
        let bitmap_height = 96;
        let bitmap = include_bytes!("test_bitmap.bin");
        let mut toggle = true;
        loop {
            if toggle {
                ili.fill_screen(0);
                ili.draw_bitmap_hs(
                    bitmap_width,
                    bitmap_height,
                    4,
                    4,
                    ili9341::colors::GREEN,
                    ili9341::colors::BLACK,
                    bitmap,
                );
                ili.circle(60, 60, 24, ili9341::colors::RED);
                ili.disc(120, 60, 24, ili9341::colors::BLUE);
                ili.print(5, 35, "grn/blk");
            } else {
                ili.fill_screen(ili9341::colors::WHITE);
                ili.draw_bitmap_hs(
                    bitmap_width,
                    bitmap_height,
                    4,
                    4,
                    ili9341::colors::RED,
                    ili9341::colors::BLUE,
                    bitmap,
                );
                ili.circle(60, 60, 24, ili9341::colors::GREEN);
                ili.disc(120, 60, 24, ili9341::colors::BLACK);
                ili.print(5, 35, "red/blue");
            }
            toggle = !toggle;
            rn::rn_os_helper::delay_ms(1000);
        }
    }
}
/*
 * \fn rnLoop
 *
 *
 */
#[no_mangle]
pub extern "C" fn rnLoop() {
    let r: runTime = runTime::new();
    let boxed: Box<runTime> = Box::new(r);
    let mut boxed2: Box<runTime>;

    let ptr = Box::into_raw(boxed);
    unsafe {
        boxed2 = Box::from_raw(ptr);
    }
    boxed2.run();
}

#[no_mangle]
pub extern "C" fn rnInit() {
    lnLogger!("Setuping up LN9341 Demo...\n");
}
// EOF
