use super::Ili9341;

impl<'a> Ili9341<'a> {

    fn half_circle(&mut self, x: usize, y : usize, radius : usize, color : u16)
    {
        let color = self.access.color_map(color);
        let mut e: isize = 5 - 4 * (radius as isize);
        let mut yy = 0;
        let mut xx = radius;
 
        while yy <= xx {
 
            self.access.set_address(x + xx - 2 * xx, y - yy, 2 * xx, 1);
            self.access.flood_words(2 * xx, color);
 
 
            self.access.set_address(x + yy - 2 * yy, y - xx, 2 * yy, 1);
            self.access.flood_words(2 * yy, color);
 
            Self::circle_advance(&mut xx, &mut yy, &mut e);
        }
    }

    pub fn gauge(&mut self, percent : usize, x: usize, y: usize, radius_internal : usize, radius_external : usize, color: u16) {
       
        self.half_circle(x,y,radius_external,color);
        self.half_circle(x,y,radius_internal,0);

        // -- Draw marker --
        let mut index=percent;
        let mut inverted = false;
        if percent > 50
        {
            index = 100 - percent;
            inverted=true;
        }   
        //---
        let line = crate::ili9341::sin::SIN_TABLE[index] as usize;
        let _line_int=(radius_internal*line+127)>>8;
        let line_ext=(radius_external*line+127)>>8;
        let   sq=radius_external*radius_external-line_ext*line_ext;
        let mut col_ext=0;
        let mut found=false;
        for i in 0..=radius_external
        {
            if i*i >= sq && found==false
            {
                col_ext=i;
                found=true;
            }
        }
        let inner_x = (col_ext*radius_internal+radius_external-1)/radius_external;
        let inner_y = (line_ext*radius_internal+radius_external-1)/radius_external;
        if inverted
        {
            //self.draw_line(x,y,x+col_ext,y-line_ext,crate::colors::BLUE);
            self.draw_line(x+inner_x,y-inner_y,x+col_ext,y-line_ext,crate::colors::RED);
        }
        else
        {
            self.draw_line(x-inner_x,y-inner_y,x-col_ext,y-line_ext,crate::colors::RED);
            //self.draw_line(x,y,x-col_ext,y-line_ext,crate::colors::RED);
        }        
        //---
    }
    pub fn gauge2(&mut self, percent : usize, x: usize, y: usize, radius_internal : usize, radius_external : usize, color: u16) {
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
            self.access.set_address(x + xx_int, y - yy_ext, ww, 1);
            self.access.flood_words(ww, color);
            self.access.set_address(x - xx_ext, y - yy_ext, ww, 1);
            self.access.flood_words(ww, color);
            // Swap X & Y
            self.access.set_address(x + yy_ext, y - xx_ext, 1,ww);
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
        }         
        //---
        let mut index=percent;
        let mut inverted = false;
        if percent > 50
        {
            index = 100 - percent;
            inverted=true;
        }   
        //---
        let line = crate::ili9341::sin::SIN_TABLE[index] as usize;
        let _line_int=(radius_internal*line+127)>>8;
        let line_ext=(radius_external*line+127)>>8;
        let   sq=radius_external*radius_external-line_ext*line_ext;
        let mut col_ext=0;
        let mut found=false;
        for i in 0..=radius_external
        {
            if i*i >= sq && found==false
            {
                col_ext=i;
                found=true;
            }
        }
        if inverted
        {
            self.draw_line(x,y,x+col_ext,y-line_ext,crate::colors::BLUE);
        }
        else
        {
            self.draw_line(x,y,x-col_ext,y-line_ext,crate::colors::RED);
        }        
        //---
    }
}
