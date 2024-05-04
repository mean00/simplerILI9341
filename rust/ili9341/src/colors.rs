pub const RED: u16 = 0x1f << 11;
pub const GREEN: u16 = 0x3f << 5;
pub const BLUE: u16 = 0x1f << 0;
pub const BLACK: u16 = 0;
pub const WHITE: u16 = 0xffff;

pub fn rgb(r: u8, g: u8, b: u8) -> u16 {
    return ((r as u16 >> 3) << 11) + ((g as u16 >> 2) << 5) + (b as u16 >> 3);
}
pub fn rgb32(rb: u32) -> u16 {
    rgb(
        (rb >> 16) as u8,
        ((rb >> 8) & 0xff) as u8,
        (rb & 0xff) as u8,
    )
}
