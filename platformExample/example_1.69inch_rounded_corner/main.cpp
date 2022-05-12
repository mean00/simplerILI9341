/**
 *  Demo code for ST7789 1.69 inch rounded corner LCD screen
 * /!\ This is one is from ali express and is a 3 wire SPI interface, MOSI is used for both RX and TX
 * Hence the call to enable3WireMode
 * /!\ If you have a 4 wires one, like the adafruit product, dont call enable3wireMode !
 * 
 */
#include "lnArduino.h"
#include "ili_lnSpi.h"
#include "simpler9341_priv.h"

lnSpi9341          *ili;
hwlnSPIClass *spi;

#define WW  280
#define HH  240


extern const uint8_t st7789v2_ada[] __attribute__((used))= {
    
    ILI_CMD(ILI9341_SOFTRESET),
    ILI_WAIT(150),

    ILI_CMD(ILI9341_SLEEPOUT),
    ILI_WAIT( 10),
    ILI_CMD2(ILI9341_PIXELFORMAT, 0x55),    // 16 bit mode
    ILI_WAIT( 10),
    ILI_CMD2(ILI9341_MADCTL, 0x08),
    ILI9341_COLADDRSET, 4, 0x00, 0x00, 240>>8, 240&0xff,
    ILI9341_PAGEADDRSET, 4, 0x00, 0x00, 320>>8, 320&0xff, // 2B
    ILI_CMD(ILI9341_INVERTON),
    ILI_WAIT( 10),
    ILI_CMD(ILI9341_NORMALDISP),
    ILI_WAIT( 10),
    ILI_CMD(ILI9341_DISPLAYON),
    ILI_WAIT( 10),
    0
};

/**
 * 
 */
void setup()
{
    Logger("Setuping up DSO...\n");
    
     // arbitrer must be created with screen already set up
    // ili must be first
    //
    spi=new hwlnSPIClass(0,-1);    
    spi->begin();
    lnSPISettings transaction(100*1000, SPI_MSBFIRST, SPI_MODE0,-1);
    spi->beginTransaction(transaction);
    
    ili=new lnSpi9341( 240, 280,
                                    spi,        
                                    PB7,       // DC/RS
                                    PB8,       // CS 
                                    PB6);  // Reset
    ili->enable3WireMode();
    ili->init(st7789v2_ada,NULL); 
    ili->setRotation(1);
    ili->fillScreen(0x0);    
    lnPinMode(LN_SYSTEM_LED,lnOUTPUT);
   }
/**
 * 
 */
void loop()
{
    
    while(1)
    {
        lnDigitalWrite(LN_SYSTEM_LED,0);
        xDelay(100);
        ili->fillScreen(0x00);
        lnDigitalWrite(LN_SYSTEM_LED,1);
        xDelay(100);        
        ili->fillScreen(0x1f);
        Logger("*\n");
        xDelay(100);
        ili->fillScreen(0xffff);
    }
}
//
