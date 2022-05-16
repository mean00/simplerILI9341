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
#define FAST 1
    spi=new hwlnSPIClass(0,-1);    
    lnSPISettings transaction(FAST*36*1000*1000+(1-FAST)*200*1000, SPI_MSBFIRST, SPI_MODE0,-1);
    spi->begin();
    
    
    ili=new lnSpi9341( 240, 280,
                                    spi,        
                                    PB7,       // DC/RS
                                    PB8,       // CS 
                                    PB6);  // Reset
    spi->beginTransaction(transaction);
    ili->enable3WireMode();
    ili->init(st7789v2_ada,NULL); 
    ili->forceChipId(0x7789);
    ili->setRotation(3);
    ili->fillScreen(WHITE);   
    ili->fillScreen(BLACK);   
    lnPinMode(LN_SYSTEM_LED,lnOUTPUT);
   }
/**
 * 
 */
#define T 80
#define K(x) 60*x

static uint16_t colors[4]=
{
    0,BLUE,GREEN,RED
};

#define FOO(x)  ili->square(colors[x],K(x),K(x),T,T);

#define LOCK() while(1)         __asm__("nop");


int cur=0;
void foo()
{
    #if 0
    for(int x=20;x<240;x+=10)
    {
        ili->drawLine(0,0,x,239,WHITE);
        ili->drawLine(239,0,239-x,239,BLUE);
        ili->drawLine(0,0,239,x,RED);
        ili->drawLine(239-x,0,239,x,GREEN);
    }
    //LOCK();
    ili->disc(GREEN,60,120,40);
    ili->disc(BLUE,30,30,10);
    for(int i=20;i<200;i+=40)
        ili->disc(RED,i,120,10);
    #endif
    ili->HLine(40,119,200,BLUE);
    ili->HLine(40,120+40,200,BLUE);
    for(int i=0;i<4;i++)
        ili->invertedDiscCorner(PINK, 40+i*40,120,40, (1<<i));
    for(int i=0;i<4;i++)
        ili->VLine( 40+i*40,0,240, RED);
}

void loop()
{

    while(1)
    {
        lnDigitalToggle(LN_SYSTEM_LED);
        foo();
    }
}
//
