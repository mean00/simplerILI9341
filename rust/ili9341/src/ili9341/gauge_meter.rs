/*
    This may look complicated but we have to deal with the following constraints :
    1- we want to write each pixel *ONCE*, else it creates flickering
    2- we dont want to compute using float
    3- we dont want to precompute too much to avoid using all the ram


    xext and xint contain the column of external/internal circle for each y

*/
use crate::ili9341::sin::SIN_TABLE;
use crate::ili9341::Ili9341;

pub struct Gauge<'a> {
    radius_internal: usize,
    radius_external: usize,
    xext: &'a mut [u8],
    xint: &'a mut [u8],
    buffer: &'a mut [u16],
    old_percent: usize,
    not_first: bool,
    old_line_int: usize,
    old_line_ext: usize,
    old_color: u16,
}
#[derive(PartialEq)]
enum Area {
    Empty,
    Full,
    Partial,
    Skip,
}

fn area_revert(i: Area) -> Area {
    return match i {
        Area::Empty => Area::Full,
        Area::Full => Area::Empty,
        Area::Partial => Area::Partial,
        Area::Skip => Area::Skip,
    };
}

impl<'a> Gauge<'a> {
    //
    pub fn new(radius_internal: usize, radius_external: usize) -> Self {
        let mut r = Gauge {
            radius_internal,
            radius_external,
            xext: crate::util::unsafe_slice_alloc::<u8>(radius_external + 1),
            xint: crate::util::unsafe_slice_alloc::<u8>(radius_external + 1),
            buffer: crate::util::unsafe_slice_alloc::<u16>(2 * radius_external + 1),
            old_percent: 101,
            not_first: false,
            old_line_int: 0,
            old_line_ext: 0,
            old_color: 0,
        };
        r.init();
        r
    }
    //
    // Bresenham method
    // compute the y for each x
    // (half circle)
    fn compute_quarter_circle(radius: usize, y: &mut [u8]) {
        let mut e: isize = 5 - 4 * radius as isize;
        let mut yy = 0;
        let mut xx = radius;
        // -- ext --
        while yy < xx {
            y[yy] = xx as u8;
            y[xx] = yy as u8;
            crate::ili9341::Ili9341::circle_advance(&mut xx, &mut yy, &mut e);
        }
    }
    //
    fn init(&mut self) {
        Self::compute_quarter_circle(self.radius_external, &mut self.xext);
        for i in 0..=self.radius_external {
            self.xint[i] = 0;
        }
        Self::compute_quarter_circle(self.radius_internal, &mut self.xint);
    }
    //
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
    fn fast_fill(array: &mut [u16], color: u16) {
        array.iter_mut().for_each(|m| *m = color)
    }
    //
    fn fill_x(&mut self, start: usize, pen: usize, color: u16) {
        Self::fast_fill(&mut self.buffer[start..(start + pen)], color)
    }
    // fill a line backward
    fn fill_antix(&mut self, start: usize, pen: usize, color: u16) {
        let dex = 2 * self.radius_external - start - (pen - 1);
        Self::fast_fill(&mut self.buffer[dex..(dex + pen)], color)
    }
    /*
     * Draw an horizontal  line of the circle(s)
     */
    fn draw_simple_right_elem(
        &mut self,
        right: &Area,
        color: u16,
        line: usize,
        column: usize,
        len_right: usize,
    ) {
        let half: usize = self.radius_external;
        Self::fast_fill(&mut self.buffer[half..], 0);
        match right {
            Area::Full => {
                self.fill_antix(column, len_right + 1, color);
            }
            Area::Empty => {
                let pen_ext = self.pen_size(&self.xext, line);
                let pen_int = self.pen_size(&self.xint, line);
                self.fill_antix(column, pen_ext, color);
                // inner
                if self.xint[line] != 0 {
                    self.fill_antix(column + len_right, pen_int, color);
                }
            }
            _ => {
                panic!("oops right")
            }
        }
    }

    fn draw_simple_left_elem(
        &mut self,
        left: &Area,
        color: u16,
        line: usize,
        column: usize,
        len_left: usize,
    ) {
        let half: usize = self.radius_external;
        Self::fast_fill(&mut self.buffer[0..half], 0);
        match left {
            Area::Full => {
                self.fill_x(column, len_left + 1, color);
            }
            Area::Empty => {
                let pen_ext = self.pen_size(&self.xext, line);
                let pen_int = self.pen_size(&self.xint, line);
                self.fill_x(column, pen_ext, color);
                // inner
                if self.xint[line] != 0 {
                    self.fill_x(column + len_left, pen_int, color);
                }
            }
            _ => {
                panic!("oops left")
            }
        };
    }

