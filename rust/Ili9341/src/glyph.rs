



// single glyph structure
#[repr(C)]
struct PFXglyph{
    bitmapOffset: u16,  //< Pointer into GFXfont->bitmap
    width       : u8,   //< Bitmap dimensions in pixels
    height      : u8,   //< Bitmap dimensions in pixels
    xAdvance    : u8,   //< Distance to advance cursor (x axis)
    xOffset     : i8,   //< X dist from cursor pos to UL corner
    yOffset     : i8,   //< Y dist from cursor pos to UL corner
} 
// Whole font
#[repr(C)]
struct PFXfont
{
  bitmap      : *const u8,        //< Glyph bitmaps, concatenated
  glyph       : *const PFXglyph,  //< Glyph array
  first       : u16,              //< ASCII extents (first char)
  last        : u16,              //< ASCII extents (last char)
  yAdvance    : u8,               //< Newline distance (y axis)
  bpp         : u8 ,              //< bit per pixel, 1 or 4 at the moment
  shrinked    : u8,               //< compressed ?
}

extern {
  fn load_font_info(data : &[u8], font : &mut PFXfont) -> bool;
}

