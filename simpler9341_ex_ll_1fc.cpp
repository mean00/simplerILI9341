#include "simpler9341.h"
#include "simpler9341_ex_hs.h"
/**
 * 
 * @param w
 * @param h
 * @param left
 * @param lineSize
 * @param color
 * @param bg
 * @param p
 */
void ili9341::innerLoop1C(int w, int h, int left, int lineSize,int color, int bg,uint8_t *p)
{
    iliHS hs(p);
    
    int  bits = 0, bit = 0;
    uint8_t *col;
    for( int line=h-1;line>=0;line--)
    {      
        col=_column+left;     
        // mid
        for( int xcol=w-1;xcol>=0;xcol--)
        {
            
            if(!bit) // reload ?
            {
       
                bits= hs.next();
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
        push2Colors(_column,lineSize,color,bg);
    }  
}
// EOF
