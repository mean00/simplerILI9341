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
#define LOW_LEVEL_PRINT(...) {}
/*
 * Replace PB3 by PB8 and output BOP word
 * STM32 direct 108M: Draw 30 chars 25 ms
 * Risc32 PB8   108M: Draw 30 chars "This is a hello world program 123"   26 ms
 * /!\ No need to optimize this further, it has little effect
 * 
 */
int swapper(int val)
{
    int extra=0;
    if(val & (1<<3))
    {
        extra=(1<<8);
        val&=~(1<<3);
    }
    else
    {
        extra=(1<<(8+16));
    }
    
    uint32_t r= ((((val^0xF7)<<16) | (val)) );    
    r|=extra;    
    return r;
}
#ifdef USE_PB8_INSTEAD_OF_PB3
    #define WR_DATA8(x) swapper(x)
#else
    #define WR_DATA8(cc)  ((((cc^0xFF)<<16) | (cc)) )
#endif

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
    int cc=WR_DATA8((int )c);    
    *_bop= cc;
    WR_STROBE;
}
/**
 * This is not used much at all. Only a few times at the beginning
 * No need to optimize
 * @return 
 */
uint8_t  ln8bit9341::read8()
{  
  RD_ACTIVE;
  
  lnDelayUs(10);  
#ifdef USE_PB8_INSTEAD_OF_PB3  
  uint16_t temp = lnReadPort(_dataPort);
  temp&= 0x1ff;
  if(temp & (1<<8)) temp|=(1<<3);
  else temp&=~(1<<3);
  temp&=0xff;
#else
  uint8_t temp = lnReadPort(_dataPort) &0xff;
#endif  
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
    
        lnPinMode(PB0,lnINPUT_PULLDOWN);
        lnPinMode(PB1,lnINPUT_PULLDOWN);
        lnPinMode(PB2,lnINPUT_PULLDOWN);
        lnPinMode(PB4,lnINPUT_PULLDOWN);
        lnPinMode(PB5,lnINPUT_PULLDOWN);
        lnPinMode(PB6,lnINPUT_PULLDOWN);
        lnPinMode(PB7,lnINPUT_PULLDOWN);
#ifdef USE_PB8_INSTEAD_OF_PB3        
        lnPinMode(PB8,lnINPUT_PULLDOWN);
#else
        lnPinMode(PB3,lnINPUT_PULLDOWN);
#endif
}

void ln8bit9341::setWriteDir()
{
        lnPinMode(PB0,lnOUTPUT);
        lnPinMode(PB1,lnOUTPUT);
        lnPinMode(PB2,lnOUTPUT);
        lnPinMode(PB4,lnOUTPUT);
        lnPinMode(PB5,lnOUTPUT);
        lnPinMode(PB6,lnOUTPUT);
        lnPinMode(PB7,lnOUTPUT);
#ifdef USE_PB8_INSTEAD_OF_PB3                
        lnPinMode(PB8,lnOUTPUT);
#else
        lnPinMode(PB3,lnOUTPUT);
#endif
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
        xDelay(50);
    }
    _chipId=readChipId();
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
    *_bop= WR_DATA8(ch );
    WR_STROBE;
    *_bop= WR_DATA8(cl );
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
        *_bop=  WR_DATA8(cc) ;
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
        int cl=WR_DATA8(cc&0xff);
        int ch=WR_DATA8(cc>>8);
        *_bop=  (ch);
        WR_STROBE;
        *_bop=  (cl);
        WR_STROBE;        
    }
    
}

/**
 * 
 */
void ln8bit9341::dataBegin()
{
    CS_ACTIVE;
    CD_COMMAND;
    sendWord(ILI9341_MEMORYWRITE);
    CD_DATA;
}
/**
 * 
 */
void ln8bit9341::dataEnd()
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
void ln8bit9341::floodWords(int nb, const uint16_t data)
{      
    
    register int cl=data&0xff;
    register int ch=data>>8;
    
    CS_ACTIVE;
    CD_COMMAND;
    sendWord(ILI9341_MEMORYWRITE);
    CD_DATA;
    if(1 && (data&0xff)==(data>>8))   // this does not work ?
    {
        uint32_t cl= WR_DATA8(cl&0xff);
        *_bop= ( cl );
        _ioWrite->pulsesLow(nb);
        CS_IDLE;
        return;
    }   
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
 void ln8bit9341::push2Colors(uint8_t *data, int len, int fg, int bg)
 {  
    _ioWrite->push2Colors(len,data,fg,bg);
 } 
static const uint8_t rotMode[4]={0x8,0xc8,0x78,0xa8};
/**
 * 
 */
void ln8bit9341::updateHwRotation(void)
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

//--- the time critical part follows
//--- If you use another implementation, this where the speed is lost

/**
 * 28 ms => 13 ms
 * \brief This is a high speed data sending of the same 16 byte pattern
 * @param count
 * @param hi
 * @param lo
 */
void lnFast8bitIo::pulseData(int nb, int  hi, int lo)
{
    
#define WRP    *onoff=down;*onoff=up;
    
    register uint32_t hh=WR_DATA8(hi),ll=WR_DATA8(lo),up=_onbit,down=_offbit;
    volatile register uint32_t *bop=_bop,*onoff=_onoff;
    
    
        int block=nb>>4;
        int leftover=nb&15;
        for(int i=0;i<block;i++)
        {
#define SINGD            *bop=  (hh);      WRP;         *bop=  (ll);            WRP;        
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
 * @param count
 */
void lnFast8bitIo::pulsesLow(int nb)
{
      
#define WRP    *onoff=down;*onoff=up;
    
    register uint32_t           up=_onbit,down=_offbit;
    volatile register uint32_t *onoff=_onoff;
    
    
        int block=nb>>4;
        int leftover=nb&15;
        for(int i=0;i<block;i++)
        {
#define PULSE            WRP; WRP;        
#define QUADD            PULSE;PULSE;PULSE;PULSE
             QUADD;
             QUADD;
             QUADD;
             QUADD;             
        }
        for(int i=0;i<leftover;i++)
        {        
            PULSE
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
#define EXP(x) x=  WR_DATA8(x)
    EXP(fhi);
    EXP(flo);
    EXP(bhi);
    EXP(blo);
    
    uint8_t *tail=len+data;
    while( data<tail)
    {
        if(*(data++))
        {
              *bop=  (fhi);   WRP;    *bop=(flo);  WRP;        
              continue;
        } else
        {
              *bop=  (bhi);   WRP;    *bop=( blo); WRP;    
        }
    }
}
// EOF
