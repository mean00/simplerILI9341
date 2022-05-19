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

#define CIRCLE_ADVANCE(xx,yy,E) { \
        if(E>0)\
        {\
                xx--;\
                E-=8*xx;\
                    \
        }\
        yy++;\
        E+=8*yy+4;\
    }

/**
 */
void ili9341::circle(int color, int x, int y, int radius)
{
// https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
    int f=colorMap(color);
    int E=5-4*radius;
    int yy=0;
    int xx=radius;
    while(yy<xx)
    {
            CIRCLE_ADVANCE(xx,yy,E)
            // Use simple symetry
            setAddress(x+xx,y+yy,1,1);
            floodWords(1,f);
            setAddress(x-xx,y+yy,1,1);
            floodWords(1,f);
            setAddress(x-xx,y-yy,1,1);
            floodWords(1,f);
            setAddress(x+xx,y-yy,1,1);
            floodWords(1,f);
            // Use 45 degrees symetry, swapping x & y
            setAddress(x+yy,y+xx,1,1);
            floodWords(1,f);
            setAddress(x-yy,y+xx,1,1);
            floodWords(1,f);
            setAddress(x-yy,y-xx,1,1);
            floodWords(1,f);
            setAddress(x+yy,y-xx,1,1);
            floodWords(1,f);

    }
}


/**
 * 
 */
void ili9341::disc(int color, int x, int y, int radius)
{
// https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
    int f=colorMap(color);
    int E=5-4*radius;
    int yy=0;
    int xx=radius;
    while(yy<=xx)
    {

            setAddress(x+xx-2*xx,y+yy,2*xx,1);
            floodWords(2*xx,f);

            setAddress(x+xx-2*xx,y-yy,2*xx,1);
            floodWords(2*xx,f);       

            setAddress(x+yy-2*yy,y+xx,2*yy,1);
            floodWords(2*yy,f);

            setAddress(x+yy-2*yy,y-xx,2*yy,1);
            floodWords(2*yy,f);       

            CIRCLE_ADVANCE(xx,yy,E)
    }
}

/**
 * 
 */
void ili9341::invertedDiscCorner(int color, int x, int y, int radius,int corner)
{
// https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
    int f=colorMap(color);
    int E=5-4*radius;
    int yy=0;
    int xx=radius;
    while(yy<=xx)
    {
        if(corner & 2)
        {
            setAddress(x+xx,y-yy+radius-1,radius,1);
            floodWords(radius-xx,f);
            setAddress(x+yy,y-xx+radius-1,radius,1);
            floodWords(radius-yy,f);
        }
        if(corner & 1)
        {
            setAddress(x,radius+y-yy,radius,1);
            floodWords(radius-xx,f);
            setAddress(x,radius+y-xx,radius,1);
            floodWords(radius-yy,f);            
        }        
#define OFS 1        
        if(corner & 4)
        {
            setAddress(x,y+yy-OFS,radius,1);
            floodWords(radius-xx,f);
            setAddress(x,y+xx-OFS,radius,1);
            floodWords(radius-yy,f);            
        }        
        if(corner & 8)
        {
            setAddress(x+xx,y+yy-OFS,radius,1);
            floodWords(radius-xx,f);
            setAddress(x+yy,y+xx-OFS,radius,1);
            floodWords(radius-yy,f);
        }
        CIRCLE_ADVANCE(xx,yy,E)
    }
}
/**
 * 
 */
int  ili9341::quarterDisc(int mx, int radius,int preload, uint16_t *out)
{
    int E=5-4*radius;
    int yy=0;
    int xx=radius;
    if(preload*2>=radius) return 0;
    for(int i=0;i<preload;i++)
    {
        if(E>0)
        {
            xx--;
            E-=8*xx;
        }
        yy++;
        E+=8*yy+4;
    }
    int nb=0;
    for(int i=preload;i<radius;i++)
    {
        out[nb++]=xx;
        if(nb*2>=radius) return nb;
        if(nb>=mx) return nb;
        if(E>0)
        {
            xx--;
            E-=8*xx;
        }
        yy++;
        E+=8*yy+4;
    }
    return nb;
}

/**
 * \brief this is slightly incorrect, we done deal with swapping xx & yy correctly
 */
void ili9341::fillRoundRect(int x0, int y0, int w, int h,int radius, int outColor, int inColor)
{
    int sizes[6];
    const uint16_t colors[3]={(uint16_t)colorMap(outColor),(uint16_t)colorMap(inColor),(uint16_t)colorMap(outColor)};
    
  
    int E=5-4*radius;
    int yy=0;
    int xx=radius;
    int lastLine=-1;
    int lastValue=-1;
    while(yy<=xx)
    {
            sizes[2]=sizes[0]=radius-xx;
            sizes[1]=w-2*sizes[0];  

            setAddress(x0,radius+y0-yy,w,1);
            multiFloodWords(3,sizes,colors);
            setAddress(x0,y0+h-(radius-yy)-1,w,1);
            multiFloodWords(3,sizes,colors);

            // XY symetry
            int line=xx;
            if(line!=lastLine && lastLine!=-1)
            {
                sizes[2]=sizes[0]=lastValue;
                sizes[1]=w-2*sizes[0]; 
                setAddress(x0,radius+y0-lastLine,w,1);
                multiFloodWords(3,sizes,colors);
                setAddress(x0,y0+h-(radius-lastLine)-1,w,1);
                multiFloodWords(3,sizes,colors);                
            }
            lastLine=line;
            lastValue=radius-yy;
            CIRCLE_ADVANCE(xx,yy,E)
    }

    // interior now
    setAddress(x0,y0+radius, w,h-2*radius);
    floodWords( w*(h-2*radius),inColor);
}

// EOF