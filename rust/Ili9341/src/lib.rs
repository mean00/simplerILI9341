#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(stmt_expr_attributes)]
extern crate alloc;

pub mod access;
pub mod colors;
pub mod glyph;
pub mod simpler9341;
mod util;

#[cfg(feature = "hs")]
use heatshrink_byte as hs;