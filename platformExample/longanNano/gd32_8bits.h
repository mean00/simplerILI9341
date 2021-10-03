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
#include "simpler9341.h"

/**
 * 
 * @param bop
 * @param writePin
 */
class lnFast8bitIo: public lnFastIO
{
    public : 
                lnFast8bitIo( uint32_t *bop, lnPin writePin) : lnFastIO(writePin)
                {
                    _bop=bop;
                }
                void pulsesLow(int count);
                void pulseData(int count, int hi, int lo);
                void push2Colors(int len, uint8_t *data,int fg, int bg);
                
    protected:                
                volatile uint32_t *_bop;
};

/**
 * 
 * @param w
 * @param h
 * @param spi
 * @param pinDc
 * @param pinCS
 */
class ln8bit9341 : public ili9341
{
    public:
                        ln8bit9341(int w, int h , int port,  int pinDc, int pinCS, int pinWrite, int pinRead, int pinReset);
            virtual      ~ln8bit9341();
            virtual void init();
            virtual void sendByte(int byte); // 8 bytes
            virtual void sendWord(int byte); // 16 bytes
            virtual void sendBytes(int nb, const uint8_t *data); // 8 bits
            virtual void sendWords(int nb, const uint16_t *data); // 16 bits
            virtual void floodWords(int nb, const uint16_t data); // 16 bits
            virtual void updateHwRotation();
            virtual void setAddress(int x, int y, int w, int h);
            virtual void push2Colors(uint8_t *data, int len, int fg, int bg);
            virtual void dataBegin();
            virtual void dataEnd();
            
            
                    uint32_t readChipId();
    protected:
                    void reset();
                    void writeCmdParam(uint16_t cmd, int payload, const uint8_t * data);
                    void writeCommand(uint16_t c);
                    void sendSequence(int size, const uint8_t *data);
                    void write8(uint8_t c);
                    uint8_t read8();
                    uint32_t readRegister32(int reg);
                    void writeRegister32(int r,uint32_t  val);
                    void setReadDir();
                    void setWriteDir();
                    
            lnFastIO     _ioRead, _ioCS, _ioDC;
            lnFast8bitIo *_ioWrite;
            lnPin       _pinReset;
            int         _dataPort;
            uint32_t    *_bop;
            uint32_t    _chipId;
};
// EOF