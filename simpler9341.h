/*
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 */
#pragma once
#include "Arduino.h"
#include "gfxfont.h"

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
            virtual void sendWords(int nb, const uint16_t *data)=0; // 16 bits
            virtual void updateHwRotation()=0;
            virtual void floodWords(int nb, const uint16_t data)=0; // 16 bits            
            virtual void setAddress(int x, int y, int w, int h)=0;
            virtual void push2Colors(uint8_t *data, int len, bool first,int fg, int bg)=0;
            virtual void dataEnd()=0;
            virtual void dataBegin()=0;
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

    
    
                         ili9341(int width, int height);
                virtual ~ili9341();
                virtual void init()=0;
                
                void fillScreen(int color); // 16 bits!
                
                void square(int color, int x, int y, int w, int g);
                void setRotation(int rotation);  // 0 1 2 3
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
                void    setTextColor(int fg,int bg)
                {
                    _fg=fg;
                    _bg=bg;
                }
                int     myDrawChar(int x, int y, unsigned char c,  int color, int bg,FontInfo &infos);
                void    setFontSize(FontSize size);
                void    setFontFamily(const GFXfont *small, const GFXfont *medium, const GFXfont *big);
                void    print(int x,int y,const char *z);
                int     writeChar(char c);
                void    print(const char *data)
                {
                    int s=strlen(data);
                    for(int i=0;i<s;i++)
                        writeChar(data[i]);
                }
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
                
protected:
                int     mySquare(int x, int y, int w, int xheight, uint16_t filler);
                
#define         ST7735_BUFFER_SIZE_WORD 256
                uint16_t scrbuf[ST7735_BUFFER_SIZE_WORD];
                int     cursor_x,cursor_y;


};



//EOF
