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
            virtual void dataMode();
            virtual void cmdMode();
            virtual void sendByte(int byte); // 8 bytes
            virtual void sendWord(int byte); // 16 bytes
            virtual void sendBytes(int nb, const uint8_t *data); // 8 bits
            virtual void sendWords(int nb, const uint16_t *data); // 16 bits
            virtual void floodWords(int nb, const uint16_t data); // 16 bits
            virtual void updateHwRotation();
                    uint32_t readChipId();
    protected:
                    void reset();
                    void writeCmdParam(uint16_t cmd, int payload, const uint8_t * data);
                    void writeCommand(uint16_t c);
                    void sendSequence(int size, const uint8_t *data);
                    void write8(uint8_t c);
                    uint8_t read8();
                    uint32_t readRegister32(int reg);
                    void setReadDir();
                    void setWriteDir();
                    
            lnFastIO _ioWrite, _ioRead, _ioCS, _ioDC;
            lnPin    _pinReset;
            int     _dataPort;
            volatile uint32_t *_bop;
};
// EOF