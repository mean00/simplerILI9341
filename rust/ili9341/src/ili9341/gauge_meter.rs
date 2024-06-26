/*
    This may look complicated but we have to deal with the following constraints :
    1- we want to write each pixel *ONCE*, else it creates flickering
    2- we dont want to compute using float
    3- we dont want to precompute too much to avoid using all the ram


*/
use crate::ili9341::sin::SIN_TABLE;
use crate::ili9341::Ili9341;

pub struct Gauge<'a> {
    radius_internal: usize,
    radius_external: usize,
    yext: &'a mut [u8],
    yint: &'a mut [u8],
    buffer: &'a mut [u16],
}
enum Area {
    Empty,
    Full,
    Partial,
}

fn area_revert(i: Area) -> Area {
    return match i {
        Area::Empty => Area::Full,
        Area::Full => Area::Empty,
        Area::Partial => Area::Partial,
    };
}

impl<'a> Gauge<'a> {
    //
    pub fn new(radius_internal: usize, radius_external: usize) -> Self {
        let mut r = Gauge {
            radius_internal,
            radius_external,
            yext: crate::util::unsafe_slice_alloc::<u8>(radius_external + 1),
            yint: crate::util::unsafe_slice_alloc::<u8>(radius_external + 1),
            buffer: crate::util::unsafe_slice_alloc::<u16>(2 * radius_external + 1),
        };
        r._init();
        r
    }
    //
    pub fn _init(&mut self) {
        let mut e: isize = 5 - 4 * (self.radius_external as isize);
        let mut yy = 0;
        let mut xx = self.radius_external;
        // -- ext --
        while yy < xx {
            self.yext[yy] = xx as u8;
            self.yext[xx] = yy as u8;
            crate::ili9341::Ili9341::circle_advance(&mut xx, &mut yy, &mut e);
        }
        for i in 0..=self.radius_external {
            self.yint[i] = 0;
        }
        // -- int --
        e = 5 - 4 * (self.radius_internal as isize);
        yy = 0;
        xx = self.radius_internal;
        // -- ext --
        while yy < xx {
            self.yint[yy] = xx as u8;
            self.yint[xx] = yy as u8;
            crate::ili9341::Ili9341::circle_advance(&mut xx, &mut yy, &mut e);
        }
    }

    fn pen_size(&self, array: &[u8], indx: usize) -> usize {
        let pen: usize;
        if array[indx] > array[indx + 1] {
            pen = (array[indx] - array[indx + 1]) as usize;
        } else {
            pen = 1;
        }
        pen
    }
    //
    fn fill_x(&mut self, start: usize, pen: usize, color: u16) {
        let mut dex = start;
        for _k in 0..pen {
            self.buffer[dex] = color;
            dex = dex + 1;
        }
    }
    fn fill_antix(&mut self, start: usize, pen: usize, color: u16) {
        let mut dex = start;
        for _k in 0..pen {
            self.buffer[2 * self.radius_external - dex] = color;
            dex = dex + 1;
        }
    }
    //
    fn draw_elem(
        &mut self,
        color: u16,
        line: usize,
        left: Area,
        right: Area,
        column: usize,
        len_left: usize,
        len_right: usize,
    ) {
        let pen_ext = self.pen_size(&self.yext, line);
        let pen_int = self.pen_size(&self.yint, line);
        match left {
            Area::Full => {
                self.fill_x(column, len_left + 1, color);
            }
            Area::Empty => {
                self.fill_x(column, pen_ext, color);
                // inner
                if self.yint[line] != 0 {
                    self.fill_x(column + len_left, pen_int, color);
                }
            }
            Area::Partial => {
                self.fill_x(column, len_left + 1, color);
                if self.yint[line] != 0 {
                    self.fill_x(column + len_right, pen_int, color); // WRONG
                }
            }
        };
        match right {
            Area::Full => {
                self.fill_antix(column, len_right + 1, color);
            }
            Area::Empty => {
                self.fill_antix(column, pen_ext, color);
                // inner
                if self.yint[line] != 0 {
                    self.fill_antix(column + len_right, pen_int, color);
                }
            }
            Area::Partial => {
                if len_right > len_left {
                    self.fill_antix(column + len_left, len_right - len_left + 1, color);
                } else {
                    self.fill_antix(column + len_right, len_left - len_right + 1, color);
                }
                self.fill_antix(column, pen_ext, color); // WRONG
                                                         //self.buffer[ 2*self.radius_external  -column] = color;
            }
        }
    }

