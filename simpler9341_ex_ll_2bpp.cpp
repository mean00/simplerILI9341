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

#define ILI_TEMPLATE_NAME innerLoop2NC
#define ILI_TEMPLATE_PROLOG                                                    \
  {}
#define ILI_TEMPLATE_NEXT() *p++

#include "simpler9341_ex_ll_2bpp_template.h"

#ifdef ILI_ENABLE_COMPRESSION
#include "simpler9341_ex_hs.h"
#undef ILI_TEMPLATE_NAME
#undef ILI_TEMPLATE_PROLOG
#undef ILI_TEMPLATE_NEXT
#define ILI_TEMPLATE_NAME innerLoop2C
#define ILI_TEMPLATE_PROLOG iliHS hs(p);
#define ILI_TEMPLATE_NEXT() bits = hs.next();
#include "simpler9341_ex_ll_2bpp_template.h"

#else
void ili9341::innerLoop2C(int w, int h, int left, int lineSize, int color,
                          int bg, uint8_t *p) {
  Logger("Font compression not activated\n");
  xAssert(0);
}
#endif

// EOF
