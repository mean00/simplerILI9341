
#define WW  128
#define HH  160

    
#define ST7735_SWRESET 0x01
#define ST7735_SLPOUT  0x11
#define ST7735_NORON   0x13
#define ST7735_INVOFF  0x20
#define ST7735_INVON   0x21
#define ST7735_DISPON  0x29
#define ST7735_CASET   0x2A
#define ST7735_RASET   0x2B
#define ST7735_RAMWR   0x2C
#define ST7735_RAMRD   0x2E
#define ST7735_MADCTL  0x36
#define ST7735_COLMOD  0x3A

#define ST7735_FRMCTR1 0xB1
#define ST7735_FRMCTR2 0xB2
#define ST7735_FRMCTR3 0xB3
#define ST7735_INVCTR  0xB4
#define ST7735_DISSET5 0xB6
#define ST7735_FRMCTR1 0xB1
#define ST7735_FRMCTR2 0xB2
#define ST7735_FRMCTR3 0xB3
#define ST7735_INVCTR  0xB4
#define ST7735_DISSET5 0xB6
#define ST7735_PWCTR1  0xC0
#define ST7735_PWCTR2  0xC1
#define ST7735_PWCTR3  0xC2
#define ST7735_PWCTR4  0xC3
#define ST7735_PWCTR5  0xC4
#define ST7735_VMCTR1  0xC5
#define ST7735_GMCTRP1 0xE0
#define ST7735_GMCTRN1 0xE1




const uint8_t st7735_data[] = {
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
      ILI_CMD2(ST7735_COLMOD  , 0x55), // 0x3a: 0x6 -> 18 bits /pix, 0x5-> 15 bit/pixel
      ILI_CMD2(ST7735_MADCTL,   0x78), // 0x36: coord 08..168 / 120...200
      ILI_CMD(ST7735_DISPON),   
      ILI_CMD(ST7735_SLPOUT),   
      0

};