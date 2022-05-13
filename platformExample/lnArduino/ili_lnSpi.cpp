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

// for very small transfer it is more expensive to setup the DMA
// than to send the actual data, so do something very simple
#define simpleWrite16(size,data)   {for(int i=0;i<size;i++)      _spi->write16(data[i]);}
#define simpleWrite8(size,data)    {for(int i=0;i<size;i++)      _spi->write(data[i]);}
#define simpleWrite16R(size,data) {for(int i=0;i<size;i++)       _spi->write16(data);}


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
    _PhysicalYoffset=(320-h)/2; // (320-280)=40/2=20 ?
    _PhysicalXoffset=(240-w)/2; //  (240-240)/2=0 ?
    _cache=NULL;
    _cacheUsed=0;
    _cacheSize=0;
    _3wire=false;

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
void lnSpi9341::enable3WireMode()
{
    _3wire=true;
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
    _spi->write(cmd);
    if(payload)
    {
        CD_DATA;
        simpleWrite8(payload,data);
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
    uint32_t tx=0,rx2;  
    uint8_t rx[4];
    {
        CS_ACTIVE;  
        CD_COMMAND;  
        _spi->write(r);
        lnDelayUs(50);
        CD_DATA;        
        lnDelayUs(50);
        if(_3wire)
            _spi->read1wire( 4,rx);
        else
            _spi->transfer(4,(uint8_t *)&tx,rx);
        // revert
        rx2=rx[3]+(rx[2]<<8)+(rx[1]<<16)+(rx[0]<<24);
        CS_IDLE;
        Logger("Reading register32 0x%x => %x\n",r,rx2);
    }
    return rx2;
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
  uint32_t regD3=readRegister32(0xd3);
  uint32_t reg04=readRegister32(0x04);
  uint32_t reg09=readRegister32(0x09);
  uint32_t reg0c=readRegister32(0x0f);
  
  if(regD3==0x9341) 
  {
    Logger("ILI 9341 detected\n");
    return 0x9341; // 9341 
  }
  reg04>>=8;
  reg04&=0xffff;
  if(reg04 ==0x8552) 
  {
    Logger("ST7789 detected\n");
    return 0x7789; // is it really a 7789 ?
  }
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
#if 0
    int cmd=0xaa;
    uint8_t xdata=0x55;
    while(1)
    {
        CS_ACTIVE;         
        CD_COMMAND;
        _spi->write(cmd);
        CD_DATA;
        //_spi->write(0x55);
        _spi->dmaWrite(1,&xdata);
        CS_IDLE;
        lnDelay(1) ;
    }
#endif
	while (*data ) 
    {
        
        uint8_t cmd = data[0];
        uint8_t len = data[1];
        data+=2;
        if (cmd == FAKE_DELAY_COMMAND) 
        {			
            delay(len);
            continue;
        }        
        CS_ACTIVE;         
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
  if(init2)
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
        simpleWrite16(nb,data);
        return;
    }
    if(_cacheUsed+nb*2>_cacheSize || nb>_cacheSize/2)
    {
        flushCache();
        simpleWrite16(nb,data);
    }
    else
    {
        memcpy(_cache+_cacheUsed,data,nb*2);
        _cacheUsed+=nb;                
    }
}

/**
 * 
 */
void lnSpi9341::dataBegin()
{
    CS_ACTIVE;
    CD_COMMAND;
    _spi->write(ILI9341_MEMORYWRITE);
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

#define ONE_CHUNK ((1<<16)-2)
/**
 * 
 */
void lnIliAsyncCB(void *cookie)
{
    lnLinkedTranfer *t=(lnLinkedTranfer *)cookie;
    t->ili->nextFlood(t);
}
/**
 * 
 * 
 */
void lnSpi9341::nextFlood(lnLinkedTranfer *t)
{
    if(t->currentStep==t->nbStep)
    {
        _spi->finishAsyncDma();
        return;
    }
    _spi->waitForCompletion();
    CD_DATA;
    int dex=t->currentStep++;
    _spi->nextDmaWrite16(t->size[dex], t->data[dex], lnIliAsyncCB,t,true);
}
/**
 * 
 */
void lnSpi9341::floodWords(int nb, const uint16_t data)
{   
    _dupeColor=colorMap(data);     
    if(nb<100) // small transfer, dont bother doing something fancy
    {
        dataBegin();
        _spi->write16Repeat(nb,data);
        dataEnd();
        return;
    }
    
    _dupeCmd=ILI9341_MEMORYWRITE;
    lnLinkedTranfer transfer(this);

    while(nb)
    {        
        int chunk=nb;
        if(chunk>ONE_CHUNK) chunk=ONE_CHUNK;
        nb-=chunk;  
        transfer.add(chunk,&_dupeColor);
    }
    
    CD_COMMAND;
    lnDelay(10); // wait a few  us, we have an extra clk tick somewhere here, make sure it happens while CS is off
    CS_ACTIVE;
    _spi->asyncDmaWrite16(1, &_dupeCmd, lnIliAsyncCB,&transfer,true);    
    _spi->waitForAsyncDmaDone();
    _spi->finishAsyncDma();
    CS_IDLE;
    CD_COMMAND;
}

/**
 */
void lnSpi9341::flushCache()
{
    xAssert(_cache);
    if(!_cacheUsed) return;

    if(_cacheUsed<64)
    {
        simpleWrite16(_cacheUsed,_cache);    
    }else
    {
        simpleWrite16(_cacheUsed,_cache);    
    }
    _cacheUsed=0;
}

// EOF

