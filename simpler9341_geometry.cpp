/*
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 */
#include "simpler9341.h"
#include "simpler9341_priv.h"

/**
 *
 * @param x0
 * @param y0
 * @param h
 * @param color
 */
void ili9341::VLine(int x0, int y0, int h, int color)
{
  int f=colorMap(color);
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
  int f=colorMap(color);
  setAddress(x0, y0, w, 1);
  floodWords(w,color);
}
#define SWAP(a,b) {int c=a;a=b;b=c;}
/**
 * 
 */
void ili9341::drawLine(int x0, int y0, int x1, int y1, int color)
{
   
    if(y0>y1) SWAP(y0,y1);
    if(x0>x1) SWAP(x0,x1);

    if(x0==x1)  {VLine(x0,y0,y1-y0+1,color);return;}
    if(y0==y1)  {HLine(x0,y0,x1-x0+1,color);return;}
    int f=colorMap(color);
    int dx=x1-x0+1;
    int dy=y1-y0+1;
    if(dx<dy)
    {
            int inc=(dy*2048)/dx;
            int val=0;
            int posy=y0;
            for(int x=x0;x<=x1;x++)
            {
                val+=inc;
                int h=(val>>11);
                val&=(1<<11)-1;
                setAddress(x,posy,1,h);
                floodWords(h,f);                
                posy+=h;
                
            }
            return;
    }
    int inc=(dx*2048)/dy;
    int val=0;
    int posx=x0;
    for(int y=y0;y<=y1;y++)
    {
        val+=inc;
        int h=(val>>11);
        setAddress(posx,y,h,1);
        floodWords(h,f);                
        posx+=h;
        val&=(1<<11)-1;
    }    
}
// EOF