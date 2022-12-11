use super :: Ili9341;
use simpler_gfx::{ PFXglyph, FontInfo};
use super ::FontFamily;
//--

impl <'a>Ili9341<'a>
{   
    pub fn set_font_size (&mut self, f : FontFamily)
    {
        self.current_font_index = f;
    }

    ///
    pub fn set_cursor(&mut self,  x: usize,  y : usize )
    {        
        self.cursor_x= x;
        self.cursor_y= y;
    }
     /// 
    /// 
    pub fn check_font(& mut self )
    {
        for _i in 0..3
        {
            let info : &mut FontInfo=&mut self.font_infos[_i];
            if info.font.shrinked!=0
            {
                if info.font.hs_conf !=0x74
                {
                    panic!("HSCONF");
                }
            }
            let mut mW : usize =0;
            let mut mH : isize =0;
                    
            for i in info.font.glyphs
            {
                let x : usize = i.x_advance as usize;
                let y : isize =(-(i.y_offset as isize)) as isize;
                if x>mW {mW=x;}
                if y>mH {mH=y;}
            }
            info.max_height=(mH as usize) + 1;
            info.max_width=mW;    
        }
    }
    ///
    /// 
    /// 
    fn string_length(&mut self, text : &str) -> usize
    {
        let mut width : usize =0;
        for c in text.chars()
        {
            width+=match c
            {
                '\n' | '\r' => 0,
                x => {
                    let first = self.current_font().font.first as usize;
                    let x = x as usize;
                    if (x < first) || (x > (self.current_font().font.last as usize))
                    {
                        return 0;
                    }
                    return self.current_font().font.glyphs[(c as usize)-(first as usize)].x_advance as usize;                    
                },
            }
        }
        width
    }
    ///
    /// 
    /// 
    fn my_square(&mut self,  x : usize, y : usize, w: usize, h:usize, filler : u16)
    {
        let mut w:usize = w;
        let mut h:usize = h;

        if (w+x)>=self.width
        {    
            w=self.width-x;    
            if w<=0
            {
                 return ;
            }
        }    
        if (h+y)>=self.height
        {    
            h=self.height-y;
            if h<=0
            {
                return;
            }
        }    
        
        self.access.set_address(x,y,w,h);
        self.access.data_begin();
        self.access.flood_words(w*h,filler);
        self.access.data_end();
        return ;
    }
    ///
    /// 
    /// 
    pub fn write_char(&mut self, c: char) -> ()
    {
        if  c == '\n'
        {
          self.cursor_x = 0;
          self.cursor_y +=  self.current_font().font.y_advance as usize;
          return;
        } 
        if c=='\r'
        {
          return ;
        }
        let c: usize = c as usize;
        if (c < self.current_font().font.first as usize) || (c > self.current_font().font.last as usize)
        {
            return ;
        }
        let first : usize = self.current_font().font.first as usize;
        let glyph : &PFXglyph = &self.current_font().font.glyphs[(c as usize)-first];
        let  w = glyph.width as usize;
        let  h = glyph.height as usize;
        
        // also ' ' here
        if (w <= 0) || (h <= 0)
        {
            //
            if self.cursor_y>self.current_font().max_height
            {
                self.my_square(
                        self.cursor_x,
                        self.cursor_y-self.current_font().max_height, 
                        self.current_font().font.glyphs[0].x_advance as usize,  // advance by the 1st char, not necessarily correct
                        self.current_font().max_height+(glyph.y_offset as usize),
                        self.bg);
            }
            self.cursor_x += glyph.x_advance as usize ;    
            return ;
        }
    
        let xo = glyph.x_offset; // sic
        if (self.cursor_x +  ((xo as usize) + w)) > self.width
        {
          self.cursor_x = 0;
          self.cursor_y +=   self.current_font().font.y_advance as usize;
        }    
        
        self.cursor_x += self.my_draw_char(
                    self.cursor_x, 
                    self.cursor_y, 
                    c, 
                    self.fg,
                    self.bg);
    }
    ///
    /// 
    /// 
    pub fn set_text_color(&mut self, f : u16, b: u16)
    {
        self.fg = f;
        self.bg = b;
    }
    ///
    /// 
    /// 
    fn my_draw_char(&mut self,  x: usize, y : usize, c: usize, fg: u16, bg : u16) -> usize
    { 
        let full_c = c;
        let mut c =c;
        let mut y : usize = y;
        c -= self.current_font().font.first as usize;

        let glyph : &PFXglyph = &( self.current_font().font.glyphs[c]);
                               
        let  w: usize    = glyph.width as usize;
        let  mut h: usize    = glyph.height as usize;    
        let  advv: usize = glyph.x_advance as usize +1;
        let  top : usize = (self.current_font().max_height as isize +glyph.y_offset as isize) as usize;
        // Special case
        if full_c==(' ' as usize)
        {
            if y>=top
            {
                self.my_square(x,
                            (y-top) as usize,
                            advv, //Fix!
                            self.current_font().max_height+2,bg);
            }
            return advv;
        }       
      
        
        // top & bottom
        if y > self.current_font().max_height
        {
            self.my_square(x,
                    y-self.current_font().max_height,
                    advv,
                    top,bg);
        }
        let bottom: isize =-(glyph.y_offset as isize)-(h as isize);
        if bottom>=-2
        {
            self.my_square(x,((y as isize)-bottom) as usize,advv,(bottom+2) as usize,bg);      
        }
        
        let fg=self.access.color_map(fg);
        let bg=self.access.color_map(bg);
    
        // offset is <0 most of the time
        let mut tmpy : isize = y as isize;
        tmpy+= glyph.y_offset as isize;   
        if tmpy<0
        {
            return glyph.x_advance as usize;
        }
        y = tmpy as usize;

        let left: usize =glyph.x_offset as usize;
        let mut right: isize =(advv as isize)-(w as isize + (left as isize));
        if right<0
        {
            right=0;
        } 
        
        self.access.set_address(x,y, advv, h);                
        // Pre-fill & left /right
        for i in 0..left
        {
            unsafe { *(self.src_buf.add(i))=bg;} // TODO speedup
        }            
        let right_border = left+w+right as usize;
        for i in (w+left)..right_border
        {
            unsafe { *(self.src_buf.add(i))=bg; } // TODO speedup            
        }
        // fill in body
        
        self.access.data_begin();
    
        // dont draw out of screen 
        if y+h>=self.height
        {
            h= self.height-y;           
        }
        let glyph_data : &[u8] = &(self.current_font().font.bitmap[ (glyph.offset as usize)..]);
        match self.current_font().font.bpp
        {
            1 =>             {
                    if self.current_font().font.shrinked != 0
                    {
                        self.innerLoop1C(w,h,left,advv,fg,bg,glyph_data );                        
                    }
                    else
                    {
                        self.innerLoop1NC(w,h,left,advv,fg,bg, glyph_data );                        
                    }
                },
            2 =>   {
                    if self.current_font().font.shrinked != 0
                    {
                        self.innerLoop2C(w,h,left,advv,fg,bg,glyph_data);      
                    }
                    else
                    {
                        self.innerLoop2NC(w,h,left,advv,fg,bg,glyph_data);
                    }
                },
            _ => panic!("Crap"),
            }
        self.access.data_end();
        return glyph.x_advance as usize;
    }
    ///
    /// 
    /// 
    pub fn print(&mut self, x : usize, y : usize, text  : &str)
    {
        self.cursor_x=x;
        self.cursor_y=y;
        for c in text.chars()
        {
            self.write_char(c);
            self.cursor_x +=1;
        }
    }   
    pub fn print_up_to(&mut self, text  : &str, max_width_in_pixels : usize)
    {
        let last_pos=self.cursor_x + max_width_in_pixels;
        let index = self.current_font_index as usize;
        let s= text.len() as usize;
        for i in 0..s
        {
            if self.cursor_x>=last_pos
            {
                return;
            }
            let c: char = text.as_bytes()[i] as char;
            self.write_char(c);
        }
        
        if last_pos>self.cursor_x // fill in
        {
            let w  = last_pos-self.cursor_x;            
            let max_height = self.font_infos[index].max_height;
            self.my_square(self.cursor_x, self.cursor_y -max_height ,  w, max_height+2, self.bg);
        }
    }
}
// EOF
