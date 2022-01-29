/*
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 */
#pragma once
#include "Arduino.h"
#include "pfxfont.h"
#include "simpler9341_color.h"

#define FAKE_DELAY_COMMAND 0xff
/**
 */
class ili9341
{
public:
//             this is the part you have to reimplment for your setup
//              in the derived class
//---------------------------------------------------------------
            virtual void sendByte(int byte)=0; // 8 bytes
            virtual void sendWord(int byte)=0; // 16 bytes
            virtual void sendBytes(int nb, const uint8_t *data)=0; // 8 bits
            virtual void sendWords(int nb, const uint16_t *data)=0; // 16 bits, that one must be inside dataBegin() dataEnd()
            virtual void updateHwRotation()=0;
            virtual void floodWords(int nb, const uint16_t data)=0; // 16 bits            
            virtual void setAddress(int x, int y, int w, int h)=0;
            virtual void dataEnd()=0;
            virtual void dataBegin()=0;
            virtual void pushColors(int len, uint16_t *data)=0;
                uint16_t colorMap(const uint16_t d);
//---------------------------------------------------------------
               enum FontSize
               {
                   SmallFont,MediumFont,BigFont
               };
               class FontInfo
               {
               public:
                 int               maxHeight;
                 int               maxWidth;
                 const GFXfont    *font;
               };
               FontInfo          fontInfo[3];

               FontInfo          *currentFont;
               const GFXfont     *gfxFont;
               int               _fg,_bg;
               uint32_t          _chipId;


                         ili9341(int width, int height);
                virtual ~ili9341();
                virtual void init(const uint8_t *init1, const uint8_t *init2)=0;

                void fillScreen(int color); // 16 bits!
                void fillRoundRect(int x0, int y0, int w, int h,int radius, int outColor,int inColor);

                void square(int color, int x, int y, int w, int g);
                void setRotation(int rotation);  // 0 1 2 3
                void VLine(int x0, int y0, int h, int color);
                void HLine(int x0, int y0, int w, int color);

                void setCursor(int x, int y)
                {
                     cursor_x=x;
                     cursor_y=y;
                }
protected:
                int _physicalWidth;
                int _physicalHeight;
                int _width;
                int _height;
                int _rotation;
                int _PhysicalXoffset;
                int _PhysicalYoffset;
                int _xOffset;
                int _yOffset;
protected:
                void baseInit();

public: // freetype font
                void    getTextColor(int &f,int &g)
                {
                    f=_fg;
                    g=_bg;
                }
                void    setTextColor(int fg,int bg)
                {
                    _fg=fg;
                    _bg=bg;
                }
                int     myDrawChar(int x, int y, unsigned char c,  int color, int bg,FontInfo &infos);
                void    setFontSize(FontSize size);
                void    setFontFamily(const GFXfont *small, const GFXfont *medium, const GFXfont *big);
                void    print(int x,int y,const char *z);
                int     writeChar(const char c);
                void    print(const char *data)
                {
                    int s=strlen(data);
                    for(int i=0;i<s;i++)
                        writeChar(data[i]);
                }
                void printUpTo(const char *data, int maxWidthInPixels);
                 void    print(int a)
                {
                    char bf[10];
                    sprintf(bf,"%d",a);
                    int s=strlen(bf);
                    for(int i=0;i<s;i++)
                        writeChar(bf[i]);
                }
                void    putPixel(int x,int y, int color);
                void    drawRLEBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data);
                void    drawHSBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data);
                

protected:
                int     mySquare(int x, int y, int w, int xheight, uint16_t filler);
                void    innerLoop1NC(int w, int h, int left,int advance, int fg,int bg,uint8_t *p);
                void    innerLoop1C(int w, int h, int left,int advance, int fg,int bg,uint8_t *p);
                void    innerLoop2C(int w, int h, int left,int advance, int fg,int bg,uint8_t *p);
                void    innerLoop2NC(int w, int h, int left,int advance, int fg,int bg,uint8_t *p);

#define         ST7735_BUFFER_SIZE_WORD 256
                uint16_t _scrbuf[ST7735_BUFFER_SIZE_WORD];
                int     cursor_x,cursor_y;

};



//EOF
