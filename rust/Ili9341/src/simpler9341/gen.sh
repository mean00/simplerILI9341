cat text_template1C.txt | sed 's/@NAME@/innerLoop1NC/g' | sed 's/@INIT@/let mut p : *const u8 = p;/g' | sed 's/@NEXT@/*p as usize;p=p.add(1);/g' > text_1NC_r.rs
cat text_template1C.txt | sed 's/@NAME@/innerLoop1C/g' | sed 's/@INIT@/let mut p : *const u8 = p;/g' | sed 's/@NEXT@/*p as usize;p=p.add(1);/g' > text_1C_r.rs
cat text_template2C.txt | sed 's/@NAME@/innerLoop2NC/g' | sed 's/@INIT@/let mut p : *const u8 = p;/g' | sed 's/@NEXT@/*p as usize;p=p.add(1);/g' > text_2NC_r.rs
cat text_template2C.txt | sed 's/@NAME@/innerLoop2C/g' | sed 's/@INIT@/let mut p : *const u8 = p;/g' | sed 's/@NEXT@/*p as usize;p=p.add(1);/g' > text_2C_r.rs
