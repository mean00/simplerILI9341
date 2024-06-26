use super::Ili9341;
use crate::settings::ST7735_BUFFER_SIZE_WORD;

impl<'a> Ili9341<'a> {
    #[cfg(feature = "hs")]
    pub fn draw_bitmap_hs(
        &mut self,
        width: usize,
        height: usize,
        wx: usize,
        wy: usize,
        fg: u16,
        bg: u16,
        hsdata: &'a [u8],
    ) {
        self.hs.reset(hsdata);
        self.access.set_address(wx, wy, width, height);

        let nb_pixels = width * height;
        let mut pixel: usize = 0;
        let mut cur: usize;
        let mut ready: usize = 0;
        let mut color: u16;

        let mut output_index: usize = 0;

        self.access.data_begin();
        let fg = self.access.color_map(fg);
        let bg = self.access.color_map(bg);
        while pixel < nb_pixels {
            // load next
            cur = self.hs.next() as usize;
            let mut mask = 0x80;
            for _i in 0..8 {
                if (mask & cur) != 0 {
                    color = fg;
                } else {
                    color = bg; //color=0xff00*0+1*bg;
                }
                mask >>= 1;
                self.src_buf[output_index] = color;
                output_index += 1;
                ready += 1;
            }
            if ready > ST7735_BUFFER_SIZE_WORD - 16 {
                // Flush
                self.access.send_words(&self.src_buf[..ready]);
                ready = 0;
                output_index = 0;
            }

            pixel += 8;
        }
        if ready > 0 {
            self.access.send_words(&self.src_buf[..ready]);
        }
        self.access.data_end();
    }

    //-------------------------------------
    pub fn draw_bitmap(
        &mut self,
        width: usize,
        height: usize,
        wx: usize,
        wy: usize,
        fg: u16,
        bg: u16,
        bitmapdata: &'a [u8],
    ) {
        self.access.set_address(wx, wy, width, height);

        let nb_pixels = width * height;
        let mut pixel: usize = 0;
        let mut cur: usize;
        let mut ready: usize = 0;
        let mut color: u16;
        let mut output_index: usize = 0;

        self.access.data_begin();
        let fg = self.access.color_map(fg);
        let bg = self.access.color_map(bg);
        let mut ix = 0;
        while pixel < nb_pixels {
            // load next
            cur = bitmapdata[ix] as usize;
            ix += 1;
            let mut mask = 0x80;
            for _i in 0..8 {
                if (mask & cur) != 0 {
                    color = fg;
                } else {
                    color = bg; //color=0xff00*0+1*bg;
                }
                mask >>= 1;
                self.src_buf[output_index] = color;
                output_index += 1;
                ready += 1;
            }
            if ready > ST7735_BUFFER_SIZE_WORD - 16 {
                // Flush
                self.access.send_words(&self.src_buf[..ready]);
                ready = 0;
                output_index = 0;
            }

            pixel += 8;
        }
        if ready > 0 {
            self.access.send_words(&self.src_buf[..ready]);
        }
        self.access.data_end();
    }
}
