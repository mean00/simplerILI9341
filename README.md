# simplerILI9341
This is a simple &amp; portable ILI9341 library. It comes in two flavors : C++ and Rust \
It only deals with the core drawing and font display, taking advantage of the specificities of that controller
such as DMA repeat etc...

The library supports modified "Truetype"/adafruit fonts (
1 bit per pixel, 2 bit per pixel and font compression are supported). \
The library prints strings "in place", so you can avoid flickering effects (no need to clear-then-print, just print).

It also supports heatshrink-byte compressed one bpp bitmap.


# C++
You must derive the class and implements the hw dependant part.
As a result, it is highly portable, both in terms of hw (SPI, parallel,...) and in terms of MCU.

In your derived class , you have to provide the following methods :

    virtual void sendByte(int byte)=0; // 8 bytes
    virtual void sendWord(int byte)=0; // 16 bytes
    virtual void sendBytes(int nb, const uint8_t *data)=0; // 8 bits
    virtual void sendWords(int nb, const uint16_t *data)=0; // 16 bits
    virtual void updateHwRotation()=0;
    virtual void floodWords(int nb, const uint16_t data)=0; // 16 bits            
    virtual void setAddress(int x, int y, int w, int h)=0;
    virtual void dataEnd()=0;
    virtual void dataBegin()=0;
    virtual void pushColors(int len, uint16_t *data)=0;

# Rust

You must provide an "access" crate, have a look at the sim or lnArduino samples.


