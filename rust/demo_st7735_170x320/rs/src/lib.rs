#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(not(feature = "rp2040"))]
mod pinout_bp;
#[cfg(feature = "rp2040")]
mod pinout_rp2040;

mod app;
mod st7735_init;
mod testfont;
// EOF
