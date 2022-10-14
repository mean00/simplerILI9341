
  fn @param_name@(&mut self, w: usize, h : usize, left: usize, line_size: usize, fg: u16, bg : u16, p : *const u8)
  {
      let mut bits : usize =0;
      let mut mask : usize =0;
      let mut col  : *mut u16;
      
      @param_prolog@;
      let mut start : *mut u16 = self.src_buf;
      
      start+=left;
      for line in 0..h // for( int line=h-1;line>=0;line--)
      {
          col=start;
          // mid
          //for( int xcol=w-1;xcol>=0;xcol--)
          for xcol in 0..w
          {
              if mask==0 // reload ?
              {
                  bits=@param_next@;
                  mask = 0x80;
              }      
                              
              if (bits & mask)!=0
              {
                  *col=fg;
                  col+=1;
              }else
              {
                  *col=bg;
                  col+=1;
              }
              mask>>=1;
          }                        
          self.access.send_words(line_size,self._scrbuf);
      }   
  }                
};