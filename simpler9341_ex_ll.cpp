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
int ili9341::myDrawChar(int x, int y, unsigned char c,  int color, int bg,FontInfo &infos)
{
    c -= infos.font->first;
    GFXglyph *glyph  = infos.font->glyph+c;
    
    uint8_t *p= infos.font->bitmap+glyph->bitmapOffset;        
    int  w  = glyph->width;
    int  h  = glyph->height;    
    int  advv=glyph->xAdvance+1;
    
   
    debug(uint16_t oldbg=bg);
    debug(bg=YELLOW);
    debug(color=GREEN);
    
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
    

    y+= glyph->yOffset;   // offset is <0 most of the time
    
    int left=glyph->xOffset;
    int right=(int)advv-(w+left);
    if(right<0) right=0;
       
    int    finalColor;    
    int  bits = 0, bit = 0;
    setAddress(x,y, advv, h);
    debug(bg=oldbg);
    uint8_t  *col=_column;
    
    // Pre-fill & left /right
    memset(col,0,left);
    memset(col+left+w,0,right);
    // fill in body
    
    dataBegin();
    for( int line=h-1;line>=0;line--)
    {      
        col=_column+left;     
        // mid
        for( int xcol=w-1;xcol>=0;xcol--)
        {
            if(!bit) // reload ?
            {
                bits= *p++;
                if(xcol>=8) // speed up some special cases
                {
                uint8_t *table8=(uint8_t *)_lookup;
                uint8_t *high=table8+(bits>>4)*4;
                uint8_t *low=table8+(bits&0xf)*4;                
                col[0]=high[0];
                col[1]=high[1];
                col[2]=high[2];
                col[3]=high[3];
                col[4]=low[0];
                col[5]=low[1];
                col[6]=low[2];
                col[7]=low[3];
                                
                //memcpy(col,high,4);
                //memcpy(col+4,low,4);
                xcol-=7;
                bit=0;
                col+=8;
                continue;
                }
                bit = 0x80;
            }      
                            
            *col++=!!(bits & bit) ;  
            bit=bit>>1;
        }
        // 9ms here
        push2Colors(_column,advv,color,bg);
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
    uint16_t *o=scrbuf;
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
              sendWords(ready,scrbuf);
              ready=0;
              o=scrbuf;
            }
        }
        pixel+=repeat*8;
    }
    if(ready)
        sendWords(ready,scrbuf);
    dataEnd();
    
}


// EOF
