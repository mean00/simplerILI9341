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
ln8bit9341::ln8bit9341( int w, int h ,int port, int pinDC,int pinCS, int pinWrite, int pinRead, int pinReset)   : ili9341(w,h), _ioWrite(pinWrite),_ioRead(pinRead),_ioCS(pinCS),_ioDC(pinDC)
{    
    _dataPort=port;    
    _pinReset=pinReset;
    _PhysicalXoffset=0;
    _PhysicalYoffset=0;
    _bop=lnGetGpioToggleRegister(_dataPort);
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

#define WR_ACTIVE   {_ioWrite.off();}
#define WR_IDLE     {_ioWrite.on();}

#define RD_ACTIVE   {_ioRead.off();}
#define RD_IDLE     {_ioRead.on();}

#define WR_STROBE {WR_ACTIVE;WR_IDLE;}
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
    write8(cmd>>8);
    write8(cmd);
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
 * 
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
  val = read8();          // Do not merge or otherwise simplify
  val <<= 8;              // these lines.  It's an unfortunate  
  val  |= read8();        // shenanigans that are going on.
  val <<= 8;              // these lines.  It's an unfortunate
  val  |= read8();;        // shenanigans that are going on.
  val <<= 8;              // these lines.  It's an unfortunat
  val  |= read8();;        // shenanigans that are going on.
  CS_IDLE;
  return val;
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
        _ioWrite.on();
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
void ln8bit9341::sendWord(int byte)
{
    //    _spi->write16(byte);
}
/**
 * 
 * @param nb
 * @param data
 */
void ln8bit9341::sendBytes(int nb, const uint8_t *data)
{
    //    _spi->write(nb,data);
}
/**
 * 
 * @param nb
 * @param data
 */
void ln8bit9341::sendWords(int nb, const uint16_t *data)
{
#ifdef SPI_USE_DMA      
    //    _spi->dmaWrite16(nb,data);
#else
//     for(int i=0;i<nb;i++)
 //         _spi->write16(data[i]);      
#endif
}
/**
* 
* @param nb
* @param data
*/
void ln8bit9341::floodWords(int nb, const uint16_t data)
{
#ifdef SPI_USE_DMA  
    //      _spi->dmaWrite16Repeat(nb,data);
#else
//      for(int i=0;i<nb;i++)
  //        _spi->write16(data);      
#endif
}
  
static const uint8_t rotMode[4]={0x8,0xc8,0x78,0xa8};
/**
 * 
 */
void ln8bit9341::updateHwRotation(void)
{
//    sendCommand(ST7735_MADCTL,1,rotMode+_rotation);
}

void ln8bit9341::dataMode()
{
    
}
void ln8bit9341::cmdMode()
{
    
}


// EOF