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

void ili9341::ILI_TEMPLATE_NAME(int w, int h, int left, int lineSize,int fg, int bg,uint8_t *p)
{
    int  bits = 0, mask = 0;
    uint16_t *col;
        
    
    ILI_TEMPLATE_PROLOG;
    uint16_t *start=(uint16_t *)scrbuf;
    start+=left;
    for( int line=h-1;line>=0;line--)
    {      
        col=start;
        // mid
        for( int xcol=w-1;xcol>=0;xcol--)
        {
            if(!mask) // reload ?
            {
                bits= ILI_TEMPLATE_NEXT();              
                mask = 0x80;
            }      
                            
            if(bits & mask)
            {
                *col++=fg;
            }else
            {
                *col++=bg;
            }
            mask>>=1;
        }
        sendWords(lineSize,(uint16_t *)scrbuf);
    }   
}


// EOF
