
use crate::ili9341::Ili9341;
use crate::ili9341::sin::SIN_TABLE;
pub struct Gauge <'a>
{    
    radius_internal : usize,
    radius_external : usize,
    yext            : &'a mut [u8],
    yint            : &'a mut [u8],
    buffer          : &'a mut [u16],
}

impl <'a> Gauge <'a>
{
    //
    pub fn new (radius_internal : usize, radius_external : usize) -> Self
    {
        let ex = crate::util::unsafe_array_alloc::<u8>(radius_external+1);
        let ix = crate::util::unsafe_array_alloc::<u8>(radius_external+1);
        let bf = crate::util::unsafe_array_alloc::<u16>(radius_external*2+1);
        Gauge{                
                    radius_internal,
                    radius_external,
                    yext :  unsafe {core::slice::from_raw_parts_mut(ex,radius_external+1)},
                    yint :  unsafe {core::slice::from_raw_parts_mut(ix,radius_external+1)},
                    buffer: unsafe {core::slice::from_raw_parts_mut(bf,2*radius_external+1)},
                   
        }
    }
    //
    pub fn init(&mut self)
    {
        let mut e: isize = 5 - 4 * (self.radius_external as isize);
        let mut yy = 0;
        let mut xx = self.radius_external;
        // -- ext --
        while yy < xx {            
            self.yext[yy]=xx as u8;
            self.yext[xx]=yy as u8;
            crate::ili9341::Ili9341::circle_advance(&mut xx, &mut yy, &mut e);
            
        }
        for i in 0..=self.radius_external
        {
            self.yint[i]=0;            
        }
        // -- int --
        e = 5 - 4 * (self.radius_internal as isize);
        yy = 0;
        xx = self.radius_internal;
        // -- ext --
        while yy < xx {                        
            self.yint[yy]=xx as u8;
            self.yint[xx]=yy as u8;
            crate::ili9341::Ili9341::circle_advance(&mut xx, &mut yy, &mut e);
            
        }        

    }
    // 
    fn draw_elem(&mut self, color : u16, line: usize, left: bool, right : bool, column: usize, len : usize)
    {
       if(left)
       {        
            let mut dex=column;
            for _k in 0..=len
            {                
                self.buffer[dex]=color;
                dex+=1;
            }
       }else
       {

       }
       if(right)
       {
        let mut dex=column;
        for _k in 0..=len
        {                                        
            self.buffer[self.radius_external*2-dex]=color;
            dex+=1;
        }
       }
       else
       {

       }
    }

    pub fn draw(&mut self, percent : usize, ili : &mut Ili9341, x : usize, y : usize, color : u16)
    {
        let last_line: usize = self.yext[self.radius_external] as usize;
        ili.hline(x-last_line,y-self.radius_external,2*last_line,color);
        let first_line: usize = self.yext[0] as usize;
        let first_ww: usize = (self.yext[0]-self.yint[0]) as usize;
        ili.hline(x-first_line,y,first_ww,color);
        ili.hline(x+first_line-first_ww-1,y,first_ww,color);

        // Check the interieur start of filled
        let mut over: bool =false;
        let index: usize;
        if percent > 50
        {
            over=true;
            index=100-percent;
        }else
        {
            over=false;
            index=percent;
        }
        

        let line = SIN_TABLE[index] as usize;
        let _line_int=(self.radius_internal*line+127)>>8;
        let line_ext=(self.radius_external*line+127)>>8; 
        let mut left : bool;
        let mut right : bool;

        //-- Start to fill ...
        for i in 0..self.radius_external        
        {
            let xext=self.yext[i] as usize;
            let xint=self.yint[i] as usize;
            let w=xext-xint;
            let xext=self.radius_external-xext;
            let xint=self.radius_external-xint;
            for i in 0..self.buffer.len()
            {
                self.buffer[i]=0;
            }
            if over == true
            {
                left=true;
                if i>line_ext
                {
                    right= true  ;               
                }else
                {
                    right= false;                
                }            

            }
            else
            {
                right=false;
                if i<line_ext
                {
                    left=true;               
                }else
                {
                    left=false;                
                }            
            }
            self.draw_elem(  color, i,  left, right, xext,w);
            ili.send_data( x-self.radius_external,y-i,self.buffer);
        
        /*
            else
            {
               
                let mut pen = (self.yext[i]-self.yext[i+1]) as usize;
                if pen==0
                {
                    pen=1;
                }
                let mut dex=xext;
                // outer..
                for _k in 0..pen
                {
                    self.buffer[dex]=color;
                    self.buffer[self.radius_external*2-dex]=color;
                    dex=dex+1;
                }
                // inner
                if self.yint[i]!=0
                {
                    let mut pen = (self.yint[i]-self.yint[i+1]) as usize;
                    if pen==0
                    {
                        pen=1;
                    }
                    let mut dex=xint;                    
                    for _k in 0..pen
                    {
                        self.buffer[dex]=color;
                        self.buffer[self.radius_external*2-dex]=color;
                        dex=dex+1;
                    }
                }
            }
            */
            
        }        
    }
    pub fn draw2(&mut self, ili : &mut Ili9341, x : usize, y : usize, color : u16, empty : bool)
    {
        let last_line: usize = self.yext[self.radius_external] as usize;
        ili.hline(x-last_line,y-self.radius_external,2*last_line,color);
        let first_line: usize = self.yext[0] as usize;
        let first_ww: usize = (self.yext[0]-self.yint[0]) as usize;
        ili.hline(x-first_line,y,first_ww,color);
        ili.hline(x+first_line-first_ww-1,y,first_ww,color);
        if empty
        {
            for i in 0..self.radius_external
            {
                let xext=self.yext[i] as usize;
                let xext_next=self.yext[i+1] as usize;                
                let w = xext-xext_next+1;
                
                let xint=self.yint[i] as usize;
                let xint_next=self.yint[i+1] as usize;
                let w_int=xint-xint_next+1;

                ili.hline(x-xext,y-i,w,color);
                ili.hline(x+xext-w-1,y-i,w,color);

                if i<=(self.radius_internal+1)
                {
                    ili.hline(x-xint,y-i,w_int,color);
                    ili.hline(x+xint-w_int-1,y-i,w_int,color);                                            
                }               
            }
        }else {
            for i in 0..self.radius_external
            {
                let xext=self.yext[i] as usize;
                let xint=self.yint[i] as usize;
                let w = xext-xint;
                ili.hline(x-xext,y-i,w,color);
                ili.hline(x+xint,y-i,w,color);
            }
        }
    }

}