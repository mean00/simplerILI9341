use super :: Ili9341;


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


}
