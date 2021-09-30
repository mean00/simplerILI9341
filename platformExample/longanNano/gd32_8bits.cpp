/*
 *  (C) Adafruit
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 * 
 * This is a 8 bit parrallel mode for the DSO150
 * 
 * 
 */
#include "gd32_8bits.h"
#include "simpler9341_priv.h"

#define FAKE_DELAY_COMMAND 0xff

/**
 * 
 * @param w
 * @param h
 * @param spi
 * @param pinDc
 * @param pinCS
 */
ln8bit9341::ln8bit9341( int w, int h ,int port, int pinDC,int pinCS, int pinWrite, int pinRead, int pinReset)   : ili9341(w,h),_ioRead(pinRead),_ioCS(pinCS),_ioDC(pinDC)
{    
    _dataPort=port;    
    _pinReset=pinReset;
    _PhysicalXoffset=0;
    _PhysicalYoffset=0;
    _bop=(uint32_t *)lnGetGpioToggleRegister(_dataPort);
    _ioWrite=new lnFast8bitIo(_bop,pinWrite);
}
/**
 * 
 */
ln8bit9341::~ln8bit9341()
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

#define CS_ACTIVE   {_ioCS.off();}
#define CS_IDLE     {_ioCS.on();}

#define CD_DATA     {_ioDC.on();}
#define CD_COMMAND  {_ioDC.off();}

#define WR_ACTIVE   {_ioWrite->off();}
#define WR_IDLE     {_ioWrite->on();}

#define RD_ACTIVE   {_ioRead.off();}
#define RD_IDLE     {_ioRead.on();}

#define WR_STROBE {_ioWrite->pulseLow();}
#define CS_ACTIVE_CD_COMMAND {CS_ACTIVE;CD_COMMAND}




/*****************************************************************************/

void ln8bit9341::write8(uint8_t c)
{
    int cc=(int )c;
    *_bop=  (((cc^0xFF)<<16) | (cc));
    WR_STROBE;
}
/**
 * 
 * @return 
 */
uint8_t  ln8bit9341::read8()
{  
  RD_ACTIVE;
  lnDelayUs(10);
  uint8_t temp = lnReadPort(_dataPort) &0xff;
  lnDelayUs(10);
  RD_IDLE;
  lnDelayUs(10);
  return temp;
}

/**
 * 
 * @param cmd
 * @param payload
 * @param data
 */
void ln8bit9341::writeCmdParam(uint16_t cmd, int payload, const uint8_t * data)
{
    CD_COMMAND;
    sendWord(cmd);
    if(payload)
    {
        CD_DATA;
        const uint8_t *tail=data+payload;
	while(data<tail) 
        {
            write8(*(data++));
	}        
    }    
}
/**
 * 
 * @param c
 */
void ln8bit9341::writeCommand(uint16_t c)
{
    writeCmdParam(c,0,NULL);
}
//------------------------------
// This is *very* seldom used....
//-------------------------------
void ln8bit9341::setReadDir()
{
    for(int i=0;i<8;i++)
        lnPinMode(PB0+i,lnINPUT_PULLDOWN);
}

void ln8bit9341::setWriteDir()
{
    for(int i=0;i<8;i++)
        lnPinMode(PB0+i,lnOUTPUT);
}


/**
 * This is not used much
 * @param reg
 * @return 
 */
uint32_t ln8bit9341::readRegister32(int r)
{
  uint32_t val;
  uint8_t x;

  CS_ACTIVE;
  setWriteDir();    
  // try reading register #4
  writeCommand(r);
  setReadDir();  // Set up LCD data port(s) for READ operations
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
  setWriteDir();    
  return val;
}

/**
 * 
 * @param reg
 * @return 
 */
void ln8bit9341::writeRegister32(int r,uint32_t  val)
{    
  uint8_t flat[4]={(uint8_t)(val>>24),(uint8_t)((val>>16)&0xff),(uint8_t)((val>>8)&0xff),(uint8_t)(val&0xff)};
  writeCmdParam(r,4,flat);
}

