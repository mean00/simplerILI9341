

#![no_std]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unreachable_code)]
extern crate ili9341;

extern crate rnarduino as rn;

use rn::rn_gpio as rnGpio;
use rn::rn_spi::rnSPI as rnSPI;

use rn::rn_gpio::rnPin as rnPin;
use rnarduino::rn_os_helper::delay_us;
use rnGpio::rnFastIO as rnFastIO;


use rnPin::NoPin;

use ili9341::access::Ili9341Access;
use ili9341::ili9341_cmds as cmd;
use rnarduino::rn_os_helper::delay_ms as rnDelay;

const SINGLE_TRANSFER : usize = (1<<16)-2;

//---------------------------------------
pub fn fail()
{
    panic!("oop");
}
//---------------------------------------
pub struct spi_ili9341
{
    spi      : rnSPI,
    pin_cs   : rnPin,
    pin_dc   : rnPin,
    pin_reset: rnPin,
    ioCS     : rnFastIO,
    ioDC     : rnFastIO,
    chip_id  : u32, 
    rotation : usize,
    x_offset : usize,
    y_offset : usize,
}
//---------------------------------------
impl spi_ili9341
{   
    pub fn new(spi: rnSPI , ics : rnPin , idc : rnPin, ireset : rnPin) -> Self
    {
        let mut me : spi_ili9341 = spi_ili9341
        {
                spi         : spi,
                pin_cs      : ics,
                pin_dc      : idc,
                pin_reset   : ireset, 
                ioCS        : rnFastIO::new(ics),
                ioDC        : rnFastIO::new(idc),
                rotation    : 0,
                x_offset    : 0,
                y_offset    : 0,
                chip_id     : 0x9341,
        };
        me
    }
    #[inline(always)]
    fn CS_ACTIVE(&mut self)
    {
        if self.pin_cs!= NoPin
        { 
            self.ioCS.off();
            
        }
    }
    #[inline(always)]
    fn CS_IDLE(&mut self)
    {
        if self.pin_cs!= NoPin
        { 
            self.ioCS.on();
        }
    }
    #[inline(always)]
    fn CD_DATA(&mut self)
    {       
        self.ioDC.on();       
    }
    #[inline(always)]
    fn CD_COMMAND(&mut self)
    {       
        self.ioDC.off();       
    }

    fn write_register32(&mut self, r: u8, val : u32)
    {    
      let flat: [u8;4]= [(val>>24& 0xff) as u8,((val>>16)&0xff) as u8,((val>>8)&0xff) as u8,(val&0xff) as u8];
      self.write_cmd_param(r,&flat);
    }

    fn read_register32(&mut self, r: u8) -> u32
    {    
        let mut rx : [u8;4] = [0;4];
        let mut tx : [u8;4] = [0;4];
        self.CS_ACTIVE();  
        self.spi.begin(8);
        self.CD_COMMAND();  
        self.spi.write16(r as u16);        
        self.CD_DATA();        
        delay_us(5);
        self.spi.transfer8(&tx,&mut rx);
        // revert
        let rx2: u32 =rx[3] as u32+( (rx[2] as u32) <<8)+( (rx[1] as u32) <<16)+( (rx[0] as u32) <<24);
        self.spi.end();
        self.CS_IDLE();
        rx2
    }
    fn read_chip_id(&mut self) -> u32
    {      
        return 0x9341;
        let reg_d3 = self.read_register32(0xd3);
        let mut reg_04 = self.read_register32(0x04);

        if reg_d3 == 0x9341
        {
            return 0x9341;
        }
        reg_04>>=8;
        reg_04&=0xffff;
        if reg_04 ==0x8552
        {                 
            return 0x7789; // is it really a 7789 ?
        }        
        return 0x9341; // unknown        
    }

    pub fn send_init_sequence(&mut self, data : &[u8])
    {
            let mut  index = 0;
            let len = data.len();
            while  index+2<len
            {
                let cmd = data[index];
                if cmd ==0
                {
                    return;
                }
                let run = data[index+1] as usize;
                index+=2;
                if cmd == 0xff
                {
                    rnDelay(len as u32);
                    continue;
                }
                self.spi.begin(8);
                self.CS_ACTIVE();
                self.write_cmd_param(cmd, &data[index..(index+run)]);
                index+=run;
                self.CS_IDLE();
                self.spi.end();
            }
    }
    pub fn reset(&mut self)
    {
        rnGpio::pinMode(self.pin_dc , rnGpio::rnGpioMode::lnOUTPUT);
        rnGpio::digital_write(self.pin_dc , true);
        if self.pin_cs!=NoPin
        {
            rnGpio::pinMode(self.pin_cs,rnGpio::rnGpioMode::lnOUTPUT);
            rnGpio::digital_write(self.pin_cs,true);
        }
        self.CS_IDLE();
        self.CD_DATA();
        if self.pin_reset != NoPin
        {            
            rnGpio::pinMode(self.pin_reset, rnGpio::rnGpioMode::lnOUTPUT);    
            rnGpio::digital_write(self.pin_reset,true);
            rnDelay(50);
            rnGpio::digital_write(self.pin_reset,false);
            rnDelay(50);
            rnGpio::digital_write(self.pin_reset,true);	
            rnDelay(50);
        }
        self.chip_id=self.read_chip_id();
    }

