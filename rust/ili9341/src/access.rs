//
pub trait Ili9341Access {
    fn get_chip_id(&mut self) -> u16 {
        return 0x9341;
    }
    fn color_map(&self, d: u16) -> u16 {
        return d;
    }
    fn send_byte(&mut self, b: u8);
    fn send_word(&mut self, b: u16);
    fn send_bytes(&mut self, data: &[u8]) {
        let n = data.len();
        for i in 0..n {
            self.send_byte(data[i]);
        }
    }
    fn send_words(&mut self, data: &[u16]) {
        let n = data.len();
        for i in 0..n {
            self.send_word(data[i]);
        }
    }
    fn update_hw_rotation(&mut self, rotation: usize);
    fn flood_words(&mut self, nb: usize, color: u16) {
        self.data_begin();
        for _i in 0..nb {
            self.send_word(color);
        }
        self.data_end();
    }
    fn set_address(&mut self, x: usize, y: usize, w: usize, h: usize);
    fn data_end(&mut self);
    fn data_begin(&mut self);
    fn push_colors(&mut self, colors: &[u16]) {
        let n = colors.len();
        for i in 0..n {
            self.send_word(colors[i]);
        }
    }
    fn multi_flood_words(&mut self, sizes: &[usize], data: &[u16]) {
        let n = sizes.len();
        if n != data.len() {
            panic!("oops");
        }
        for i in 0..n {
            self.flood_words(sizes[i], data[i]);
        }
    }
}
