use super::Ili9341;
const ST7735_BUFFER_SIZE_WORD : usize = 320; // TODO FIXME, DUPLICATE WITH TOP
impl<'a> Ili9341<'a> {
    // width is in pixel
    pub fn drawRLEBitmap(&mut self, width : usize, height: usize, wx: usize, wy: usize, fg: u16, bg : u16, data : &[u8])
    {        
        
        self.access.set_address(wx, wy,  width, height);

        let nb_pixels =width * height;
        let mut pixel: usize = 0;        
        let mut cur : usize;
        let mut ready : usize=0;
        let mut repeat:usize;
        let mut color : u16;
        let mut index: usize =0;
        let mut o : *mut u16 = self.src_buf;
        self.access.data_begin();
        while pixel<nb_pixels
        {
            // load next
            cur=data[index] as usize;
            index+=1;
            if cur==0x76
            {
                cur=data[index] as usize;
                repeat=data[index+1] as usize;
                index+=2;
            }else
            {
                repeat=1;
            }
            // 8 pixels at a time
            for _r in 0..repeat
            {
                let mut mask=0x80;
                for _i in 0..8
                {
                    if (mask & cur)!=0
                    {
                        color=fg;
                    }else
                    {
                        color=bg; //color=0xff00*0+1*bg;
                    }
                    mask>>=1;
                    unsafe {
                        *o=color;o=o.add(1);
                    }
                    ready+=1;
                }
                if ready>ST7735_BUFFER_SIZE_WORD-16
                { // Flush
                    unsafe  {
                        self.access.send_words( core::slice::from_raw_parts(self.src_buf, ready));
                    }
                    ready=0;
                    o= self.src_buf;
                }
            }
            pixel+=repeat*8;
        }
        if ready>0
        {
            unsafe{
            self.access.send_words( core::slice::from_raw_parts(self.src_buf, ready) );
            }
        }
        self.access.data_end();
    }    
}
