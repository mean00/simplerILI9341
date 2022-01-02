
#include "lnArduino.h"
#include "simpler9341.h"
#include "simpler9341_priv.h"

// borrowed from adafruit

extern const uint8_t dso_resetOff[] __attribute__((used))= {
	0x01, 0,            //Soft Reset
	FAKE_DELAY_COMMAND, 150,  // .kbv will power up with ONLY reset, sleep out, display on
	0x28, 0,            //Display Off
	0x3A, 1, 0x55,      //Pixel read=565, write=565.
    0
} ;
extern const uint8_t dso_wakeOn[] __attribute__((used))= {
	0x11, 0,            //Sleep Out
	FAKE_DELAY_COMMAND, 150,
	0x29, 0,            //Display On
	//additional settings
	ILI9341_INVERTOFF, 0,			// invert off
	0x36, 1, 0x48,      //Memory Access
	0xB0, 1, 0x40,      //RGB Signal [40] RCM=2
    0
} ;

// EOF