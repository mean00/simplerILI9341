use super :: Ili9341;
use crate::glyph::{PFXfont,PFXglyph,FontInfo};



macro_rules! INCLUDE_TEMPLATE {
    ($param_func:expr, $param_prolog:expr, $param_next:expr,  $param_next2:expr) => 
        {
            fn jjjjj(&mut self, w: usize, h : usize, left: usize, line_size: usize, fg: u16, bg : u16, p : *const u8)
                {
                    let mut bits : usize =0;
                    let mut mask : usize =0;
                    let mut col :   usize;
                    
                    $param_prolog;
                    let mut start : *mut u16 = self.src_buf;
                    for line in 0..h // for( int line=h-1;line>=0;line--)
                    {
                        col=left;
                        // mid
                        //for( int xcol=w-1;xcol>=0;xcol--)
                        for xcol in 0..w
                        {
                            if mask==0 // reload ?
                            {
                                bits= $param_next; $param_next2;
                                mask = 0x80;
                            }      
                            let pix : u16;   
                            if (bits & mask)!=0
                            {
                                pix=fg;
                            }else
                            {
                                pix=bg;
                            }
                            *(start.add(col))=pix;
                            col+=1;                            
                            mask>>=1;
                        }                        
                        self.access.send_words(line_size,self._scrbuf);
                    }   
                }                
        };
    }    
//--

impl <'a>Ili9341<'a>
{
    //INCLUDE_TEMPLATE!( innerLoop1 , {}, *p, p=p.add(1));   
    //INCLUDE_TEMPLATE!( innerLoop1C , iliHS hs(p),  hs.next() );

    ///
    /// 
    /// 
    pub fn check_font(& mut self )
    {
        for _i in 0..3
        {
            let info : &mut FontInfo=&mut self.font_infos[_i];
            let mut mW : usize =0;
            let mut mH : isize =0;
                    
            for i in info.font.glyphs
            {
                let mut x: usize =0;
                let mut y: isize =0;
                x=i.x_advance as usize;
                y=(-(i.y_offset as isize)) as isize;
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
                    let first = self.current_font.font.first as usize;
                    let x = x as usize;
                    if (x < first) || (x > (self.current_font.font.last as usize))
                    {
                        return 0;
                    }
                    return self.current_font.font.glyphs[(c as usize)-(first as usize)].x_advance as usize;                    
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
          self.cursor_y +=  self.current_font.font.y_advance as usize;
          return;
        } 
        if c=='\r'
        {
          return ;
        }
        let c: usize = c as usize;
        if (c < self.current_font.font.first as usize) || (c > self.current_font.font.last as usize)
        {
            return ;
        }
        let first : usize = self.current_font.font.first as usize;
        let glyph : &PFXglyph = &self.current_font.font.glyphs[(c as usize)-first];
        let  w = glyph.width as usize;
        let  h = glyph.height as usize;
        
        // also ' ' here
        if (w <= 0) || (h <= 0)
        {
            //
            self.my_square(
                    self.cursor_x,
                    self.cursor_y-self.current_font.max_height, 
                    self.current_font.font.glyphs[0].x_advance as usize,  // advance by the 1st char, not necessarily correct
                    self.current_font.max_height+(glyph.y_offset as usize),
                    self.bg);
            self.cursor_x += glyph.x_advance as usize ;    
            return ;
        }
    
        let xo = glyph.x_offset; // sic
        if (self.cursor_x +  ((xo as usize) + w)) > self.width
        {
          self.cursor_x = 0;
          self.cursor_y +=   self.current_font.font.y_advance as usize;
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
    fn my_draw_char(&mut self,  x: usize, y : usize, c: usize, fg: u16, bg : u16) -> usize
    { 

        let full_c = c;
        let mut c =c;
        let mut y : usize = y;
        c -= self.current_font.font.first as usize;

        let glyph : &PFXglyph = &( self.current_font.font.glyphs[c]);
                               
        let  w: usize    = glyph.width as usize;
        let  mut h: usize    = glyph.height as usize;    
        let  advv: usize = glyph.x_advance as usize +1;
        let  top : usize = (self.current_font.max_height as isize +glyph.y_offset as isize) as usize;
        // Special case
        if full_c==(' ' as usize)
        {
            if(y>=top)
            {
                self.my_square(x,
                            (y-top) as usize,
                            advv, //Fix!
                            self.current_font.max_height+2,bg);
            }
            return advv;
        }       
      
        
        // top & bottom
        if y > self.current_font.max_height
        {
            self.my_square(x,
                    y-self.current_font.max_height,
                    advv,
                    top,bg);
        }
        let bottom: isize =-(glyph.y_offset as isize)-(h as isize);
        if bottom>=-2
        {
            self.my_square(x,((y as isize)-bottom) as usize,advv,(bottom+2) as usize,bg);      
        }
        
        let fg=self.color_map(fg);
        let bg=self.color_map(bg);
    
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
            if h<0
            {
                h=0;
            }
        }

        match self.current_font.font.bpp
        {
            1 =>             {
                    if self.current_font.font.shrinked != 0
                    {
                        //TODO self.innerLoop1C(w,h,left,advv,fg,bg,p);
                    }
                    else
                    {
                        self.innerLoop1NC(w,h,left,advv,fg,bg, &(self.current_font.font.bitmap[ glyph.offset as usize]));
                        //TODO self.innerLoop1NC(w,h,left,advv,fg,bg,p);
                    }
                },
            2 =>   {
                    if self.current_font.font.shrinked != 0
                    {
                        //TODO self.innerLoop2C(w,h,left,advv,fg,bg,p);
                    }
                    else
                    {
                        //TODO self.innerLoop2NC(w,h,left,advv,fg,bg,p);
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
}
// EOF
