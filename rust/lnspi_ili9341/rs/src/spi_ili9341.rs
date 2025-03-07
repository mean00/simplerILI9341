#![no_std]
/*
 * Access crate to use SPI / LnArduino
 *
 */
extern crate ili9341;
extern crate rnarduino as rn;

use rn::rn_gpio as rnGpio;
use rn::rn_spi::rnSPI;

use rn::rn_gpio::rnPin;
use rn::rn_gpio::rn_fast_gpio::rnFastIO;

use rnPin::NoPin;

use ili9341::access::Ili9341Access;
use ili9341::ili9341_cmds as cmd;
use rnarduino::rn_os_helper::delay_ms as rnDelay;

const SINGLE_TRANSFER: usize = (1 << 16) - 2;

//---------------------------------------
pub fn fail() {
    panic!("oop");
}
//---------------------------------------
pub struct SpiIli9341 {
    spi: rnSPI,
    pin_dc: rnPin,
    pin_reset: rnPin,
    io_cs: Option<rnFastIO>,
    io_dc: rnFastIO,
    chip_id: u32,
    rotation: usize,
    x_offset: usize,
    y_offset: usize,
}
//---------------------------------------
impl SpiIli9341 {
    pub fn new(spi: rnSPI, ics: rnPin, idc: rnPin, ireset: rnPin) -> Self {
        let me: SpiIli9341 = SpiIli9341 {
            spi: spi,
            pin_dc: idc,
            pin_reset: ireset,
            io_cs: match ics {
                NoPin => None,
                x => Some(rnFastIO::new(x)),
            },
            io_dc: rnFastIO::new(idc),
            rotation: 0,
            x_offset: 0,
            y_offset: 0,
            chip_id: 0x9341,
        };
        me
    }
    #[inline(always)]
    fn cs_active(&mut self) {
        match &mut self.io_cs {
            Some(x) => x.off(),
            _ => (),
        }
    }
    #[inline(always)]
    fn cs_idle(&mut self) {
        match &mut self.io_cs {
            Some(x) => x.on(),
            _ => (),
        }
    }
    #[inline(always)]
    fn cd_data(&mut self) {
        self.io_dc.on();
    }
    #[inline(always)]
    fn cd_command(&mut self) {
        self.io_dc.off();
    }

    fn write_register32(&mut self, r: u8, val: u32) {
        let flat: [u8; 4] = [
            (val >> 24 & 0xff) as u8,
            ((val >> 16) & 0xff) as u8,
            ((val >> 8) & 0xff) as u8,
            (val & 0xff) as u8,
        ];
        self.write_cmd_param(r, &flat);
    }
    /*
    fn read_register32(&mut self, r: u8) -> u32 {
        let mut rx: [u8; 4] = [0; 4];
        let tx: [u8; 4] = [0; 4];
        self.cs_active();
        self.spi.begin(8);
        self.cd_command();
        self.spi.write16(r as u16);
        self.cd_data();
        delay_us(5);
        self.spi.transfer8(&tx, &mut rx);
        // revert
        let rx2: u32 =
            rx[3] as u32 + ((rx[2] as u32) << 8) + ((rx[1] as u32) << 16) + ((rx[0] as u32) << 24);
        self.spi.end();
        self.cs_idle();
        rx2
    }
    */
    fn read_chip_id(&mut self) -> u32 {
        return 0x9341;
        /*
        let reg_d3 = self.read_register32(0xd3);
        let mut reg_04 = self.read_register32(0x04);

        if reg_d3 == 0x9341 {
            return 0x9341;
        }
        reg_04 >>= 8;
        reg_04 &= 0xffff;
        if reg_04 == 0x8552 {
            return 0x7789; // is it really a 7789 ?
        }
        return 0x9341; // unknown
        */
    }