/**
 * 
 */
uint32_t ln8bit9341::readChipId()
{  
  uint32_t regD3=readRegister32(0xd3)&0xffff ;
  uint32_t reg04=readRegister32(0x04)&0xffff ;
  
  if(regD3==0x9341) return 0x9341; // 9341 
  if(reg04==0x8552) return 0x7789; // is it really a 7789 ?
  return 0; // unknown
}

/**
 * 
 */
void ln8bit9341::reset()
{
    _ioWrite->on();
    _ioRead.on();
    _ioCS.on();
    _ioDC.on();

    CS_IDLE; 
    WR_IDLE;
    RD_IDLE;
    CD_DATA; 	
    setWriteDir();

    if(_pinReset!=-1)
    {            
        lnPinMode(_pinReset,lnOUTPUT);    
        lnDigitalWrite(_pinReset,HIGH);
        xDelay(50);
        lnDigitalWrite(_pinReset,LOW);
        xDelay(50);
        lnDigitalWrite(_pinReset,HIGH);	
    }
}
/**
 * 
 * @param size
 * @param data
 */
void ln8bit9341::sendSequence(int size, const uint8_t *data)
{
        const uint8_t *tail=data+size;
        CS_ACTIVE;
        
	while (data < tail) 
        {
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
	}
        CS_IDLE;
}

/**
 * 
 */
void ln8bit9341::init()
{   
  setWriteDir();
  reset();
  baseInit();
  sendSequence(sizeof(resetOff),resetOff);
  sendSequence(sizeof(wakeOn),wakeOn);  
}
/**
 * 
 * @param byte
 */
void ln8bit9341::sendByte(int c)
{
    write8(c);
}
/**
 * 
 * @param byte
 */
void ln8bit9341::sendWord(int xbyte)
{
    int cc=(int )xbyte;
    int cl=cc&0xff;
    int ch=cc>>8;
    *_bop=  (((ch^0xFF)<<16) | (ch));
    WR_STROBE;
    *_bop=  (((cl^0xFF)<<16) | (cl));
    WR_STROBE;        
}
/**
 * 
 * @param nb
 * @param data
 */
void ln8bit9341::sendBytes(int nb, const uint8_t *data)
{
    const uint8_t *tail=data+nb;
    while(data<tail)
    {
        int cc=(int )*data++;
        *_bop=  (((cc^0xFF)<<16) | (cc));
        WR_STROBE;
    }
}
/**
 * 
 * @param nb
 * @param data
 */
void ln8bit9341::sendWords(int nb, const uint16_t *data)
{
    const uint16_t *tail=data+nb;
    while(data<tail)
    {
        int cc=(int )*data++;
        int cl=cc&0xff;
        int ch=cc>>8;
        *_bop=  (((ch^0xFF)<<16) | (ch));
        WR_STROBE;
        *_bop=  (((cl^0xFF)<<16) | (cl));
        WR_STROBE;        
    }
    
}
/**
 * 
 * @param nb
 * @param data
 */
