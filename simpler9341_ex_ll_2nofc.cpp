#include "simpler9341.h"
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
void ili9341::innerLoop2C(int w, int h, int left, int lineSize,int color, int bg,uint8_t *p)
{
    xAssert(0);
}
/**
 * 
 * @param w
 * @param h
 * @param left
 * @param right
 * @param lineSize
 * @param fg
 * @param bg
 * @param p
 */
void ili9341::innerLoop2NC(int w, int h, int left, int lineSize,int fg, int bg,uint8_t *p)
{
    
#define RR(a,b)     (((((    fg>>0  &0x1f)*a+  (bg>>0) &0x1f)*b)/(a+b))<<0)
#define GG(a,b)     (((((( (fg>>5) &0x3f)*a+ ((bg>>5) &0x3f)*b))/(a+b)))<<5)
#define BB(a,b)     (((((( (fg>>11)&0x1f)*a+ ((bg>>11)&0x1f)*b))/(a+b)))<<11)
    
    int low=RR(1,3)+GG(1,3)+BB(1,3);
    int hi=RR(3,1)+GG(3,1)+BB(3,1);
    
    low=0x1f<<11;
    hi=0x1f;
    
    const uint16_t colorGrad[4]={bg,low,hi,fg};        
    
    uint16_t *col;
    
    col=(uint16_t *)_column;
    for(int i=0;i<left;i++)
            *col++=bg;
        
    col+=w;
    int right=lineSize-left;
    for(int i=0;i<right;i++)
            *col++=bg;
    
    int  bits = 0, rank = 0;        
    for( int line=h-1;line>=0;line--)
    {      
        col=(uint16_t *)_column;
        col+=left;
        // mid
        for( int xcol=w-1;xcol>=0;xcol--)
        {
            if(rank<0) // reload ?
            {
                bits= *p++;              
                rank = 6;
            }      
            int pix=(bits>>rank)&3;
            rank-=2;
            
            *col++=colorGrad[pix];
        }
        sendWords(lineSize,(uint16_t *)_column);
    }   
}

// EOF
