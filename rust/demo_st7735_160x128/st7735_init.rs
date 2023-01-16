
use ili9341::ili9341_cmds::{ILI9341_SOFTRESET,FAKE_DELAY_COMMAND,ILI9341_DISPLAYOFF, ILI9341_PIXELFORMAT,ILI9341_SLEEPOUT,ILI9341_DISPLAYON, ILI9341_INVERTOFF,ILI9341_MADCTL };

//------
pub const WW : u8 =  160;
pub const HH : u8 =  128;

 
pub const ST7735_SWRESET : u8 = 0x01;
pub const ST7735_SLPOUT  : u8 = 0x11;
pub const ST7735_NORON   : u8 = 0x13;
pub const ST7735_INVOFF  : u8 = 0x20;
pub const ST7735_INVON   : u8 = 0x21;
pub const ST7735_DISPON  : u8 = 0x29;
pub const ST7735_CASET   : u8 = 0x2A;
pub const ST7735_RASET   : u8 = 0x2B;
pub const ST7735_RAMWR   : u8 = 0x2C;
pub const ST7735_RAMRD   : u8 = 0x2E;
pub const ST7735_MADCTL  : u8 = 0x36;
pub const ST7735_COLMOD  : u8 = 0x3A;
pub const ST7735_INVCTR  : u8 = 0xB4;
pub const ST7735_DISSET5 : u8 = 0xB6;
pub const ST7735_FRMCTR1 : u8 = 0xB1;
pub const ST7735_FRMCTR2 : u8 = 0xB2;
pub const ST7735_FRMCTR3 : u8 = 0xB3;
pub const ST7735_PWCTR1  : u8 = 0xC0;
pub const ST7735_PWCTR2  : u8 = 0xC1;
pub const ST7735_PWCTR3  : u8 = 0xC2;
pub const ST7735_PWCTR4  : u8 = 0xC3;
pub const ST7735_PWCTR5  : u8 = 0xC4;
pub const ST7735_VMCTR1  : u8 = 0xC5;
pub const ST7735_GMCTRP1 : u8 = 0xE0;
pub const ST7735_GMCTRN1 : u8 = 0xE1;

pub const ST7735 : &[u8] = &[
	ST7735_INVOFF,		0,
	ST7735_FRMCTR1,		3,  0x05, 0x3a, 0x3a,
	ST7735_FRMCTR2,		3,  0x05, 0x3a, 0x3a, 
	ST7735_FRMCTR3, 	6,  0x05, 0x3a, 0x3a, 0x05, 0x3a, 0x3a,
	ST7735_INVCTR,   	1,	0x03,
	ST7735_PWCTR1,   	3,  0x62, 0x02, 0x04,
	ST7735_PWCTR2,   	1,  0xc0,
	ST7735_PWCTR3,   	2,	0x0d, 0x00,
	ST7735_PWCTR4,   	2,	0x8d, 0x6a,
	ST7735_PWCTR5,   	2,	0x8d, 0xee,
	ST7735_VMCTR1,   	1,	0x0e,
	ST7735_GMCTRP1,  	16, 0x10, 0x0e, 0x02, 0x03, 0x0e, 0x07, 0x02, 0x07, 0x0a, 0x12, 0x27, 0x37, 0x00, 0x0d, 0x0e, 0x10,
	ST7735_GMCTRN1,  	16, 0x10, 0x0e, 0x03, 0x03, 0x0f, 0x06, 0x02, 0x08, 0x0a, 0x13, 0x26, 0x36, 0x00, 0x0d, 0x0e, 0x10,
	ST7735_COLMOD  ,	1, 	0x05, // 0x3a: 0x6 -> 18 bits /pix, 0x5-> 15 bit/pixel /3 12 bits pixel
	//
	ST7735_CASET, 		4, 	0x00, 0, 0x00, WW-1, // Column addr set: XSTART=0, XEND=width
	ST7735_RASET, 		4, 	0x00, 0, 0x00, HH-1, // Row addr set: YSTART=0, YEND=height
	// 
	ST7735_MADCTL,   	1, 	0xA8, //  0x28 <-inverted->:   0x78: <-normal->, || 48 , || 18
	ST7735_DISPON,		0,   
	ST7735_SLPOUT,   	0,
	0

];
/*

	ILI9341_SOFTRESET,0,      //Soft Reset
	FAKE_DELAY_COMMAND,150,  // .kbv will power up with ONLY reset, sleep out, display on
	ILI9341_DISPLAYOFF,0,            //Display Off
	ILI9341_PIXELFORMAT, 1, 0x55,      //Pixel read=565, write=565.
    0 ];
*/
// EOF
