use super :: Ili9341;

impl <'a>Ili9341<'a>
{
  pub fn innerLoop1NC(&mut self, w: usize, h : usize, left: usize, line_size: usize, fg: u16, bg : u16, p: &'a [u8])
  {
      let mut bits : usize =0;
      let mut mask : usize =0;
      let mut col  : *mut u16;
      let mut p : *const u8 = p.as_ptr();
      let mut start : *mut u16 = self.src_buf;
      unsafe {
      start=start.add(left);
      }
      for _line in 0..h // for( int line=h-1;line>=0;line--)
      {
          col=start;
          // mid
          //for( int xcol=w-1;xcol>=0;xcol--)
          for _xcol in 0..w
          {
            unsafe {
              if mask==0 // reload ?
              {
                  bits=*p as usize;p=p.add(1);
                  mask = 0x80;
              }      
                              
              if (bits & mask)!=0
              {
                  *col=fg;
                  col=col.add(1);
              }else
              {
                  *col=bg;
                  col=col.add(1);
              }
            }
              mask>>=1;
          }  
          unsafe {                      
            self.access.send_words(  core::slice::from_raw_parts(self.src_buf, line_size));// TODO
          }
      }   
  }                
}
// EOF
