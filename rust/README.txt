You have to provide an access object to the ili9341 library (fulfilling the access trait)
There is an exemple in demo_st7735_160x128/spi_ili9341.rs .
Additionnaly you have to provide an init sequence 
Two examples are provided :
- example 1: demo_st7735_160x128/st7735_init.rs  for a 160x128 1.8' ST7735 LCD
- example 2: ili9341/src/ili9341/ili9341_cmd/init_sequence for  320x240 3.2" ili9341 LCD