void ln8bit9341::floodSameWords(int nb, const uint8_t data)
{      
    register int cl=data;    
    cl= (((cl^0xFF)<<16) | (cl));    
    CS_ACTIVE;
    CD_COMMAND;
    sendWord(ILI9341_MEMORYWRITE);
    CD_DATA;
    *_bop=  cl;
    _ioWrite->pulsesLow(nb);
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
void ln8bit9341::floodWords(int nb, const uint16_t data)
{      
    
    if((data&0xff)==(data>>8)) return floodSameWords(nb,data&0xff);
    
    register int cl=data&0xff;
    register int ch=data>>8;
    
    cl= (((cl^0xFF)<<16) | (cl));
    ch= (((ch^0xFF)<<16) | (ch));
    CS_ACTIVE;
    CD_COMMAND;
    sendWord(ILI9341_MEMORYWRITE);
    CD_DATA;
    _ioWrite->pulseData(nb,ch,cl);
    CS_IDLE;
}
/**
 * 
 * @param data
 * @param len
 * @param first
 * @param fg
 * @param bg
 */
 void ln8bit9341::push2Colors(uint8_t *data, int len, bool first,int fg, int bg)
 {
    CS_ACTIVE;
    if(first)
    {
        CD_COMMAND;
        sendWord(ILI9341_MEMORYWRITE);
    }
    CD_DATA;
    _ioWrite->push2Colors(len,data,fg,bg);
    CS_IDLE;
 } 
static const uint8_t rotMode[4]={0x8,0xc8,0x78,0xa8};
/**
 * 
 */
void ln8bit9341::updateHwRotation(void)
{
    uint8_t t;
    switch(_rotation) 
    {
        case 1:
          t = ILI9341_MADCTL_MX | ILI9341_MADCTL_MY | ILI9341_MADCTL_MV | ILI9341_MADCTL_RGB;
          break;
        case 2:
          t = ILI9341_MADCTL_MX | ILI9341_MADCTL_RGB;
          break;
        case 3:
          t = ILI9341_MADCTL_MV | ILI9341_MADCTL_RGB;
          break;
        case 0:
        default:
         t = ILI9341_MADCTL_MY | ILI9341_MADCTL_RGB;
         break;
   }
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
void ln8bit9341::setAddress(int x, int y, int w, int h)
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
 * @param count
 */
void lnFast8bitIo::pulsesLow(int count)
{
    register uint32_t on=_onbit,off=_offbit;
    register volatile uint32_t *a=_onoff;
    
    int block=count>>4;
    int leftover=count&15;

#define DO4 *a=off; *a=on;*a=off; *a=on;*a=off; *a=on;*a=off; *a=on;*a=off; *a=on;*a=off; *a=on;*a=off; *a=on;*a=off; *a=on;
    
    for(int i=0;i<block;i++)
    {
        DO4
        DO4
        DO4
        DO4
    }
    
    for(int i=0;i<leftover;i++)
    {
        *a=off;
        *a=on;
        *a=off;
        *a=on;
    }    
}   
/**
 * 28 ms => 13 ms
 * \brief This is a high speed data sending of the same 16 byte pattern
 * @param count
 * @param hi
 * @param lo
 */
void lnFast8bitIo::pulseData(int nb, int hi, int lo)
{
    
#define WRP    *onoff=down;*onoff=up;
    
    register uint32_t hh=hi,ll=lo,up=_onbit,down=_offbit;
    volatile register uint32_t *bop=_bop,*onoff=_onoff;
    
    
        int block=nb>>4;
        int leftover=nb&15;
        for(int i=0;i<block;i++)
        {
#define SINGD            *bop=  hh;      WRP;         *bop=  ll;            WRP;        
#define QUADD            SINGD;SINGD;SINGD;SINGD
             QUADD;
             QUADD;
             QUADD;
             QUADD;
             
        }
        for(int i=0;i<leftover;i++)
        {        
            SINGD
        }
}
/**
 * 
 * @param len
 * @param data
 * @param fg
 * @param bg
 */
void   lnFast8bitIo::push2Colors(int len, uint8_t *data, int fg, int bg)
{
    uint32_t fhi=fg>>8;
    uint32_t flo=fg&0xff;
    uint32_t bhi=bg>>8;
    uint32_t blo=bg&0xff;
    volatile register uint32_t *bop=_bop,*onoff=_onoff;
    register uint32_t up=_onbit,down=_offbit;
#define EXP(x) x=  (((x^0xFF)<<16) | (x));
    EXP(fhi);
    EXP(flo);
    EXP(bhi);
    EXP(blo);
    
    uint8_t *tail=len+data;
    while( data<tail)
    {
        if(*(data++))
        {
              *bop=  fhi;      WRP;         *bop=  flo;            WRP;        
              continue;
        } else
        {
              *bop=  bhi;      WRP;         *bop=  blo;            WRP;    
        }
    }
}
// EOF