    pub fn send_init_sequence(&mut self, data: &[u8]) {
        let mut index = 0;
        let len = data.len();
        while index + 2 < len {
            let cmd = data[index];
            if cmd == 0 {
                return;
            }
            let run = data[index + 1] as usize;
            index += 2;
            if cmd == 0xff {
                rnDelay(len as u32);
                continue;
            }
            self.spi.begin(8);
            self.cs_active();
            self.write_cmd_param(cmd, &data[index..(index + run)]);
            index += run;
            self.cs_idle();
            self.spi.end();
        }
    }
    pub fn reset(&mut self) {
        rnGpio::pinMode(self.pin_dc, rnGpio::rnGpioMode::lnOUTPUT);
        rnGpio::digital_write(self.pin_dc, true);
        match &mut self.io_cs {
            Some(x) => x.on(),
            _ => (),
        }
        self.cs_idle();
        self.cd_data();
        if self.pin_reset != NoPin {
            rnGpio::pinMode(self.pin_reset, rnGpio::rnGpioMode::lnOUTPUT);
            rnGpio::digital_write(self.pin_reset, true);
            rnDelay(50);
            rnGpio::digital_write(self.pin_reset, false);
            rnDelay(50);
            rnGpio::digital_write(self.pin_reset, true);
            rnDelay(50);
        }
        self.chip_id = self.read_chip_id();
    }
    pub fn set_chip_id(&mut self, id: u32) {
        self.chip_id = id;
    }
    fn write_cmd_param(&mut self, cmd: u8, data: &[u8]) {
        self.cd_command();
        self.spi.write8(cmd);
        self.spi.wait_for_completion();
        if data.len() != 0 {
            self.cd_data();
            let n = data.len();
            for i in 0..n {
                self.spi.write8(data[i]);
            }
            self.spi.wait_for_completion();
        }
    }
    fn send_data_to_screen(&mut self, data: &[u16]) {
        let n = data.len();
        // for small transfer do it directly...
        if n < 32 {
            let n = data.len();
            for i in 0..n {
                self.spi.write16(data[i]);
            }
            return;
        }
        //self.spi.end();
        self.spi.block_write16(data);
        //self.spi.begin(16);
    }
}
//-----------------------------------
impl Ili9341Access for SpiIli9341 {
    fn color_map(&self, d: u16) -> u16 {
        //  if self.chip_id  != 0x7789
        //{
        //   return d;
        //}
        let r = d >> 11;
        let b = d & 0x1f;
        let g = (d >> 5) & 0x3f;
        return r + (g << 5) + (b << 11);
    }

    fn send_byte(&mut self, b: u8) {
        self.spi.write8(b);
    }
    fn send_word(&mut self, b: u16) {
        self.spi.write16(b);
    }
    fn send_bytes(&mut self, _data: &[u8]) {
        fail();
    }
    fn send_words(&mut self, data: &[u16]) {
        self.send_data_to_screen(data);
    }
    fn update_hw_rotation(&mut self, rotation: usize) {
        self.rotation = rotation;
        let mut t = match self.chip_id {
            0x7735 => match self.rotation {
                1 => cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MY,
                2 => cmd::ILI9341_MADCTL_MY | cmd::ILI9341_MADCTL_MV,
                3 => cmd::ILI9341_MADCTL_MY | cmd::ILI9341_MADCTL_MV | cmd::ILI9341_MADCTL_ML,
                0 => cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MV | cmd::ILI9341_MADCTL_ML,
                _ => {
                    fail();
                    0
                }
            },
            0x9341 => match self.rotation {
                1 => cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MY | cmd::ILI9341_MADCTL_MV,
                2 => cmd::ILI9341_MADCTL_MX,
                3 => cmd::ILI9341_MADCTL_MV,
                0 => cmd::ILI9341_MADCTL_MY,
                _ => {
                    fail();
                    0
                }
            },

            0x7789 => match self.rotation {
                1 => cmd::ILI9341_MADCTL_MY | cmd::ILI9341_MADCTL_MV,
                2 => 0,
                3 => cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MV,
                0 => cmd::ILI9341_MADCTL_MX | cmd::ILI9341_MADCTL_MY,
                _ => {
                    fail();
                    0
                }
            },
            _ => {
                fail();
                0
            }
        };
        t |= cmd::ILI9341_MADCTL_RGB;
        self.spi.begin(8);
        self.cs_active();
        let tray: [u8; 1] = [t];
        self.write_cmd_param(cmd::ILI9341_MADCTL, &tray);
        self.cs_idle();
        self.spi.end();
    }

    fn set_address(&mut self, x: usize, y: usize, w: usize, h: usize) {
        let a1: usize;
        let a2: usize;
        let b1: usize;
        let b2: usize;

        a1 = x + self.x_offset;
        a2 = a1 + w - 1;
        b1 = y + self.y_offset;
        b2 = b1 + h - 1;
        self.spi.begin(8);
        self.cs_active();
        self.write_register32(cmd::ILI9341_COLADDRSET, ((a1 << 16) as u32) | (a2 as u32)); // HX8357D uses same registers!
        self.write_register32(cmd::ILI9341_PAGEADDRSET, ((b1 << 16) as u32) | (b2 as u32)); // HX8357D uses same registers!
        self.cs_idle();
        self.spi.end();
    }
    fn data_end(&mut self) {
        self.cd_command();
        self.cs_idle();
        self.spi.end();
    }
    fn data_begin(&mut self) {
        self.spi.begin(16);
        self.cs_active();
        self.cd_command();
        self.spi.write16(cmd::ILI9341_MEMORYWRITE as u16);
        self.spi.wait_for_completion();
        self.cd_data();
    }
    fn flood_words(&mut self, nb: usize, color: u16) {
        let dupe_color = color; //colorMap(data);
        let mut nb = nb;
        self.data_begin();
        while nb > 0 {
            let mut r = nb;
            if r > SINGLE_TRANSFER {
                r = SINGLE_TRANSFER;
            }
            nb -= r;
            self.spi.block_write16_repeat(r, dupe_color);
        }
        self.data_end();
    }
}
//---------------------------------------
