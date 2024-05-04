// This is the init sequence for ST7735 12x160
use ili9341::ili9341_cmds::*;
pub const WW: usize = 176;
pub const HH: usize = 320;
pub const XOFFSET: usize = 0;
pub const YOFFSET: usize = 0;

// If the color are wrong red/blue => cyan magenta
// replace ST7735_INVON by ST7735_INVOFF or the other way around

pub const ST7735: &[u8] = &[
   ST7735_SWRESET, 0,  //  1: Software reset, no args, w/delay
   //ILI_WAIT(10),
   ST7735_SLPOUT,  0,  //  2: Out of sleep mode, no args, w/delay
   //ILI_WAIT(10),
   ST7735_INVON,   0,
   ST7735_FRMCTR1, 3, 0x05, 0x3a, 0x3a,
   ST7735_FRMCTR2, 3, 0x05, 0x3a, 0x3a, 
   ST7735_FRMCTR3, 6, 0x05, 0x3a, 0x3a, 0x05, 0x3a, 0x3a,
   ST7735_INVCTR,  1, 0x03,
   ST7735_PWCTR1,  3, 0x62, 0x02, 0x04,
   ST7735_PWCTR2,  1, 0xc0,
   ST7735_PWCTR3,  2, 0x0d, 0x00,
   ST7735_PWCTR4,  2, 0x8d, 0x6a,
   ST7735_PWCTR5,  2, 0x8d, 0xee,
   ST7735_VMCTR1,  1, 0x0e,
   ST7735_GMCTRP1,16, 0x10, 0x0e, 0x02, 0x03, 0x0e, 0x07, 0x02, 0x07, 0x0a, 0x12, 0x27, 0x37, 0x00, 0x0d, 0x0e, 0x10,
   ST7735_GMCTRN1,16, 0x10, 0x0e, 0x03, 0x03, 0x0f, 0x06, 0x02, 0x08, 0x0a, 0x13, 0x26, 0x36, 0x00, 0x0d, 0x0e, 0x10,
   ST7735_COLMOD,  1, 0x55, // 0x3a: 0x6 -> 18 bits /pix, 0x5-> 15 bit/pixel /3 12 bits pixel
   //
   ST7735_CASET, 4, ((XOFFSET>>8) & 0xff) as u8, ((XOFFSET)&0xff) as u8, 
     ((WW+XOFFSET-1)>>8) as u8, ((WW+XOFFSET-1)&0xff) as u8, // Column addr set: XSTART=0, XEND=width
   ST7735_RASET, 4, ((YOFFSET>>8) & 0xff) as u8, ((YOFFSET)&0xff) as u8, 
     ((HH+YOFFSET-1)>>8) as u8, ((HH+YOFFSET-1)&0xff) as u8, // Row addr set: YSTART=0, YEND=height
   // 
   ST7735_MADCTL, 1,
   // No rotation MX+MY+MV+ML
   // Rotation 90 : MY
   // Rotation 180 :  MX
   // Rotation 270 : MX+ML

     // ILI9341_MADCTL_MX  +
      ILI9341_MADCTL_MY  +
     // ILI9341_MADCTL_MV  +
     // ILI9341_MADCTL_ML  +
       
       ILI9341_MADCTL_RGB
       , //  0x28 <-inverted->:   0x78: <-normal->, || 48 , || 18
   ST7735_DISPON,   0,
   0

];
// OLD
pub const ST7735_OLD: &[u8] = &[
 ST7735_INVOFF,
 0,
 ST7735_FRMCTR1,
 3,
 0x05,
 0x3a,
 0x3a,
 ST7735_FRMCTR2,
 3,
 0x05,
 0x3a,
 0x3a,
 ST7735_FRMCTR3,
 6,
 0x05,
 0x3a,
 0x3a,
 0x05,
 0x3a,
 0x3a,
 ST7735_INVCTR,
 1,
 0x03,
 ST7735_PWCTR1,
 3,
 0x62,
 0x02,
 0x04,
 ST7735_PWCTR2,
 1,
 0xc0,
 ST7735_PWCTR3,
 2,
 0x0d,
 0x00,
 ST7735_PWCTR4,
 2,
 0x8d,
 0x6a,
 ST7735_PWCTR5,
 2,
 0x8d,
 0xee,
 ST7735_VMCTR1,
 1,
 0x0e,
 ST7735_GMCTRP1,
 16,
 0x10,
 0x0e,
 0x02,
 0x03,
 0x0e,
 0x07,
 0x02,
 0x07,
 0x0a,
 0x12,
 0x27,
 0x37,
 0x00,
 0x0d,
 0x0e,
 0x10,
 ST7735_GMCTRN1,
 16,
 0x10,
 0x0e,
 0x03,
 0x03,
 0x0f,
 0x06,
 0x02,
 0x08,
 0x0a,
 0x13,
 0x26,
 0x36,
 0x00,
 0x0d,
 0x0e,
 0x10,
 ST7735_COLMOD,
 1,
 0x05, // 0x3a: 0x6 -> 18 bits /pix, 0x5-> 15 bit/pixel /3 12 bits pixel
 //
 ST7735_CASET,
 4,
 (XOFFSET >> 8) as u8,
 ((XOFFSET) & 0xff) as u8,
 ((WW + XOFFSET - 1) >> 8) as u8,
 ((WW + XOFFSET - 1) & 0xff) as u8, // Column addr set: XSTART=0, XEND=width
 ST7735_RASET,
 4,
 (YOFFSET >> 8) as u8,
 ((YOFFSET) & 0xff) as u8,
 ((HH + YOFFSET - 1) >> 8) as u8,
 ((HH + YOFFSET - 1) & 0xff) as u8, // Row addr set: YSTART=0, YEND=height
 //
 ST7735_MADCTL,
 1,
 0x78, //  0x28 <-inverted->:   0x78: <-normal->, || 48 , || 18
 ST7735_DISPON,
 0,
 ST7735_SLPOUT,
 0,
 0,
];
// EOF
