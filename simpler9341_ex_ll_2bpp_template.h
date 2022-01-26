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
void ili9341::ILI_TEMPLATE_NAME(int w, int h, int left, int lineSize,int fg, int bg,uint8_t *p)
{
    
#define RR(a,b)     (((((    fg>>0  &0x1f)*a+  (bg>>0) &0x1f)*b)/(a+b))<<0)
#define GG(a,b)     (((((( (fg>>5) &0x3f)*a+ ((bg>>5) &0x3f)*b))/(a+b)))<<5)
#define BB(a,b)     (((((( (fg>>11)&0x1f)*a+ ((bg>>11)&0x1f)*b))/(a+b)))<<11)
    
    
#define C(x,shift,bits)    ((x>>shift)&bits)
#define D(x,shift)    ((x<<shift))
    
#define R(l,r)     D((C(fg,0,0x1f)*l+C(bg,0,0x1f)*r)/(l+r),0)
#define G(l,r)     D((C(fg,5,0x3f)*l+C(bg,5,0x3f)*r)/(l+r),5)
#define B(l,r)     D((C(fg,11,0x1f)*l+C(bg,11,0x1f)*r)/(l+r),11)
    int low=R(1,3)+G(1,3)+B(1,3);
    int hi=R(3,1)+G(3,1)+B(3,1);
    
    ILI_TEMPLATE_PROLOG;
    
#if 1    
    //low=0x1f<<11;
    //hi=0x3f<<5;    
    //fg=bg=0;
#endif    
    
    const uint16_t colorGrad[4]={(uint16_t)bg,(uint16_t)low,(uint16_t)hi,(uint16_t)fg};
    
    
    int  bits = 0, rank = -1;        
    uint16_t *start;
    start=(uint16_t *)_column;
    start+=left;
    uint16_t *col;
    for( int line=0;line<h;line++)
    {      
        col=start;
        // mid
        for( int xcol=0;xcol<w;xcol++)
        {
            if(rank<0) // reload ?
            {
                bits= ILI_TEMPLATE_NEXT();
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
