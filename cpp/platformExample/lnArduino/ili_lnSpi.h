/*
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 
  This is a 8 bit parrallel mode for the DSO150
  The code is a bit hardcoded for the following pinout, you can easily duplicate/change it 
  for another 8 bit parallel screen
 * 
 * Read Signal PA6
 * Write Signal PC15
 * ChipSelect  PC13
 * Reset       PB9
 * RS/CD       PC14
 * 
 * DATA = PB0...PB7
 
 */
#pragma once
#include "lnSPI.h"
#include "simpler9341.h"

class lnLinkedTranfer;

/**
 * 
 * @param w
 * @param h
 * @param spi
 * @param pinDc
 * @param pinCS
 */
class lnSpi9341 : public ili9341
{
    public:
                        lnSpi9341(int w, int h , lnSPI *spi,  lnPin pinDc, lnPin pinCS, lnPin pinReset=NoPin);
            virtual      ~lnSpi9341();
                    void enableCache(int cacheSizeInWords);
                    void enable3WireMode(); // call this if you have only CLK/SDA i.e. not MOSI AND MISO, MISO is used for both tx & rx
            virtual void init(const uint8_t *init1, const uint8_t *init2);
            virtual void sendByte(int byte); // 8 bytes
            virtual void sendWord(int byte); // 16 bytes
            virtual void sendBytes(int nb, const uint8_t *data); // 8 bits
            virtual void sendWords(int nb, const uint16_t *data); // 16 bits
            virtual void floodWords(int nb, const uint16_t data); // 16 bits
            virtual void updateHwRotation();
            virtual void setAddress(int x, int y, int w, int h);            
            virtual void dataBegin();
            virtual void dataEnd();
            virtual void multiFloodWords(int n, int *size, const uint16_t *colors);
            virtual void pushColors(int len, uint16_t *data) {xAssert(0);}
                    void setOffset(int xOffset, int yOffset);
            
            
                    uint32_t readChipId();
    protected:                    
                    void reset();
                    void writeCmdParam(uint16_t cmd, int payload, const uint8_t * data);
                    void writeCommand(uint16_t c);
                    void sendSequence(const uint8_t *data);
                    void write8(uint8_t c);
                    uint32_t readRegister32(int reg);
                    void writeRegister32(int r,uint32_t  val);                    
                    void flushCache();
                    void sendDataToScreen(int nb, const uint16_t *data);
            lnPin           _pinReset,_pinDC,_pinCS;
            lnFastIO        _ioCS,_ioDC;
            lnSPI           *_spi;
#define ILI_BUFFER_SIZE 320            
            uint16_t        _buffer[ILI_BUFFER_SIZE];
            uint16_t        *_cache;
            uint16_t         _dupeColor;
            uint16_t         _dupeCmd;
            int             _cacheUsed;
            int             _cacheSize;
            bool            _3wire;

public:
            void             nextFlood(lnLinkedTranfer *);
};
// EOF