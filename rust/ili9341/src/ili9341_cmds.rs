/*
 *  This is coming from adafruit driver
 */
pub const  ILI9341_SOFTRESET           : u8 = 0x01;
pub const  ILI9341_SLEEPIN             : u8 = 0x10;
pub const  ILI9341_SLEEPOUT            : u8 = 0x11;
pub const  ILI9341_NORMALDISP          : u8 = 0x13;
pub const  ILI9341_INVERTOFF           : u8 = 0x20;
pub const  ILI9341_INVERTON            : u8 = 0x21;
pub const  ILI9341_GAMMASET            : u8 = 0x26;
pub const  ILI9341_DISPLAYOFF          : u8 = 0x28;
pub const  ILI9341_DISPLAYON           : u8 = 0x29;
pub const  ILI9341_COLADDRSET          : u8 = 0x2A;
pub const  ILI9341_PAGEADDRSET         : u8 = 0x2B;
pub const  ILI9341_MEMORYWRITE         : u8 = 0x2C;
pub const  ILI9341_MEMCONTROL          : u8 = 0x36;
pub const  ILI9341_MADCTL			   : u8 = 0x36;
pub const  ILI9341_PIXELFORMAT         : u8 = 0x3A;
pub const  ILI9341_FRAMECONTROL        : u8 = 0xB1;
pub const  ILI9341_DISPLAYFUNC         : u8 = 0xB6;
pub const  ILI9341_ENTRYMODE           : u8 = 0xB7;
pub const  ILI9341_POWERCONTROL1       : u8 = 0xC0;
pub const  ILI9341_POWERCONTROL2       : u8 = 0xC1;
pub const  ILI9341_VCOMCONTROL1        : u8 = 0xC5;
pub const  ILI9341_VCOMCONTROL2        : u8 = 0xC7;

pub const  ILI9341_MADCTL_MY            : u8 = 0x80;
pub const  ILI9341_MADCTL_MX            : u8 = 0x40;
pub const  ILI9341_MADCTL_MV            : u8 = 0x20;
pub const  ILI9341_MADCTL_ML            : u8 = 0x10;
pub const  ILI9341_MADCTL_RGB           : u8 = 0x00;
pub const  ILI9341_MADCTL_BGR           : u8 = 0x08;
pub const  ILI9341_MADCTL_MH            : u8 = 0x04;
pub const  FAKE_DELAY_COMMAND           : u8 = 0xff;
// EOF


