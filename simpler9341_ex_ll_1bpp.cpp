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
#define debug(...)                                                             \
  {}
#define ILI_TEMPLATE_NAME innerLoop1NC
#define ILI_TEMPLATE_PROLOG                                                    \
  {}
#define ILI_TEMPLATE_NEXT() *p++

#include "simpler9341_ex_ll_1bpp_template.h"

#ifdef ILI_ENABLE_COMPRESSION
#include "simpler9341_ex_hs.h"
#undef ILI_TEMPLATE_NAME
#undef ILI_TEMPLATE_PROLOG
#undef ILI_TEMPLATE_NEXT
#define ILI_TEMPLATE_NAME innerLoop1C
#define ILI_TEMPLATE_PROLOG iliHS hs(p);
#define ILI_TEMPLATE_NEXT() bits = hs.next();
#include "simpler9341_ex_ll_1bpp_template.h"

void ili9341::drawHSBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data)
{    
    iliHS hs(data);
    int nbPixel=widthInPixel*height;
    int pixel=0;
    setAddress(wx, wy,  widthInPixel, height);
    int mask=0;
    int cur;   
    uint16_t *o=_scrbuf;
    int ready=0;
   
    uint16_t color;
    dataBegin();
    while(pixel<nbPixel)        
    {
        // load next
        cur= hs.next();
        int mask=0x80;
        for(int i=0;i<8;i++)
        {
            if(mask & cur)
            {
                color=fgcolor;
            }else
                color=bgcolor;
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
        pixel+=8;
    }
    if(ready)
        sendWords(ready,_scrbuf);
    dataEnd();
    
}
#else
void ili9341::innerLoop1C(int w, int h, int left, int lineSize, int color,
                          int bg, uint8_t *p) {
  Logger("Font compression not activated\n");
  xAssert(0);
}
#endif

// EOF
