/*
 *  (C) Adafruit
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 */
#include "simpler9341.h"
#include "simpler9341_priv.h"

/**
 *
 * @param width
 * @param height
 * @param pinDc
 * @param pinCS
 */
ili9341::ili9341(int width, int height)
{
    _xOffset=0;
    _yOffset=0;
    _PhysicalXoffset=0;
    _PhysicalYoffset=0;
    _physicalWidth=width;
    _physicalHeight=height;
    _rotation=0;
    _width=_physicalWidth;
    _height=_physicalHeight;
    _fg=0xffff;
    _bg=0;
    _scrbuf=new uint16_t [ST7735_BUFFER_SIZE_WORD];
}
/**
 *
 */
ili9341::~ili9341()
{
    delete [] _scrbuf;
    _scrbuf=NULL;
}
/*
 *
 * @param color
 */
void ili9341::fillScreen(int color) // 16 bits!
{
    // ~ 180 is the max => 57600 ko, 176 is ok 56320
    #define ONE_GO 178
    if(_height>ONE_GO) // bug ? int color, int x, int y, int w, int h
    {
        square(color,0,0,_width,ONE_GO);
        square(color,0,ONE_GO,_width,_height-ONE_GO);
    }
    else
        square(color,0,0,_width,_height);
}
/**
 *
 * @param rotation
 */
void ili9341::setRotation(int rotation)
{
    _rotation=rotation;
    switch(rotation)
    {
        case 0:
        case 2:
                _xOffset=_PhysicalXoffset;
                _yOffset=_PhysicalYoffset;
                _width=_physicalWidth;
                _height=_physicalHeight;
                break;
        case 1:
        case 3:
                _xOffset=_PhysicalYoffset;
                _yOffset=_PhysicalXoffset;
                _width=_physicalHeight;
                _height=_physicalWidth;
                break;
        default : xAssert(0);
                break;
    }
    updateHwRotation();
}
/**
 *
 */
void ili9341::baseInit()
{

}


/**
 *
 * @param color
 * @param x
 * @param y
 * @param w
 * @param h
 */
void ili9341::square(int color, int x, int y, int w, int h)
{
    setAddress(x,y,w,h);
    int f=w*h;
    floodWords(f,color);
}
void ili9341::fillRoundRect(int x0, int y0, int w, int h,int radius, int outColor, int inColor)
{
    int ll=radius*w;
    int hh=radius*h;

      setAddress(x0,y0,w,radius);
      floodWords(ll,outColor);
      setAddress(x0,y0+h-radius,w,radius);
      floodWords(ll,outColor);

      setAddress(x0,y0,radius,h);
      floodWords(hh,outColor);
      setAddress(x0+w-radius,y0,radius,h);
      floodWords(hh,outColor);

      // interior now
      setAddress(x0+radius,y0+radius, w-2*radius,h-2*radius);
      floodWords( (w-2*radius)*(h-2*radius),inColor);

}

/**
 *
 * @param x0
 * @param y0
 * @param h
 * @param color
 */
void ili9341::VLine(int x0, int y0, int h, int color)
{
  setAddress(x0, y0, 1, h);
  floodWords(h,color);
}
/**
 *
 * @param x0
 * @param y0
 * @param w
 * @param color
 */
void ili9341::HLine(int x0, int y0, int w, int color)
{
  setAddress(x0, y0, w, 1);
  floodWords(w,color);
}
/**
 */
#define IS_7789()  (_chipId== 0x7789)
uint16_t ili9341::colorMap(const uint16_t d)
{
    if(!IS_7789()) return d;
    uint32_t r=(d>>11),b=d&0x1f,g=(d>>5)&0x3f;
    return r+(g<<5)+(b<<11);
}

// EOF
