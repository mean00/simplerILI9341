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
#define ILI_TEMPLATE_NAME   innerLoop1NC
#define ILI_TEMPLATE_PROLOG {}
#define ILI_TEMPLATE_NEXT() *p++

#include "simpler9341_ex_ll_1bpp_template.h"

#ifdef ILI_ENABLE_COMPRESSION
    #include "simpler9341_ex_hs.h"  
    #undef ILI_TEMPLATE_NAME
    #undef ILI_TEMPLATE_PROLOG
    #undef ILI_TEMPLATE_NEXT
    #define ILI_TEMPLATE_NAME       innerLoop1C
    #define ILI_TEMPLATE_PROLOG     iliHS hs(p);
    #define ILI_TEMPLATE_NEXT()     bits= hs.next();
    #include "simpler9341_ex_ll_1bpp_template.h"

#else
void ili9341::innerLoop1C(int w, int h, int left, int lineSize,int color, int bg,uint8_t *p)
{
    Logger("Font compression not activated\n");
    xAssert(0);
}
#endif


// EOF
