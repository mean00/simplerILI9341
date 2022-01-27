#include "simpler9341.h"
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
void ili9341::innerLoop2NC(int w, int h, int left, int lineSize,int color, int bg,uint8_t *p)
{
    Logger("Font compression not activated\n");
    xAssert(0);
}

void ili9341::innerLoop2C(int w, int h, int left, int lineSize,int color, int bg,uint8_t *p)
{
    Logger("Font compression not activated\n");
    xAssert(0);
}

// EOF
