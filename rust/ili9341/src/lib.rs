#![no_std]
#![no_main]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![feature(stmt_expr_attributes)]
extern crate alloc;

pub mod access;
pub mod colors;
pub mod glyph;
pub mod ili9341;
pub mod ili9341_cmds;
mod util;
mod settings;

