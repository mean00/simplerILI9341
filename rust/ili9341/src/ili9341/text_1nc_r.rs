use super::Ili9341;

impl<'a> Ili9341<'a> {
    pub fn inner_loop_1nc(
        &mut self,
        w: usize,
        h: usize,
        left: usize,
        line_size: usize,
        fg: u16,
        bg: u16,
        p: &'a [u8],
    ) {
        let mut bits: usize = 0;
        let mut mask: usize = 0;
        let mut col: usize;
        let mut ix: usize = 0;
        let mut start: usize = 0;
        start = start + left;

        for _line in 0..h
        // for( int line=h-1;line>=0;line--)
        {
            col = start;
            // mid
            //for( int xcol=w-1;xcol>=0;xcol--)
            for _xcol in 0..w {
                if mask == 0
                // reload ?
                {
                    bits = p[ix] as usize;
                    ix += 1;
                    mask = 0x80;
                }

                if (bits & mask) != 0 {
                    self.src_buf[col] = fg;
                    col += 1;
                } else {
                    self.src_buf[col] = bg;
                    col += 1;
                }
                mask >>= 1;
            }
            self.access.send_words(&self.src_buf[..line_size]); // TODO
        }
    }
}
// EOF
