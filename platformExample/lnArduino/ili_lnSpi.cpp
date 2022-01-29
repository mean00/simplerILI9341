/*
 *  (C) Adafruit
 *  (C) 2021/2022 MEAN00 fixounet@free.fr
 *  See license file
 * 
 * This is a spi driver for the ILI9341/ST7789 
 * 
 * 
 * 
 */
#include "ili_lnSpi.h"
#include "simpler9341_priv.h"



#define CHECK_ARBITER() 
#define FAKE_DELAY_COMMAND 0xff
#define LOW_LEVEL_PRINT(...) {}

/**
 * 
 * @param w
 * @param h
 * @param spi
 * @param pinDc
 * @param pinCS
 */
lnSpi9341::lnSpi9341( int w, int h ,hwlnSPIClass *spi, int pinDC,int pinCS, int pinReset)   : ili9341(w,h),_ioDC(pinDC),_ioCS(pinCS)
{    
    _spi=spi;    
    _pinReset=pinReset;
    _pinDC=pinDC;
    _pinCS=pinCS;
    _PhysicalXoffset=0;
    _PhysicalYoffset=0;
    _cache=NULL;
    _cacheUsed=0;
    _cacheSize=0;

}
/**
 */
void lnSpi9341::enableCache(int cacheSizeInWords)
{
    _cacheUsed=0;
    _cacheSize=cacheSizeInWords;
    _cache=new uint16_t[_cacheSize];
}


/**
 * 
 */
lnSpi9341::~lnSpi9341()
{
    if(_cache)
    {
        delete [] _cache;
        _cache=NULL;
    }
}

#define CS_ACTIVE   {if(_pinCS!=-1) _ioCS.off();}
#define CS_IDLE     {if(_pinCS!=-1) _ioCS.on();}
#define CD_DATA     {_ioDC.on();}
#define CD_COMMAND  {_ioDC.off();}
/**
 * 
 */
void lnSpi9341::write8(uint8_t c)
{
    _spi->write(c);
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
  CS_ACTIVE;  
  CD_COMMAND;  
  _spi->write16(r);
  CD_DATA;
  uint32_t tx=0,rx=0;
  lnDelayUs(50);
  _spi->transfer(4,(uint8_t *)&tx,(uint8_t *)&rx);
  CS_IDLE;
  return rx;
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
  uint32_t regD3=readRegister32(0xd3)&0xffff ;
  uint32_t reg04=readRegister32(0x04)&0xffff ;
  
  if(regD3==0x9341) return 0x9341; // 9341 
  if(reg04==0x8552) return 0x7789; // is it really a 7789 ?
  Logger("Warning : Unknown LCD detected\n");
  return 0x9341; // unknown
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
void lnSpi9341::sendSequence( const uint8_t *data)
{
    
	while (*data ) 
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
void lnSpi9341::init(const uint8_t *init1, const uint8_t *init2)
{   
  
  reset();
  baseInit();
  sendSequence(init1); //sizeof(resetOff),resetOff);  
  sendSequence(init2); //(wakeOn),wakeOn);  
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
    if(!_cache)     
    {
        _spi->dmaWrite16(nb,data);    
        return;
    }
    if(_cacheUsed+nb*2>_cacheSize || nb>_cacheSize/2)
    {
        flushCache();
        _spi->dmaWrite16(nb,data);    
    }
    else
    {
        memcpy(_cache+_cacheUsed,data,nb*2);
        _cacheUsed+=nb;                
    }
}
/**
 */
void lnSpi9341::flushCache()
{
    xAssert(_cache);
    if(!_cacheUsed) return;
    _spi->dmaWrite16(_cacheUsed,_cache);    
    _cacheUsed=0;
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
    if(_cache && _cacheUsed)
    {
        flushCache();
    }
     CD_COMMAND;
     CS_IDLE;
}
/**
* 
* @param nb
* @param data
 * 
 *
*/
void lnSpi9341::floodWords(int nb, const uint16_t data)
{      
    uint16_t f=colorMap(data);
    dataBegin();
    while(nb)
    {        
        int chunk=nb;
        if(chunk>65534) chunk=65534;
        nb-=chunk;               
        _spi->dmaWrite16Repeat(chunk,f);        
    }
    dataEnd();        
}
// EOF

