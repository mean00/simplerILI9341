/*
 *  (C) Adafruit
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 * 
 * This is a 8 bit parrallel mode for the ILI9341/ST7789 as present on the DSO150
 * With the GD32vF103 there is a timing issue so it is slowed down a bit
 * Not sure if the timing issue is due to the speed (running at 108 Mhz with max
 * GPIO speed = 50 Mhz + speed of ILI ignals) or if it is due to fence issue
 * 
 * 
 * 
 */
#include "gd32_spi.h"
#include "simpler9341_priv.h"


#define IS_7789()  (_chipId== 0x7789)
#define CHECK_ARBITER() 
#define FAKE_DELAY_COMMAND 0xff
#define LOW_LEVEL_PRINT(...) {}

#define colorMap(x) x


/**
 * 
 * @param w
 * @param h
 * @param spi
 * @param pinDc
 * @param pinCS
 */
lnSpi9341::lnSpi9341( int w, int h ,hwlnSPIClass *spi, int pinDC,int pinCS, int pinReset)   : ili9341(w,h)
{    
    _spi=spi;    
    _pinReset=pinReset;
    _pinDC=pinDC;
    _pinCS=pinCS;
    _PhysicalXoffset=0;
    _PhysicalYoffset=0;
}
/**
 * 
 */
lnSpi9341::~lnSpi9341()
{

}

// borrowed from adafruit

static const uint8_t resetOff[] = {
	0x01, 0,            //Soft Reset
	FAKE_DELAY_COMMAND, 150,  // .kbv will power up with ONLY reset, sleep out, display on
	0x28, 0,            //Display Off
	0x3A, 1, 0x55,      //Pixel read=565, write=565.
};
static const uint8_t wakeOn[] = {
	0x11, 0,            //Sleep Out
	FAKE_DELAY_COMMAND, 150,
	0x29, 0,            //Display On
	//additional settings
	ILI9341_INVERTOFF, 0,			// invert off
	0x36, 1, 0x48,      //Memory Access
	0xB0, 1, 0x40,      //RGB Signal [40] RCM=2
};

#define CS_ACTIVE   {if(_pinCS!=-1) lnDigitalWrite(_pinCS,false);}
#define CS_IDLE     {if(_pinCS!=-1) lnDigitalWrite(_pinCS,true);}

#define CD_DATA     {lnDigitalWrite(_pinDC,true);}
#define CD_COMMAND  {lnDigitalWrite(_pinDC,false);}


#define CS_ACTIVE_CD_COMMAND {CS_ACTIVE;CD_COMMAND}




/*****************************************************************************/

void lnSpi9341::write8(uint8_t c)
{
    _spi->write(c);
}
/**
 * This is not used much at all. Only a few times at the beginning
 * No need to optimize
 * @return 
 */
uint8_t  lnSpi9341::read8()
{  
    //return _spi->read
    return 0;
  
}

/**
 * 
 * @param cmd
 * @param payload
 * @param data
 */
void lnSpi9341::writeCmdParam(uint16_t cmd, int payload, const uint8_t * data)
{
    CD_COMMAND;
    //sendWord(cmd);
    _spi->write16(cmd);
    if(payload)
    {
        CD_DATA;
        _spi->dmaWrite(payload,data);
    }    
}
/**
 * 
 * @param c
 */
void lnSpi9341::writeCommand(uint16_t c)
{
    writeCmdParam(c,0,NULL);
}


/**
 * This is not used much
 * @param reg
 * @return 
 */
uint32_t lnSpi9341::readRegister32(int r)
{
  uint32_t val;
  uint8_t x;

  CS_ACTIVE;
  
  // try reading register #4
  writeCommand(r);
  CD_DATA;
  lnDelayUs(50);
  val = read8();          
  val <<= 8;              
  val  |= read8();        
  val <<= 8;              
  val  |= read8();;       
  val <<= 8;              
  val  |= read8();;       
  CS_IDLE;
  
  return val;
}

/**
 * 
 * @param reg
 * @return 
 */
void lnSpi9341::writeRegister32(int r,uint32_t  val)
{    
  uint8_t flat[4]={(uint8_t)(val>>24),(uint8_t)((val>>16)&0xff),(uint8_t)((val>>8)&0xff),(uint8_t)(val&0xff)};
  writeCmdParam(r,4,flat);
}

/**
 * 
 */
uint32_t lnSpi9341::readChipId()
{  
    return 0x9341;
  uint32_t regD3=readRegister32(0xd3)&0xffff ;
  uint32_t reg04=readRegister32(0x04)&0xffff ;
  
  if(regD3==0x9341) return 0x9341; // 9341 
  if(reg04==0x8552) return 0x7789; // is it really a 7789 ?
  return 0; // unknown
}

/**
 * 
 */
void lnSpi9341::reset()
{
    lnPinMode(_pinDC,lnOUTPUT);
    lnDigitalWrite(_pinDC,true);
    if(_pinCS!=-1)
    {
        lnPinMode(_pinCS,lnOUTPUT);
        lnDigitalWrite(_pinCS,true);
    }
    
    
    CS_IDLE; 
    CD_DATA; 	
    

    if(_pinReset!=-1)
    {            
        lnPinMode(_pinReset,lnOUTPUT);    
        lnDigitalWrite(_pinReset,HIGH);
        xDelay(50);
        lnDigitalWrite(_pinReset,LOW);
        xDelay(50);
        lnDigitalWrite(_pinReset,HIGH);	
        xDelay(50);
    }
    _chipId=readChipId();
}
/**
 * 
 * @param size
 * @param data
 */
