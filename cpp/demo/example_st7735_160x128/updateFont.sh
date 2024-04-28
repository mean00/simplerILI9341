flatconvert-rs -f ../font/OpenSans-Regular.ttf            -o demofont1.h -s 18  -p2  
cat demofont1.h | sed 's/OpenSans_Regular18pt7b/demofont/g' > demofont.h

