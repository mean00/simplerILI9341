#![no_std]
#![no_main]
#![allow(dead_code)]
#![feature(stmt_expr_attributes)]
extern crate alloc;

pub mod access;
pub mod colors;
pub mod ili9341;
pub mod ili9341_cmds;
mod util;
mod settings;
pub mod ili9341_init_sequence;

