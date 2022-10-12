
use super :: Ili9341;

impl <'a>Ili9341<'a>
{
    fn myabs(x: isize) -> isize
    {
        if x < 0         {return -x;}
        x
    }
    fn swap( a: &mut isize, b : &mut isize)
    {
        let z: isize = *a;
        *a=*b;
        *b=z;
    }
    fn min(a : isize, b: isize) -> isize
    {
        if a< b { return a;}
        b
    }
    pub fn vline(&mut self,x0:usize, y0: usize, h:usize, color : u16)
    {
        let color=self.color_map(color);
        self.access.set_address(x0,y0,1,h);
        self.access.flood_words(h,color);
    }
    pub fn hline(&mut self,x0:usize, y0: usize, w:usize, color : u16)
    {
        let color=self.color_map(color);
        self.access.set_address(x0,y0,w,1);
        self.access.flood_words(w,color);
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
        let mut z : isize;
       
       
        let adx = (Ili9341::myabs( x1  - x0 ) as usize)+1;
        let ady = (Ili9341::myabs( y1  - y0 ) as usize)+1;
    
        if x0==x1  { self.vline(x0 as usize, Ili9341::min(y0,y1) as usize ,ady, color);return;}
        if y0==y1  { self.hline(Ili9341::min(x0,x1) as usize, y0 as usize ,adx, color);return;}
       
        let color=self.color_map(color);

        if adx < ady
        {
            let inc=(ady*2048)/adx;
            let mut val: usize=0;
            let mut inv: bool =false;
            if x0>x1
            {
                Self::swap(&mut x0,&mut x1);
                Self::swap(&mut y0,&mut y1);
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
                self.access.flood_words(h,color);
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
            Self::swap(&mut x0,&mut x1);
            Self::swap(&mut y0,&mut y1);
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
            self.access.flood_words(h,color);            
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