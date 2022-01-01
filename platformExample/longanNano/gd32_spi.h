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
                        lnSpi9341(int w, int h , hwlnSPIClass *spi,  int pinDc, int pinCS, int pinReset=-1);
            virtual      ~lnSpi9341();
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
            virtual void VLine(int x0, int y0, int h, int color);
            virtual void HLine(int x0, int y0, int w, int color);
            virtual void pushColors(int len, uint16_t *data);
            
            
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


            lnPin       _pinReset,_pinDC,_pinCS;
            uint32_t    _chipId;
            hwlnSPIClass    *_spi;
};
// EOF