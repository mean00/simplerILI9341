cat text_template1C.txt | sed 's/@IMPORT@//g' | \
  sed 's/@NAME@/inner_loop_1nc/g' | \
  sed 's/@INIT@/let mut ix : usize =0; /g' | \
  sed 's/@NEXT@/ p[ix] as usize;ix+=1;/g' > text_1nc_r.rs
cat text_template2C.txt | sed 's/@IMPORT@//g' | \
  sed 's/@NAME@/inner_loop_2nc/g' | \
  sed 's/@INIT@/let mut ix : usize = 0 ;/g' | \
  sed 's/@NEXT@/ p[ix] as usize; ix+=1;/g' > text_2nc_r.rs

cat text_template1C.txt |  \
        sed 's/@IMPORT@//g' | sed 's/@NAME@/inner_loop_1c/g' | sed 's/@INIT@/self.hs.reset(p);/g' | sed 's/@NEXT@/self.hs.next() as usize;/g' > \
  text_1c_r.rs

cat text_template2C.txt |  \
        sed 's/@IMPORT@//g' | sed 's/@NAME@/inner_loop_2c/g' | sed 's/@INIT@/self.hs.reset(p);/g' | sed 's/@NEXT@/self.hs.next() as usize;/g' > \
  text_2c_r.rs


