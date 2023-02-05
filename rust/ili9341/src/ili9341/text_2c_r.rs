use super :: Ili9341;
#[cfg(feature = "sim")]
extern crate std;

#[inline(always)]
fn c(x : usize, shift : usize, bits: usize) -> usize
{
    (x>>shift)&bits
}
#[inline(always)]
fn d(x : usize, shift : usize) -> usize
{
    x<<shift
}
#[inline(always)]
fn r( l:usize, r: usize, fg: usize, bg : usize) -> u16
{
    d((c(fg,0,0x1f)*l+c(bg,0,0x1f)*r) / (l+r),0) as u16    
}
#[inline(always)]
fn g( l:usize, r: usize, fg: usize, bg : usize) -> u16
{
    d((c(fg,5,0x3f)*l+c(bg,5,0x3f)*r) / (l+r),5) as u16
}
#[inline(always)]
fn b( l:usize, r: usize, fg: usize, bg : usize) -> u16
{
    d((c(fg,11,0x1f)*l+c(bg,11,0x1f)*r) / (l+r),11) as u16
}


impl <'a>Ili9341<'a>
{
    pub fn inner_loop_2c(&mut self, w: usize, h : usize, left: usize, line_size: usize, fg: u16, bg : u16, p: &'a [u8])
    {        
    let fg2= fg as usize;
    let bg2= bg as usize;
    let  low: u16=r(1,3,fg2,bg2)+g(1,3,fg2,bg2)+b(1,3,fg2,bg2);
    let  hi:  u16=r(3,1,fg2,bg2)+g(3,1,fg2,bg2)+b(3,1,fg2,bg2);

    self.hs.reset(p);
    let color_grad : [u16;4]=[bg,low,hi,fg];
    let mut bits : usize =0;
    let mut rank : isize = -1;
    let start : usize =left;
    let mut col  : usize;            
    for _line in 0..h // for( int line=h-1;line>=0;line--)
    {             
        col=start;
        // mid
        for _xcol in 0..w          
        {
            if rank<0  // reload ?
            {
                bits=self.hs.next() as usize;
                rank = 6;
            }      
            let pix=(bits>>rank)&3;
            #[cfg(feature = "sim")]
            match pix
            {
                0 =>  std::print!("."),
                1 =>  std::print!("-"),
                2 =>  std::print!("+"),
                3 =>  std::print!("#"),
                _ =>  (),
            }
            rank-=2;
            self.src_buf[col]=color_grad[pix];
            col+=1;
            
        }
        #[cfg(feature = "sim")]
        std::print!("\n");
        self.access.send_words( &self.src_buf[..line_size]);// TODO        
    }   
 }
}
