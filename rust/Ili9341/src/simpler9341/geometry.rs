
use super :: Ili9341;
use crate::util::xswap as xswap;
use crate::util::xmin as xmin;
use crate::util::xabs as xabs;

impl <'a>Ili9341<'a>
{
   
    pub fn vline(&mut self,x0:usize, y0: usize, h:usize, color : u16)
    {
        
        let color=self.color_map(color);
        self.access.set_address(x0,y0,1,h);
        self.access.data_begin();
        self.access.flood_words(h,color);
        self.access.data_end();
    }
    pub fn hline(&mut self,x0:usize, y0: usize, w:usize, color : u16)
    {
        let color=self.color_map(color);
        self.access.set_address(x0,y0,w,1);
        self.access.data_begin();
        self.access.flood_words(w,color);
        self.access.data_end();
    }
    fn circle_advance( xx : &mut usize, yy : &mut usize, E : &mut isize)
    {
        if (*E)>0
        {
            (*xx)-=1;
            *E=(*E)-8*(*xx as isize);
        }
        (*yy)+=1;
        (*E)+=8*(*yy as isize)+4;
    }
    pub fn circle(&mut self, x: usize, y: usize, radius: usize, color: u16)    
    {
        // https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
        let color=self.color_map(color);
        let mut E: isize=5-4*(radius as isize);
        let mut yy=0;
        let mut xx=radius;
        while yy<xx
        {
            Self::circle_advance(&mut xx,&mut yy,&mut E);
            // Use simple symetry
            self.access.set_address(x+xx,y+yy,1,1);
            self.access.flood_words(1,color);
            self.access.set_address(x-xx,y+yy,1,1);
            self.access.flood_words(1,color);
            self.access.set_address(x-xx,y-yy,1,1);
            self.access.flood_words(1,color);
            self.access.set_address(x+xx,y-yy,1,1);
            self.access.flood_words(1,color);
            // Use 45 degrees symetry, swapping x & y
            self.access.set_address(x+yy,y+xx,1,1);
            self.access.flood_words(1,color);
            self.access.set_address(x-yy,y+xx,1,1);
            self.access.flood_words(1,color);
            self.access.set_address(x-yy,y-xx,1,1);
            self.access.flood_words(1,color);
            self.access.set_address(x+yy,y-xx,1,1);
            self.access.flood_words(1,color);
        }

    }
    pub fn disc(&mut self, x: usize, y: usize, radius: usize, color: u16)    
    {
        // https://bariweiss.substack.com/p/hollywoods-new-rules?s=r
        let color=self.color_map(color);
        let mut E: isize=5-4*(radius as isize);
        let mut yy=0;
        let mut xx=radius;
        
        while yy<=xx
        {
    
            self.access.set_address(x+xx-2*xx,y+yy,2*xx,1);
            self.access.flood_words(2*xx,color);
    
            self.access.set_address(x+xx-2*xx,y-yy,2*xx,1);
            self.access.flood_words(2*xx,color);       

            self.access.set_address(x+yy-2*yy,y+xx,2*yy,1);
            self.access.flood_words(2*yy,color);

            self.access.set_address(x+yy-2*yy,y-xx,2*yy,1);
            self.access.flood_words(2*yy,color);       

            Self::circle_advance(&mut xx,&mut yy,&mut E);
        }
    }

    /**
     * 
     */
    pub fn draw_line(&mut self, x0:usize, y0: usize, x1:usize, y1:usize, color: u16)
    {
        let mut x0: isize= x0 as isize;
        let mut x1: isize= x1 as isize;
        let mut y0: isize= y0 as isize;
        let mut y1: isize= y1 as isize;
        
       
       
        let adx = (xabs( x1  - x0 ) as usize)+1;
        let ady = (xabs( y1  - y0 ) as usize)+1;
    
        if x0==x1  { self.vline(x0 as usize, xmin(y0,y1) as usize ,ady, color);return;}
        if y0==y1  { self.hline(xmin(x0,x1) as usize, y0 as usize ,adx, color);return;}
       
        let color=self.color_map(color);

        if adx < ady
        {
            let inc=(ady*2048)/adx;
            let mut val: usize=0;
            let mut inv: bool =false;
            if x0>x1
            {
                xswap(&mut x0,&mut x1);
                xswap(&mut y0,&mut y1);
            }
            if y0 > y1
            {
                inv = true;
            }
            let mut posy : usize =y0 as usize;
            for x  in (x0 as usize)..=(x1 as usize)
            {
                val+=inc;
                let h : usize=val>>11;
                val&=(1<<11)-1;
                self.access.set_address(x,posy,1,h);
                self.access.data_begin();
                self.access.flood_words(h,color);
                self.access.data_end();
                if inv
                {
                    posy-=h;
                }else 
                {
                    posy+=h;
                }           
            }
            return;
        }
        let inc: usize=(adx*2048)/ady;
        let mut val : usize=0;
        
        let mut inv : bool =false;
        if y0>y1
        {
            xswap(&mut x0,&mut x1);
            xswap(&mut y0,&mut y1);
        }
        if x0 > x1
        {
            inv = true;
        }
        let mut posx: usize=x0 as usize;
        for y in (y0 as usize)..=(y1 as usize)
        {
            val+=inc;
            let h: usize =val>>11;
            val&=(1<<11)-1;
            self.access.set_address(posx,y,h,1);
            self.access.data_begin();
            self.access.flood_words(h,color);            
            self.access.data_end();
            if inv
            {
                posx-=h;
            }else 
            {
                posx+=h;
            }
        }    
    }
}