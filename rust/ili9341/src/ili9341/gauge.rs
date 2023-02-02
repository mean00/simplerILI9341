use super::Ili9341;
use crate::util::xabs;
use crate::util::xmin;
use crate::util::xswap;

impl<'a> Ili9341<'a> {

    pub fn gauge(&mut self, x: usize, y: usize, radius_internal : usize, radius_external : usize, color: u16) {
        // https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
        let color = self.access.color_map(color);
        let mut e_int: isize = 5 - 4 * (radius_internal as isize);
        let mut e_ext: isize = 5 - 4 * (radius_external as isize);

        let mut yy_int = 0;
        let mut xx_int = radius_internal;
        let mut yy_ext = 0;
        
        let mut xx_ext = radius_external;
        while yy_int < xx_int {
            Self::circle_advance(&mut xx_ext, &mut yy_ext, &mut e_ext);
            Self::circle_advance(&mut xx_int, &mut yy_int, &mut e_int);
            let ww=xx_ext-xx_int;
            self.access.set_address(x + xx_int, y - yy_int, ww, 1);
            self.access.flood_words(ww, color);
            self.access.set_address(x - xx_ext, y - yy_ext, ww, 1);
            self.access.flood_words(ww, color);
            // Swap X & Y
            self.access.set_address(x + yy_int, y - xx_ext, 1,ww);
            self.access.flood_words(ww, color);
            self.access.set_address(x - yy_ext , y - xx_ext, 1,ww);
            self.access.flood_words(ww, color);
            
        } 
        // now do the ext size only
        let half = (radius_internal*181)>>8; // 1/sqrt(2)
        while yy_ext < xx_ext {
            Self::circle_advance(&mut xx_ext, &mut yy_ext, &mut e_ext);
            let ww=xx_ext-half;
            let hh=yy_ext-half;

            self.access.set_address(x + xx_ext-ww, y - yy_ext,  ww,1);
            self.access.flood_words(ww, color);
            self.access.set_address(x - xx_ext, y - yy_ext,  ww, 1);
            self.access.flood_words(ww, color);

            self.access.set_address(x + yy_ext-hh, y - xx_ext,  hh,1);
            self.access.flood_words(hh, color);
            self.access.set_address(x - yy_ext, y - xx_ext,  hh, 1);
            self.access.flood_words(hh, color);


            /*
            self.access.flood_words(ww, color);
            self.access.set_address(x - xx_ext, y - yy_ext, ww, 1);
            self.access.flood_words(ww, color);
            */
        }            
    }
}
