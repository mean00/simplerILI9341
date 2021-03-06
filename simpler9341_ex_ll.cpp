#include "simpler9341.h"
/**
 * 
 * @param x
 * @param y
 * @param c
 * @param color
 * @param bg
 * @param infos
 * @return 
 */
#define debug(...) {}

/**
 * 
 * @param x
 * @param y
 * @param c
 * @param color
 * @param bg
 * @param infos
 * @return 
 */
int ili9341::myDrawChar(int x, int y, unsigned char c,  int fg, int bg,FontInfo &infos)
{
    c -= infos.font->first;
    GFXglyph *glyph  = infos.font->glyph+c;
    
    uint8_t *p= infos.font->bitmap+glyph->bitmapOffset;        
    int  w  = glyph->width;
    int  h  = glyph->height;    
    int  advv=glyph->xAdvance+1;
    
   
    debug(uint16_t oldbg=bg);
    debug(bg=YELLOW);
    debug(fg=GREEN);
    
  
    
    // Special case, space, it has yOffsset > 0
    if(infos.font->first+c==' ')
    {
        int adv=advv;
        int top=infos.maxHeight;
         mySquare(x,
                  y-top,
                  adv, //Fix!
                  top+2,bg);
         return adv;
    }
    
    // top & bottom
    int top=infos.maxHeight+glyph->yOffset; 
    mySquare(x,
            y-infos.maxHeight,
            advv,
            top,bg);

    int bottom=-glyph->yOffset-h;
    if(bottom>=-2)
        mySquare(x,y-bottom,advv,bottom+2,bg);      
    
    fg=colorMap(fg);
    bg=colorMap(bg);

    y+= glyph->yOffset;   // offset is <0 most of the time
    
    int left=glyph->xOffset;
    int right=(int)advv-(w+left);
    if(right<0) right=0;
       
     
    
    setAddress(x,y, advv, h);
    debug(bg=oldbg);
    
    
    // Pre-fill & left /right
    uint16_t *col;
    
    col=(uint16_t *)_scrbuf;
    for(int i=0;i<left;i++)
            *col++=bg;
        
    col+=w;

    for(int i=0;i<right;i++)
            *col++=bg;
    // fill in body
    
    dataBegin();

// dont draw out of screen 
    if(y+h>=_height)
    {
        h=_height-y;
        if(h<0) h=0;
    }

    switch(infos.font->bpp)
    {
        case 1:
            if(infos.font->shrinked)
                innerLoop1C(w,h,left,advv,fg,bg,p);
            else
                innerLoop1NC(w,h,left,advv,fg,bg,p);
            break;
        case 2:
            if(infos.font->shrinked)
                innerLoop2C(w,h,left,advv,fg,bg,p);
            else
                innerLoop2NC(w,h,left,advv,fg,bg,p);
            break;
                 
        default:
            xAssert(0);
            break;
    }
    dataEnd();
    return glyph->xAdvance;
}


/**
 * This could be optimized ** A LOT ** by swapping x & y in the source image
 * @param widthInPixel
 * @param height
 * @param wx
 * @param wy
 * @param fgcolor
 * @param bgcolor
 * @param data
 */

void ili9341::drawRLEBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data)
{    
    
    int nbPixel=widthInPixel*height;
    int pixel=0;
    setAddress(wx, wy,  widthInPixel, height);
    int mask=0;
    int cur;   
    uint16_t *o=_scrbuf;
    int ready=0;
    int repeat;
    uint16_t color;
    dataBegin();
    while(pixel<nbPixel)        
    {
        // load next
        cur=*data++;
        if(cur==0x76)
        {
            cur=*data++;
            repeat=*data++;
        }else
        {
            repeat=1;
        }
        // 8 pixels at a time
        for(int r=0;r<repeat;r++)
        {
            int mask=0x80;
            for(int i=0;i<8;i++)
            {
                if(mask & cur)
                {
                    color=fgcolor;
                }else
                    color=0xff00*0+1*bgcolor;
                mask>>=1;
                *o++=color;
                ready++;
            }
            if(ready>(ST7735_BUFFER_SIZE_WORD-16))
            { // Flush
              sendWords(ready,_scrbuf);
              ready=0;
              o=_scrbuf;
            }
        }
        pixel+=repeat*8;
    }
    if(ready)
        sendWords(ready,_scrbuf);
    dataEnd();
    
}


// EOF
