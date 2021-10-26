#include "simpler9341.h"
#include "dso_colors.h"

/**
 * \fn checkFont
 * \brief extract max width/ max height from the font
 */
static void checkFont(const GFXfont *font, ili9341::FontInfo *info)
{
    int mW=0,mH=0;
    int x,y;
   for(int i=font->first;i<font->last;i++)
   {
         GFXglyph *glyph  = font->glyph+i-font->first;
         x=glyph->xAdvance;
         y=-glyph->yOffset;
         if(x>mW) mW=x;         
         if(y>mH) mH=y;
   }
    info->maxHeight=mH + 1;
    info->maxWidth=mW;    
    info->font=font;
}

void  ili9341::print(int x, int y,const char *z)
{
    cursor_x=x;
    cursor_y=y;
   int l=strlen(z);
   while(*z)
   {
       int inc=writeChar(*z);
       cursor_x+=inc;
       z++;
   }
}

/**
 * 
 * @param small
 * @param medium
 * @param big
 */
void  ili9341::setFontFamily(const GFXfont *small, const GFXfont *medium, const GFXfont *big)
{
    checkFont(small, fontInfo+0);
    checkFont(medium,fontInfo+1);
    checkFont(big,   fontInfo+2);
}       

/**
 * \fn mySquare
 * \brief Draw a square of w*xheight size at position x,y
 * \param filler is a prefill color array
 */
int ili9341::mySquare(int x, int y, int w, int xheight, uint16_t filler)
{
    if(w+x>=_width)
    {
        w=_width-x;
        if(w<=0) return 0;
    }
    if(xheight+y>=_height)
    {
        xheight=_height-y;
        if(xheight<=0)
            return 0;
    }
    setAddress(x,y,w, xheight);
    floodWords(w*xheight,filler);
    return 0;
}




/**
 * 
 * @param size
 */
void  ili9341::setFontSize(FontSize size)
{
    switch(size)
    {
        case SmallFont :    currentFont=fontInfo+0;break;
        default:
        case MediumFont :   currentFont=fontInfo+1;break;
        case BigFont :      currentFont=fontInfo+2;break;
    }    
    gfxFont=currentFont->font;
}  
/**
 * 
 * @param data
 * @param maxWidthInPixels
 */
void ili9341::printUpTo(const char *data, int maxWidthInPixels)
{
    int lastPos=cursor_x+maxWidthInPixels;
    int s=strlen(data);
    for(int i=0;i<s;i++)
    {
        if(cursor_x>=lastPos) 
            return;
        writeChar(data[i]);
    }
    if(lastPos>cursor_x)
    {
        int w=lastPos-cursor_x;
         while(w>0)
         {
            int rnd=w;
            if(rnd>currentFont->maxWidth) rnd=currentFont->maxWidth;
            
            mySquare(cursor_x,cursor_y-currentFont->maxHeight,                  rnd, currentFont->maxHeight+2, 0*WHITE+1*_bg);
            cursor_x+=rnd;
            w=lastPos-cursor_x;
            
         }

    }
}
/**
 * 
 * @param c
 * @return 
 */
int ili9341::writeChar(const char c) 
{

    if (c == '\n') 
    {
      cursor_x = 0;
      cursor_y +=           gfxFont->yAdvance;
      return 1;
    } 
    if(c=='\r')
      return 1;
    uint8_t first = gfxFont->first;
    if ((c < first) || (c > gfxFont->last)) 
        return 1;
    
    GFXglyph *glyph = gfxFont->glyph + c-first;
    int w = glyph->width,   h = glyph->height;
    
    // also ' ' here
    if ((w <= 0) || (h <= 0)) 
    {
        //
        mySquare(cursor_x,cursor_y-currentFont->maxHeight, 
                gfxFont->glyph->xAdvance,currentFont->maxHeight+glyph->yOffset,
                _bg);
        cursor_x += glyph->xAdvance ;    
        return 1;
    }

    int xo = glyph->xOffset; // sic
    if ( ((cursor_x +  (xo + w)) > _width)) 
    {
      cursor_x = 0;
      cursor_y +=   gfxFont->yAdvance;
    }    
    
    cursor_x += myDrawChar(cursor_x, cursor_y, c, _fg,_bg,*currentFont);    
    return 1;
}
// EOF