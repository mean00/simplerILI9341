/*
 *  This is coming from adafruit driver
 */
#pragma once

#define ILI_WAIT(x) FAKE_DELAY_COMMAND, x
#define ILI_CMD(x) x, 0
#define ILI_CMD2(x, y) x, 1, y
#define ILI_CMD3(x, y, z) x, 2, y, z
#define ILI_CMD4(x, y, z, t) x, 3, y, z, t
#define ILI_CMD5(x, y, z, t, u) x, 4, y, z, t, u
#define ILI_CMD6(x, y, z, t, u, v) x, 5, y, z, t, u, v

// Register names
#define ILI9341_SOFTRESET 0x01
#define ILI9341_SLEEPIN 0x10
#define ILI9341_SLEEPOUT 0x11
#define ILI9341_NORMALDISP 0x13
#define ILI9341_INVERTOFF 0x20
#define ILI9341_INVERTON 0x21
#define ILI9341_GAMMASET 0x26
#define ILI9341_DISPLAYOFF 0x28
#define ILI9341_DISPLAYON 0x29
#define ILI9341_COLADDRSET 0x2A
#define ILI9341_PAGEADDRSET 0x2B
#define ILI9341_MEMORYWRITE 0x2C
#define ILI9341_MEMCONTROL 0x36
#define ILI9341_MADCTL 0x36
#define ILI9341_PIXELFORMAT 0x3A
#define ILI9341_FRAMECONTROL 0xB1
#define ILI9341_DISPLAYFUNC 0xB6
#define ILI9341_ENTRYMODE 0xB7
#define ILI9341_POWERCONTROL1 0xC0
#define ILI9341_POWERCONTROL2 0xC1
#define ILI9341_VCOMCONTROL1 0xC5
#define ILI9341_VCOMCONTROL2 0xC7

#define ILI9341_MADCTL_MY 0x80
#define ILI9341_MADCTL_MX 0x40
#define ILI9341_MADCTL_MV 0x20
#define ILI9341_MADCTL_ML 0x10
#define ILI9341_MADCTL_RGB 0x00
#define ILI9341_MADCTL_BGR 0x08
#define ILI9341_MADCTL_MH 0x04

/**
 * \brief list of DMA transfer to perform
 * @param nb
 * @param data
 *
 *
 */
class lnSpi9341;
class lnLinkedTranfer
{
  public:
    lnLinkedTranfer(lnSpi9341 *me, bool rep)
    {
        ili = me;
        nbStep = 0;
        currentStep = 0;
        repeat = rep;
    }
    void setIndex(int i)
    {
        xAssert(i <= nbStep);
        currentStep = i;
    }
    void add(int nb, const uint16_t *c)
    {
        size[nbStep] = nb;
        data[nbStep] = c;
        nbStep++;
        xAssert(nbStep < 4)
    }
    int nbStep;
    int currentStep;
    int size[4];
    bool repeat;
    const uint16_t *data[4];
    lnSpi9341 *ili;
};