    fn write_cmd_param(&mut self, cmd : u8, data : &[u8] )
    {
        self.CD_COMMAND();
        self.spi.write8(cmd);
        if data.len()!=0 {
            self.CD_DATA();
            let n=data.len();
            for i in 0..n {
                self.spi.write8(data[i]);
            }
        }
    }
    fn send_data_to_screen(&mut self, data: &[u16]) 
    {     
        let n = data.len();
        // for small transfer do it directly...
        if n < 32
        {
            let n=data.len();
            for i in 0..n {
                self.spi.write16(data[i]);
            }
            return;
        }   
        self.spi.end();
        self.spi.block_write16(data);
        self.spi.begin(16);        
    }    
}
//-----------------------------------
impl Ili9341Access for spi_ili9341
{
    fn color_map(&self, d : u16) -> u16
    {
      //  if self.chip_id  != 0x7789
        //{
        //   return d;
        //}
        let r= d>>11;
        let b= d&0x1f;
        let g= (d>>5)&0x3f;
        return  r+(g<<5)+(b<<11);
    }

    fn send_byte(&mut self, b: u8)
    {
        self.spi.write8(b);
    }
    fn send_word(&mut self, b: u16)
    {
        self.spi.write16(b);
    }
    fn send_bytes(&mut self, data: &[u8]) {
        fail();
    }
    fn send_words(&mut self, data: &[u16]) {
        self.send_data_to_screen(data);
    }
    fn update_hw_rotation(&mut self, rotation: usize)
    {
        self.rotation = rotation;
        let mut t =        match self.chip_id
        {
            0x9341 => {
                match self.rotation
                {
                    1 =>        cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MY | cmd::ILI9341_MADCTL_MV ,
                    2 =>        cmd::ILI9341_MADCTL_MX ,
                    3 =>        cmd::ILI9341_MADCTL_MV ,
                    0 =>        cmd::ILI9341_MADCTL_MY ,
                    _ =>        {fail();0},
               }},
               
            0x7789 => {
                match self.rotation
                {
                     1 =>        cmd::ILI9341_MADCTL_MY | cmd::ILI9341_MADCTL_MV ,
                     2 =>        0 ,
                     3 =>        cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MV ,
                     0 =>        cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MY ,
                     _ =>        {fail();0},
               }},
            _ => {fail();0},
        };
        t|= cmd::ILI9341_MADCTL_RGB;
        self.spi.begin(8);
        self.CS_ACTIVE();
        let tray : [u8;1] = [t];
        self.write_cmd_param(cmd::ILI9341_MADCTL,&tray);  
        self.CS_IDLE();   
        self.spi.end();
    
    }
   
    fn set_address(&mut self, x: usize, y: usize, w: usize, h: usize)
    {
        let a1 : usize;
        let a2 : usize;
        let b1 : usize;
        let b2 : usize;

        a1=x+self.x_offset;
        a2=a1+w-1;
        b1=y+self.y_offset;
        b2=b1+h-1;
        self.spi.begin(8);
        self.CS_ACTIVE();    
        self.write_register32(cmd::ILI9341_COLADDRSET,  ((a1<<16) as u32) | (a2 as u32));  // HX8357D uses same registers!
        self.write_register32(cmd::ILI9341_PAGEADDRSET, ((b1<<16) as u32) | (b2 as u32)); // HX8357D uses same registers!
        self.CS_IDLE();
        self.spi.end();
    
    }
    fn data_end(&mut self)
    {        
        self.CD_COMMAND();
        self.CS_IDLE();
        self.spi.end();
    
    }
    fn data_begin(&mut self)
    {
        self.spi.begin(16);
        self.CS_ACTIVE();
        self.CD_COMMAND();
        self.spi.write16(cmd::ILI9341_MEMORYWRITE as u16);
        self.CD_DATA();
    
    }
    fn flood_words(&mut self, nb: usize, color: u16) 
    {
        let dupeColor = color; //colorMap(data);     
        let mut nb = nb;

        if nb > 64 // for "big" transfer use dma, else use plain polling
        {
            self.data_begin();
            self.spi.end();

            while nb>0
            {
                let mut r= nb;            
                if r>SINGLE_TRANSFER
                {
                    r=SINGLE_TRANSFER;
                }
                nb-=r;
                self.spi.block_write16_repeat(r,dupeColor);
            }               
            self.data_end();
            self.spi.begin(16);
        }else
        { // small transfer
            self.data_begin();
            while nb>0
            {
                let mut r= nb;            
                if r>SINGLE_TRANSFER
                {
                    r=SINGLE_TRANSFER;
                }
                nb-=r;
                self.spi.block_write16_repeat(r,dupeColor);
            }   
            self.data_end();
        }
    }          
}
//---------------------------------------
