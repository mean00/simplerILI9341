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
#include "opensans20.h"
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
    lnSPISettings transaction(FAST*36*1000*1000+(1-FAST)*10*1000, SPI_MSBFIRST, SPI_MODE0,-1);
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
    ili->fillScreen(0xffff);   
#define FONT OpenSans_Regular28pt7b    
    ili->setFontFamily(&FONT,&FONT,&FONT) ;
    ili->setFontSize(ili9341::SmallFont);
    lnPinMode(LN_SYSTEM_LED,lnOUTPUT);
   }
/**
 * 
 */
#define T 80
#define K(x) 60*x

static uint16_t colors[4]=
{
    0,0x1f,0x3f<<5,0x1f<<11
};

#define FOO(x)  ili->square(colors[x],K(x),K(x),T,T);


void zsquare()
{    
    #define SQ(x) ili->square(colors[x],K(x),K(x),T,T);
    SQ(0);
    SQ(1);
    SQ(2);
    SQ(3);
   

    for(int x=20;x<120;x+=40)
    {
        ili->fillRoundRect( 120-x,120-x,2*x,2*x,5,0x1f,0xffff);
    }
     ili->setTextColor(0,0xffff);
    ili->print(36,80,"ABCD");
    ili->setTextColor(colors[1],colors[2]);
    ili->print(36,160,"1234");
     ili->setTextColor(colors[3],colors[0]);
    ili->print(36,120,"abc57");

}


int cur=0;
void foo()
{
    ili->fillScreen(colors[cur]);
    uint16_t next=(cur+1)&0x3;
    ili->square(colors[next],2,240-38,36,36);

    zsquare();
    cur=(cur+1)&3;
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
