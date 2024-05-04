
#pragma once
#include "stdint.h"
#define WW  176
#define HH  320

#define XOFFSET 0 //2
#define YOFFSET 0 //2
    
#include "simpler_9341_st7735_common.h"


// If the color are wrong red/blue => cyan magenta
// replace ST7735_INVON by ST7735_INVOFF or the other way around

const uint8_t st7735_data[] = {
      ILI_CMD(ST7735_SWRESET),  //  1: Software reset, no args, w/delay
      ILI_WAIT(10),
      ILI_CMD(ST7735_SLPOUT),  //  2: Out of sleep mode, no args, w/delay
      ILI_WAIT(10),
      ILI_CMD(ST7735_INVON),
      ILI_CMD4(ST7735_FRMCTR1,  0x05, 0x3a, 0x3a),
      ILI_CMD4(ST7735_FRMCTR2,  0x05, 0x3a, 0x3a), 
      ST7735_FRMCTR3, 6  ,  0x05, 0x3a, 0x3a, 0x05, 0x3a, 0x3a,
      ILI_CMD2(ST7735_INVCTR,   0x03),
      ILI_CMD4(ST7735_PWCTR1,   0x62, 0x02, 0x04),
      ILI_CMD2(ST7735_PWCTR2,   0xc0),
      ILI_CMD3(ST7735_PWCTR3,   0x0d, 0x00),
      ILI_CMD3(ST7735_PWCTR4,   0x8d, 0x6a),
      ILI_CMD3(ST7735_PWCTR5,   0x8d, 0xee),
      ILI_CMD2(ST7735_VMCTR1,   0x0e),
      ST7735_GMCTRP1,  16   , 0x10, 0x0e, 0x02, 0x03, 0x0e, 0x07, 0x02, 0x07, 0x0a, 0x12, 0x27, 0x37, 0x00, 0x0d, 0x0e, 0x10,
      ST7735_GMCTRN1,  16   , 0x10, 0x0e, 0x03, 0x03, 0x0f, 0x06, 0x02, 0x08, 0x0a, 0x13, 0x26, 0x36, 0x00, 0x0d, 0x0e, 0x10,
      ILI_CMD2(ST7735_COLMOD  , 0x55), // 0x3a: 0x6 -> 18 bits /pix, 0x5-> 15 bit/pixel /3 12 bits pixel
      //
      ST7735_CASET, 4, XOFFSET>>8, (XOFFSET)&0xff, (WW+XOFFSET-1)>>8, (WW+XOFFSET-1)&0xff, // Column addr set: XSTART=0, XEND=width
      ST7735_RASET, 4, YOFFSET>>8, (YOFFSET)&0xff, (HH+YOFFSET-1)>>8, (HH+YOFFSET-1)&0xff, // Row addr set: YSTART=0, YEND=height
      // 
      ILI_CMD2(ST7735_MADCTL,        
      // No rotation MX+MY+MV+ML
      // Rotation 90 : MY
      // Rotation 180 :  MX
      // Rotation 270 : MX+ML

                 // ILI9341_MADCTL_MX  +
                  ILI9341_MADCTL_MY  +
                 // ILI9341_MADCTL_MV  +
                 // ILI9341_MADCTL_ML  +
                   +
                   ILI9341_MADCTL_RGB
                   ), //  0x28 <-inverted->:   0x78: <-normal->, || 48 , || 18
      ILI_CMD(ST7735_DISPON),         
      0

};
