use super :: Ili9341;

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
    pub fn innerLoop2NC(&mut self, w: usize, h : usize, left: usize, line_size: usize, fg: u16, bg : u16, p : *const u8)
    {        
    let fg2= fg as usize;
    let bg2= bg as usize;
    let  low: u16=r(1,3,fg2,bg2)+g(1,3,fg2,bg2)+b(1,3,fg2,bg2);
    let  hi:  u16=r(3,1,fg2,bg2)+g(3,1,fg2,bg2)+b(3,1,fg2,bg2);

    let mut p : *const u8 = p;
    let colorGrad : [u16;4]=[bg,low,hi,fg];
    let mut bits : usize =0;
    let mut rank : isize = -1;
    let mut start : *mut u16 = self.src_buf;
    unsafe {
        start=start.add(left);
        }
    let mut col  : *mut u16;            
    for _line in 0..h // for( int line=h-1;line>=0;line--)
    {             
        col=start;
        // mid
        for _xcol in 0..w          
        {
            if rank<0  // reload ?
            {
                unsafe {
                bits=*p as usize; //@param_next@;
                p=p.add(1);
                }
                rank = 6;
            }      
            let pix=(bits>>rank)&3;
            rank-=2;
            unsafe {
            *col=colorGrad[pix];
            col=col.add(1);
            }
        }
        unsafe {                      
            self.access.send_words(  core::slice::from_raw_parts(self.src_buf, line_size));// TODO
            }
    }   
 }
}