cat text_template1C.txt | sed 's/@IMPORT@//g' | \
  sed 's/@NAME@/innerLoop1NC/g' | \
  sed 's/@INIT@/let mut ix : usize =0; /g' | \
  sed 's/@NEXT@/ p[ix] as usize;ix+=1;/g' > text_1NC_r.rs
cat text_template2C.txt | sed 's/@IMPORT@//g' | \
  sed 's/@NAME@/innerLoop2NC/g' | \
  sed 's/@INIT@/let mut ix : usize = 0 ;/g' | \
  sed 's/@NEXT@/ p[ix] as usize; ix+=1;/g' > text_2NC_r.rs

cat text_template1C.txt |  \
        sed 's/@IMPORT@//g' | sed 's/@NAME@/innerLoop1C/g' | sed 's/@INIT@/self.hs.reset(p);/g' | sed 's/@NEXT@/self.hs.next() as usize;/g' > \
  text_1C_r.rs

cat text_template2C.txt |  \
        sed 's/@IMPORT@//g' | sed 's/@NAME@/innerLoop2C/g' | sed 's/@INIT@/self.hs.reset(p);/g' | sed 's/@NEXT@/self.hs.next() as usize;/g' > \
  text_2C_r.rs


