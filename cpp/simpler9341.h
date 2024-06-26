/*
 *  (C) 2021 MEAN00 fixounet@free.fr
 *  See license file
 */
#pragma once
#include "Arduino.h"
#include "pfxfont.h"
#include "simpler9341_color.h"

#define FAKE_DELAY_COMMAND 0xff

enum ChipID
{
    CHIPID_ILI9341 = 0x9341,
    CHIPID_ST7789 = 0x7789,
    CHIPID_ST7735 = 0x7735,
};

/**
 *  ChipID:
 *          0x7789 : ST7789
 *          0x9341 : ILI9341
 *
 */
class ili9341
{
  public:
    //             this is the part you have to reimplment for your setup
    //              in the derived class
    //---------------------------------------------------------------
    virtual void sendByte(int byte) = 0;                      // 8 bytes
    virtual void sendWord(int byte) = 0;                      // 16 bytes
    virtual void sendBytes(int nb, const uint8_t *data) = 0;  // 8 bits
    virtual void sendWords(int nb, const uint16_t *data) = 0; // 16 bits, that one must be inside dataBegin() dataEnd()
    virtual void updateHwRotation() = 0;
    virtual void floodWords(int nb, const uint16_t data) = 0; // 16 bits
    virtual void setAddress(int x, int y, int w, int h) = 0;
    virtual void dataEnd() = 0;
    virtual void dataBegin() = 0;
    virtual void pushColors(int len, uint16_t *data) = 0;
    virtual void multiFloodWords(int n, int *size, const uint16_t *colors)
    {
        for (int i = 0; i < n; i++)
        {
            floodWords(size[i], colors[i]);
        }
    }

    //---------------------------------------------------------------
    enum FontSize
    {
        SmallFont,
        MediumFont,
        BigFont
    };
    class FontInfo
    {
      public:
        int maxHeight;
        int maxWidth;
        const GFXfont *font;
    };
    FontInfo fontInfo[3];

    FontInfo *currentFont;
    const GFXfont *gfxFont;
    int _fg, _bg;
    uint32_t _chipId;

    ili9341(int width, int height);
    virtual ~ili9341();
    virtual void init(const uint8_t *init1, const uint8_t *init2) = 0;
    void forceChipId(int chipID)
    {
        _chipId = chipID;
    }
    void setInvertedDirection(bool invert)
    {
        _invertedDir = invert;
    }

    void fillScreen(int color); // 16 bits!
    void fillRoundRect(int x0, int y0, int w, int h, int radius, int outColor, int inColor);

    void square(int color, int x, int y, int w, int g);
    void setRotation(int rotation); // 0 1 2 3
    void VLine(int x0, int y0, int h, int color);
    void HLine(int x0, int y0, int w, int color);
    void drawLine(int x0, int y0, int x1, int y1, int color);
    void circle(int color, int x, int y, int radius);
    void disc(int color, int x, int y, int radius); // same as circle but filled
    void invertedDiscCorner(int color, int x, int y, int radius,
                            int corner); // draw 1/4 of corner 1=> top left, 2 top right, 4 bottom left 8 bottom right
    int quarterDisc(int mx, int radius, int preload,
                    uint16_t *out); // calculate at most mx disc offset starting from preload
    void setCursor(int x, int y)
    {
        cursor_x = x;
        cursor_y = y;
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
    void getTextColor(int &f, int &g)
    {
        f = _fg;
        g = _bg;
    }
    void setTextColor(int fg, int bg)
    {
        _fg = fg;
        _bg = bg;
    }
    uint32_t getChipId()
    {
        return _chipId;
    }
    int myDrawChar(int x, int y, unsigned char c, int color, int bg, FontInfo &infos);
    void setFontSize(FontSize size);
    void setFontFamily(const GFXfont *small, const GFXfont *medium, const GFXfont *big);
    int stringLength(const char *z); // lengh in pixel of the string
    void print(int x, int y, const char *z);
    int writeChar(const char c);
    void print(const char *data)
    {
        int s = strlen(data);
        for (int i = 0; i < s; i++)
            writeChar(data[i]);
    }
    void printUpTo(const char *data, int maxWidthInPixels);
    void print(int a)
    {
        char bf[10];
        sprintf(bf, "%d", a);
        int s = strlen(bf);
        for (int i = 0; i < s; i++)
            writeChar(bf[i]);
    }
    void putPixel(int x, int y, int color);
    void drawRLEBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data);
    void drawHSBitmap(int widthInPixel, int height, int wx, int wy, int fgcolor, int bgcolor, const uint8_t *data);

  protected:
    int charLength(const char c);
    int mySquare(int x, int y, int w, int xheight, uint16_t filler);
    void innerLoop1NC(int w, int h, int left, int advance, int fg, int bg, uint8_t *p);
    void innerLoop1C(int w, int h, int left, int advance, int fg, int bg, uint8_t *p);
    void innerLoop2C(int w, int h, int left, int advance, int fg, int bg, uint8_t *p);
    void innerLoop2NC(int w, int h, int left, int advance, int fg, int bg, uint8_t *p);
    uint16_t colorMap(const uint16_t d);
#define ST7735_BUFFER_SIZE_WORD 320
    uint16_t *_scrbuf;
    int cursor_x, cursor_y;
    bool _invertedDir;

  public:
    void centeredButton(int x, int y, int w, int h, const char *text, int fontColor, int buttonColor, int bgColor);
};

// EOF
