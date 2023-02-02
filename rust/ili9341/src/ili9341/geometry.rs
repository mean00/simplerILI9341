use super::Ili9341;
use crate::util::xabs;
use crate::util::xmin;
use crate::util::xswap;

impl<'a> Ili9341<'a> {
    pub fn vline(&mut self, x0: usize, y0: usize, h: usize, color: u16) {
        let color = self.access.color_map(color);
        self.access.set_address(x0, y0, 1, h);
        self.access.data_begin();
        self.access.flood_words(h, color);
        self.access.data_end();
    }
    pub fn hline(&mut self, x0: usize, y0: usize, w: usize, color: u16) {
        let color = self.access.color_map(color);
        self.access.set_address(x0, y0, w, 1);
        self.access.data_begin();
        self.access.flood_words(w, color);
        self.access.data_end();
    }   
    pub fn circle(&mut self, x: usize, y: usize, radius: usize, color: u16) {
        // https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
        let color = self.access.color_map(color);
        let mut e: isize = 5 - 4 * (radius as isize);
        let mut yy = 0;
        let mut xx = radius;
        while yy < xx {
            Self::circle_advance(&mut xx, &mut yy, &mut e);
            // Use simple symetry
            self.access.set_address(x + xx, y + yy, 1, 1);
            self.access.flood_words(1, color);
            self.access.set_address(x - xx, y + yy, 1, 1);
            self.access.flood_words(1, color);
            self.access.set_address(x - xx, y - yy, 1, 1);
            self.access.flood_words(1, color);
            self.access.set_address(x + xx, y - yy, 1, 1);
            self.access.flood_words(1, color);
            // Use 45 degrees symetry, swapping x & y
            self.access.set_address(x + yy, y + xx, 1, 1);
            self.access.flood_words(1, color);
            self.access.set_address(x - yy, y + xx, 1, 1);
            self.access.flood_words(1, color);
            self.access.set_address(x - yy, y - xx, 1, 1);
            self.access.flood_words(1, color);
            self.access.set_address(x + yy, y - xx, 1, 1);
            self.access.flood_words(1, color);
        }
    }
    pub fn disc(&mut self, x: usize, y: usize, radius: usize, color: u16) {
        // https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
        let color = self.access.color_map(color);
        let mut e: isize = 5 - 4 * (radius as isize);
        let mut yy = 0;
        let mut xx = radius;

        while yy <= xx {
            self.access.set_address(x + xx - 2 * xx, y + yy, 2 * xx, 1);
            self.access.flood_words(2 * xx, color);

            self.access.set_address(x + xx - 2 * xx, y - yy, 2 * xx, 1);
            self.access.flood_words(2 * xx, color);

            self.access.set_address(x + yy - 2 * yy, y + xx, 2 * yy, 1);
            self.access.flood_words(2 * yy, color);

            self.access.set_address(x + yy - 2 * yy, y - xx, 2 * yy, 1);
            self.access.flood_words(2 * yy, color);

            Self::circle_advance(&mut xx, &mut yy, &mut e);
        }
    }

