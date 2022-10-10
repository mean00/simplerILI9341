
use super :: Ili9341;

impl <'a>Ili9341<'a>
{
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
        let mut x0= x0;
        let mut x1= x1;
        let mut y0= y0;
        let mut y1= y1;
        let mut z:usize;
        if y0>y1
        {
            z=y0;y0=y1;y1=z;            
        }
        if x0>x1
        {
            z=x0;x0=x1;x1=z;        
        }
    
        if x0==x1  { self.vline(x0,y0,y1-y0+1,color);return;}
        if y0==y1  { self.hline(x0,y0,x1-x0+1,color);return;}
        let color=self.color_map(color);
        let dx=( x1   - x0   +1);        
        let dy=( y1  - y0   +1);
        if dx<dy
        {
                let inc=(dy*2048)/dx;
                let mut val: usize=0;
                let mut posy : usize =y0;
                for x in x0..=x1
                {
                    val+=inc;
                    let h : usize=val>>11;
                    val&=(1<<11)-1;
                    self.access.set_address(x,posy,1,h);
                    self.access.flood_words(h,color);                
                    posy+=h;                    
                }
                return;
        }
        let inc: usize=(dx*2048)/dy;
        let mut val : usize=0;
        let mut posx : usize=x0;
        for y in y0..=y1
        {
            val+=inc;
            let h: usize =val>>11;
            self.access.set_address(posx,y,h,1);
            self.access.flood_words(h,color);
            posx+=h;
            val&=(1<<11)-1;
        }    
    }
}