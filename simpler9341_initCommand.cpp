
#include "lnArduino.h"
#include "simpler9341.h"
#include "simpler9341_priv.h"

// borrowed from adafruit


extern const uint8_t dso_resetOff[] __attribute__((used))= {
	ILI_CMD(ILI9341_SOFTRESET),            //Soft Reset
	ILI_WAIT(150),  // .kbv will power up with ONLY reset, sleep out, display on
	ILI_CMD(ILI9341_DISPLAYOFF),            //Display Off
	ILI_CMD2(ILI9341_PIXELFORMAT, 0x55),      //Pixel read=565, write=565.
    0
} ;
extern const uint8_t dso_wakeOn[] __attribute__((used))= {
	ILI_CMD(ILI9341_SLEEPOUT),            //Sleep Out
	ILI_WAIT(150),
	ILI_CMD(ILI9341_DISPLAYON),            //Display On
	//additional settings
	ILI_CMD(ILI9341_INVERTOFF),			// invert off
	ILI_CMD2(ILI9341_MADCTL, 0x48),      //Memory Access
	ILI_CMD2(0xB0, 0x40),      //RGB Signal [40] RCM=2
    0
} ;

// EOF