    pub fn draw(&mut self, percent: usize, ili: &mut Ili9341, x: usize, y: usize, color: u16) {
        // Check the interieur start of filled
        let over: bool;
        let index: usize = match percent {
            0..=50 => {
                over = false;
                percent
            }
            51..=100 => {
                over = true;
                100 - percent
            }
            _ => {
                over = true;
                0
            }
        };

        let line = SIN_TABLE[index] as usize;
        let col = SIN_TABLE[50 - index] as usize;
        let line_int = (self.radius_internal * line + 127) >> 8;
        let line_ext = (self.radius_external * line + 127) >> 8;
        let col_int = (self.radius_internal * col + 127) >> 8;
        let col_ext = (self.radius_external * col + 127) >> 8;

        let mut left: Area;
        let mut right: Area;

        let mut koeff: usize = 0;
        if line_ext != line_int {
            koeff = col_ext - col_int;
            let den = line_ext - line_int;
            koeff = ((256 * koeff) + den / 2) / den;
        }

        //--------------Start to fill ------------------------
        for i in 0..self.radius_external {
            let xext = self.yext[i] as usize;
            let xint = self.yint[i] as usize;
            let w: usize;
            if xext > xint {
                w = xext - xint;
            } else {
                w = 1;
            }
            let xext = self.radius_external - xext;
            let _xint = self.radius_external - xint;

            for i in 0..self.buffer.len() {
                self.buffer[i] = 0;
            }

            let ev: Area;
            let mut wleft: usize = w;
            let mut wright: usize = w;
            let mut adj = w;
            let abs_pos: usize;
            if i <= line_int {
                ev = Area::Empty;
            } else if i > line_ext {
                ev = Area::Full;
            } else {
                ev = Area::Partial;
                let alpha = (koeff * (i - line_int)) >> 8;
                let abs_x = self.radius_external - xext;
                abs_pos = col_int + alpha;
                if abs_x > abs_pos {
                    adj = abs_x - abs_pos;
                    adj = crate::util::xmaxu(crate::util::xminu(adj, w), 1);
                } else {
                    adj = 1;
                }
            }
            if over {
                left = Area::Full;
                right = ev;
                wright = adj;
            } else {
                left = area_revert(ev);
                right = Area::Empty;
                wleft = adj;
            }
            self.draw_elem(
                ili.access.color_map(color),
                i,
                left,
                right,
                xext,
                wleft,
                wright,
            );
            ili.send_data(x - self.radius_external, y - i, &self.buffer);
        }
        let last_line: usize = self.yext[self.radius_external] as usize;
        ili.hline(
            x - last_line,
            y - self.radius_external,
            2 * last_line,
            color,
        );
        let first_line: usize = self.yext[0] as usize;
        let first_ww: usize = (self.yext[0] - self.yint[0]) as usize;
        ili.hline(x - first_line + 1, y, first_ww, color);
        ili.hline(x + first_line - first_ww, y, first_ww, color);
        /*
        if over
        {
            ili.draw_line(x+col_int, y-line_int, x+col_ext, y-line_ext, crate::colors::BLUE) ;
        }else
        {
            ili.draw_line(x-col_int, y-line_int, x-col_ext, y-line_ext, crate::colors::BLUE) ;
        }
        */
    }
}
