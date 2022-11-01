
// borrowed from adafruit
//use ili9341::ili9341_cmds::{ILI9341_SOFTRESET};
use crate::ili9341_cmds::{ILI9341_SOFTRESET,FAKE_DELAY_COMMAND,ILI9341_DISPLAYOFF, ILI9341_PIXELFORMAT,ILI9341_SLEEPOUT,ILI9341_DISPLAYON, ILI9341_INVERTOFF,ILI9341_MADCTL };
/*
 *
#define ILI_WAIT(x) FAKE_DELAY_COMMAND, x
#define ILI_CMD(x) x,0
#define ILI_CMD2(x,y) x,1,y

 */

//------
pub const DSO_RESET : &[u8] = &[
	ILI9341_SOFTRESET,0,      //Soft Reset
	FAKE_DELAY_COMMAND,150,  // .kbv will power up with ONLY reset, sleep out, display on
	ILI9341_DISPLAYOFF,0,            //Display Off
	ILI9341_PIXELFORMAT, 1, 0x55,      //Pixel read=565, write=565.
    0 ];
//-----    
pub const DSO_WAKEUP : &[u8] = &[
	ILI9341_SLEEPOUT,0,            //Sleep Out
	FAKE_DELAY_COMMAND, 150,
	ILI9341_DISPLAYON,0 ,           //Display On
	//additional settings
	ILI9341_INVERTOFF,0,			// invert off
	ILI9341_MADCTL, 1, 0x48 ,     //Memory Access
	0xB0, 1, 0x40 ,    //RGB Signal [40] RCM=2
    0
] ;

// EOF