    /**
     *
     */
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: u16) {
        let mut x0: isize = x0 as isize;
        let mut x1: isize = x1 as isize;
        let mut y0: isize = y0 as isize;
        let mut y1: isize = y1 as isize;

        let adx = (xabs(x1 - x0) as usize) + 1;
        let ady = (xabs(y1 - y0) as usize) + 1;

        if x0 == x1 {
            self.vline(x0 as usize, xmin(y0, y1) as usize, ady, color);
            return;
        }
        if y0 == y1 {
            self.hline(xmin(x0, x1) as usize, y0 as usize, adx, color);
            return;
        }

        let color = self.access.color_map(color);

        if adx < ady {
            let inc = (ady * 2048) / adx;
            let mut val: usize = 0;
            let mut inv: bool = false;
            if x0 > x1 {
                xswap(&mut x0, &mut x1);
                xswap(&mut y0, &mut y1);
            }
            if y0 > y1 {
                inv = true;
            }
            let mut posy: usize = y0 as usize;
            for x in (x0 as usize)..=(x1 as usize) {
                val += inc;
                let h: usize = val >> 11;
                val &= (1 << 11) - 1;
                self.access.set_address(x, posy, 1, h);
                self.access.data_begin();
                self.access.flood_words(h, color);
                self.access.data_end();
                if inv {
                    posy -= h;
                } else {
                    posy += h;
                }
            }
            return;
        }
        let inc: usize = (adx * 2048) / ady;
        let mut val: usize = 0;

        let mut inv: bool = false;
        if y0 > y1 {
            xswap(&mut x0, &mut x1);
            xswap(&mut y0, &mut y1);
        }
        if x0 > x1 {
            inv = true;
        }
        let mut posx: usize = x0 as usize;
        for y in (y0 as usize)..=(y1 as usize) {
            val += inc;
            let h: usize = val >> 11;
            val &= (1 << 11) - 1;
            self.access.set_address(posx, y, h, 1);
            self.access.data_begin();
            self.access.flood_words(h, color);
            self.access.data_end();
            if inv {
                posx -= h;
            } else {
                posx += h;
            }
        }
    }
    /**
     *
     */
    pub fn inverted_disc_corner(
        &mut self,
        x: usize,
        y: usize,
        radius: usize,
        corner: usize,
        color: u16,
    ) {
        const OFS: usize = 1;
        // https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
        let color = self.access.color_map(color);
        let mut e: isize = 5 - 4 * (radius as isize);
        let mut yy = 0;
        let mut xx = radius;
        while yy <= xx {
            if (corner & 2) != 0 {
                self.access
                    .set_address(x + xx, y - yy + radius - 1, radius, 1);
                self.access.flood_words(radius - xx, color);
                self.access
                    .set_address(x + yy, y - xx + radius - 1, radius, 1);
                self.access.flood_words(radius - yy, color);
            }
            if (corner & 1) != 0 {
                self.access.set_address(x, radius + y - yy, radius, 1);
                self.access.flood_words(radius - xx, color);
                self.access.set_address(x, radius + y - xx, radius, 1);
                self.access.flood_words(radius - yy, color);
            }

            if (corner & 4) != 0 {
                self.access.set_address(x, y + yy - OFS, radius, 1);
                self.access.flood_words(radius - xx, color);
                self.access.set_address(x, y + xx - OFS, radius, 1);
                self.access.flood_words(radius - yy, color);
            }
            if (corner & 8) != 0 {
                self.access.set_address(x + xx, y + yy - OFS, radius, 1);
                self.access.flood_words(radius - xx, color);
                self.access.set_address(x + yy, y + xx - OFS, radius, 1);
                self.access.flood_words(radius - yy, color);
            }
            Self::circle_advance(&mut xx, &mut yy, &mut e);
        }
    }
    pub fn quarter_disc(
        &mut self,
        mx: usize,
        radius: usize,
        preload: usize,
        out: &mut [u16],
    ) -> usize {
        let mut e: isize = 5 - 4 * (radius as isize);
        let mut yy = 0;
        let mut xx = radius as isize;
        if preload * 2 >= radius {
            return 0;
        }
        for _i in 0..preload {
            if e > 0 {
                xx -= 1;
                e = e - 8 * xx;
            }
            yy += 1;
            e += 8 * yy + 4;
        }
        let mut nb: usize = 0;
        for _i in preload..radius {
            out[nb] = xx as u16;
            nb += 1;
            if nb * 2 >= radius {
                return nb;
            }
            if nb >= mx {
                return nb;
            }
            if e > 0 {
                xx -= 1;
                e -= 8 * xx;
            }
            yy += 1;
            e += 8 * yy + 4;
        }
        return nb;
    }
    //
    pub fn fill_round_rect(
        &mut self,
        x0: usize,
        y0: usize,
        w: usize,
        h: usize,
        radius: usize,
        outcolor: u16,
        incolor: u16,
    ) {
        let outcolor = self.access.color_map(outcolor);
        let incolor = self.access.color_map(incolor);
        let mut e: isize = 5 - 4 * (radius as isize);
        let mut yy = 0;
        let mut xx = radius - 1;

        let mut last: usize = 1 << 31;
        let _colors: [u16; 3] = [outcolor, incolor, outcolor];
        let mut sizes: [usize; 3] = [0, 0, 0];

        //#define LINE(dest)     {setAddress(x0,y0+dest,w,1); multiFloodWords(3,sizes,colors);}
        macro_rules! LINE {
        ( $( $x:expr ),* ) =>
            {
            {
                $(
                    self.access.set_address(x0,y0+$x,w,1);
                    self.access.multi_flood_words(&sizes,&_colors);
                    //temp_vec.push($x);
                )*
            }
            };
        }
        while yy <= xx {
            sizes[0] = radius - xx;
            sizes[2] = sizes[0];
            sizes[1] = w - 2 * sizes[0];

            let invy = radius - yy - 1;
            LINE!(invy);
            LINE!(h - invy - 1);

            sizes[0] = radius - yy;
            sizes[2] = sizes[0];
            sizes[1] = w - 2 * sizes[0];
            let invx = radius - xx - 1;
            if invx != last {
                LINE!(invx);
                LINE!(h - invx - 1);
            }
            last = invx;
            Self::circle_advance(&mut xx, &mut yy, &mut e);
        }
        // interior now
        self.access.set_address(x0, y0 + radius, w, h - 2 * radius);
        self.access.flood_words(w * (h + 2 - 2 * radius), incolor);
    }
}