    /**
     *
     *
     */
    fn compute_partial(
        &mut self,
        koeff: usize,
        line: usize,
        line_int: usize,
        full_width: usize,
        start_x: usize,
    ) -> usize {
        // compute partial in case we use it
        let mut adj: usize;
        let alpha = (koeff * (line - line_int)) >> 8;
        let abs_x = self.xext[line] as usize;
        let abs_pos: usize = start_x + alpha;
        if abs_x > abs_pos {
            adj = abs_x - abs_pos;
            adj = crate::util::xmaxu(crate::util::xminu(adj, full_width), 1);
        } else {
            adj = 1;
        }
        adj
    }

    /*
     *
     */
    pub fn draw(&mut self, percent: usize, ili: &mut Ili9341, x: usize, y: usize, color: u16) {
        // if nothing changed, do nothing
        let mut percent = percent;
        if percent > 100 {
            percent = 100;
        }
        if self.old_percent == percent && self.not_first && self.old_color == color {
            return;
        }
        // Check if the partial fill is in the left side or right side
        let over: bool;
        let index: usize;
        if percent > 50 {
            over = true;
            index = 100 - percent;
        } else {
            over = false;
            index = percent;
        }
        let old_over: bool;
        if self.old_percent >= 50 {
            old_over = true;
        } else {
            old_over = false
        }
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
        let scan_from: usize;
        let scan_to: usize;
        // just redraw the part that has changed
        if color == self.old_color && self.not_first {
            scan_from = crate::util::xmaxu(1, crate::util::xminu(self.old_line_int, line_int));
            scan_to = crate::util::xminu(
                self.radius_external,
                crate::util::xmaxu(self.old_line_ext, line_ext) + 1,
            );
        } else {
            scan_from = 1;
            scan_to = self.radius_external;
        }
        //--------------Start to fill ------------------------
        for i in scan_from..scan_to {
            let mut xext = self.xext[i] as usize;
            let xint = self.xint[i] as usize;
            let w: usize;
            if xext > xint {
                w = xext - xint;
            } else {
                w = 1;
            }
            // invert
            xext = self.radius_external - xext;

            let mut wleft: usize = w;
            let mut wright: usize = w;
            let half_width = self.buffer.len() >> 1;

            // more than 50%, left side is full
            if over {
                if false && color == self.old_color && self.not_first && old_over {
                    left = Area::Skip; // already drawn
                } else {
                    left = Area::Full;
                    self.draw_simple_left_elem(&left, color, i, xext, wleft);
                }
                if i < line_int {
                    right = Area::Empty;
                    self.draw_simple_right_elem(&right, color, i, xext, wright);
                } else if i > line_ext {
                    right = Area::Full;
                    self.draw_simple_right_elem(&right, color, i, xext, wright);
                } else {
                    // first draw an empty one
                    right = Area::Full;
                    self.draw_simple_right_elem(&right, color, i, xext, wright);
                    // overwrite with the partially filled info
                    right = Area::Partial;
                    wright = self.compute_partial(koeff, i, line_int, w, col_int);
                    self.fill_antix(xext, wright, 0);
                    let pen_ext = self.pen_size(&self.xext, i);
                    self.fill_antix(xext, pen_ext, color);
                }
            } else {
                // left than 50%, right side is empty
                if false && color == self.old_color && self.not_first && !old_over {
                    right = Area::Skip; // already drawn
                } else {
                    right = Area::Empty;
                    self.draw_simple_right_elem(&right, color, i, xext, wright);
                }
                if i < line_int {
                    //
                    left = Area::Full;
                    self.draw_simple_left_elem(&left, color, i, xext, wleft);
                } else if i > line_ext {
                    left = Area::Empty;
                    self.draw_simple_left_elem(&left, color, i, xext, wleft);
                } else {
                    // start with an empty one
                    left = Area::Empty;
                    self.draw_simple_left_elem(&left, color, i, xext, wleft);
                    // then complete
                    left = Area::Partial;
                    wleft = self.compute_partial(koeff, i, line_int, w, col_int);
                    self.fill_x(xext, wleft, color);
                }
            }
            if left != Area::Skip && right != Area::Skip {
                // draw both
                ili.send_data(x - self.radius_external, y - i, &self.buffer);
            } else {
                if left != Area::Skip {
                    // left only
                    ili.send_data(x - self.radius_external, y - i, &self.buffer[0..half_width]);
                }
                if right != Area::Skip {
                    // right only
                    ili.send_data(
                        x, //- (self.radius_external >> 1),
                        y - i,
                        &self.buffer[half_width..],
                    );
                }
            }
        }
        let last_line: usize = self.xext[self.radius_external] as usize;
        ili.hline(
            x - last_line,
            y - self.radius_external,
            2 * last_line,
            color,
        );
        let first_ww = self.radius_external - self.radius_internal;
        let first_x = self.radius_external;

        ili.hline(x - first_x, y, first_ww, color);
        ili.hline(x + first_x - first_ww, y, first_ww, color);
        self.old_percent = percent;
        self.not_first = true;
        self.old_line_int = line_int;
        self.old_line_ext = line_ext;
        self.old_color = color;
    }
}
//
