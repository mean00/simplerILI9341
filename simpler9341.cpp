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
     if(width>height) _column=new uint8_t[width+1];
     else _column=new uint8_t[height+1];
}
/**
 * 
 */
ili9341::~ili9341()
{
    delete [] _column;
    _column=NULL;
}
/*
 * 
 * @param color
 */              
void ili9341::fillScreen(int color) // 16 bits!
{
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

// EOF