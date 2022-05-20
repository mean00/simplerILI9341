/*
 *  (C) Adafruit
 *  (C) 2022 MEAN00 fixounet@free.fr
 *  See license file
 */
#include "simpler9341.h"
#include "simpler9341_priv.h"

/**
 * 
 */
void ili9341::centeredButton(int x, int y, int w, int h, const char *text, int fontColor,int buttonColor,int bgColor)
{
    // 1 Draw button
    fillRoundRect( x, y,w,h, h/4, bgColor,buttonColor);

    // 2 Draw text
    int textW=stringLength(text);
    int left=(w-textW)/2;
    if(left<0) left=0;
    setTextColor(fontColor,buttonColor);
    print(x+left,y+((h*3)/4),text);
}