void lnSpi9341::sendSequence(int size, const uint8_t *data)
{
        const uint8_t *tail=data+size;
        
        
	while (data < tail) 
        {
            CS_ACTIVE;
            uint8_t cmd = data[0];
            uint8_t len = data[1];
            data+=2;
            if (cmd == FAKE_DELAY_COMMAND) 
            {			
                delay(len);
                continue;
            }                 
            writeCmdParam(cmd, len, data);
            data += len;		
            CS_IDLE;
	}
        
}

/**
 * 
 */
void lnSpi9341::init()
{   
  
  reset();
  baseInit();
  sendSequence(sizeof(resetOff),resetOff);  
  sendSequence(sizeof(wakeOn),wakeOn);  
}
/**
 * 
 * @param byte
 */
void lnSpi9341::sendByte(int c)
{
    write8(c);
}

static const uint8_t rotMode[4]={0x8,0xc8,0x78,0xa8};
/**
 * 
 */
void lnSpi9341::updateHwRotation(void)
{
    uint8_t t;
    switch(_chipId)
    {
        case 0x9341:       
            switch(_rotation) 
            {
                case 1:        t = ILI9341_MADCTL_MX | ILI9341_MADCTL_MY | ILI9341_MADCTL_MV ;          break;
                case 2:        t = ILI9341_MADCTL_MX ;                          break;
                case 3:        t = ILI9341_MADCTL_MV ;                          break;
                case 0:        default:t = ILI9341_MADCTL_MY ;                  break;
           }
           break;
        case 0x7789:       
            switch(_rotation) 
            {
                case 1:        t = ILI9341_MADCTL_MY | ILI9341_MADCTL_MV ;          break;
                case 2:        t = 0 ;                                              break;
                case 3:        t = ILI9341_MADCTL_MX | ILI9341_MADCTL_MV ;          break;
                case 0:        default: t = ILI9341_MADCTL_MX | ILI9341_MADCTL_MY ; break;
           }
           break;
        default:
            xAssert(0);
            break;
    }
    t|= ILI9341_MADCTL_RGB;
    CS_ACTIVE;
    writeCmdParam(ILI9341_MADCTL,1,&t);
    CS_IDLE;   
}


/**
 * 
 * @param x
 * @param y
 * @param w
 * @param h
 */
void lnSpi9341::setAddress(int x, int y, int w, int h)
{
    int a1,a2,b1,b2;
    a1=x+_xOffset;
    a2=a1+w-1;
    b1=y+_yOffset;
    b2=b1+h-1;
    CS_ACTIVE;    
    writeRegister32(ILI9341_COLADDRSET,  ((uint32_t)(a1<<16) | a2));  // HX8357D uses same registers!
    writeRegister32(ILI9341_PAGEADDRSET, ((uint32_t)(b1<<16) | b2)); // HX8357D uses same registers!
    CS_IDLE;
}
/**
 * 
 * @param x0
 * @param y0
 * @param h
 * @param color
 */
void lnSpi9341::VLine(int x0, int y0, int h, int color)
{
  setAddress(x0, y0, 1, h);
  floodWords(h,color); 
}
/**
 * 
 * @param x0
 * @param y0
 * @param w
 * @param color
 */
void lnSpi9341::HLine(int x0, int y0, int w, int color)
{
  setAddress(x0, y0, w, 1);
  floodWords(w,color); 
}

/**
 * 
 * @param byte
 */
void lnSpi9341::sendWord(int xbyte)
{
    _spi->write16(xbyte);
}
/**
 * 
 * @param nb
 * @param data
 */
void lnSpi9341::sendBytes(int nb, const uint8_t *data)
{
    xAssert(0);
}
/**
 * 
 * @param nb
 * @param data
 */
void lnSpi9341::sendWords(int nb, const uint16_t *data)
{
    _spi->dmaWrite16(nb,data);    
}

/**
 * 
 */
void lnSpi9341::dataBegin()
{
    CS_ACTIVE;
    CD_COMMAND;
    sendWord(ILI9341_MEMORYWRITE);
    CD_DATA;
}
/**
 * 
 */
void lnSpi9341::dataEnd()
{
     CD_COMMAND;
     CS_IDLE;
}
/**
* 
* @param nb
* @param data
 * 
 * GD32M4 : Simple = 33 ms, same = 28 ms
 * Quad16            26 ms  same = 21 ms
 * 
 * pulsesLow Simple  = 26 ms , same = 8 ms
 * 
*/
void lnSpi9341::floodWords(int nb, const uint16_t data)
{      
    uint16_t f=colorMap(data);
    // except for fill screen, it fits in one dma request
    // no need to optimize much
    while(nb)
    {
        CS_ACTIVE;
        int chunk=nb;
        if(chunk>65535) chunk=65535;
        nb-=chunk;
        CD_COMMAND;
        sendWord(ILI9341_MEMORYWRITE);
        CD_DATA;
        _spi->dmaWrite16Repeat(chunk,data);
        CS_IDLE;        
    }
    
}
/**
 * 
 * @param data
 * @param len
 * @param first
 * @param fg
 * @param bg
 */
 void lnSpi9341::push2Colors(uint8_t *data, int len, int fg, int bg)
 {  
#warning FIXME: Max line hardcoded to 320
    uint16_t colors[320];
    uint16_t *p=colors;
    xAssert(len<320);
    uint8_t *tail=len+data;
    while( data<tail)
    {
        if(*(data++))
        {
            *p=fg;
        } else
        {
            *p=bg;
        }
        p++;
    }
    _spi->dmaWrite16(len,colors);
}



// EOF
