



// single glyph structure
#[repr(C)]
pub struct PFXglyph{
    pub offset      : u16,  //< Pointer into GFXfont->bitmap
    pub width       : u8,   //< Bitmap dimensions in pixels
    pub height      : u8,   //< Bitmap dimensions in pixels
    pub x_advance   : u8,   //< Distance to advance cursor (x axis)
    pub x_offset    : i8,   //< X dist from cursor pos to UL corner
    pub y_offset    : i8,   //< Y dist from cursor pos to UL corner
} 
#[repr(C)]
pub struct PFXfont
{
  pub bitmap      : &'static[u8],        //< Glyph bitmaps, concatenated
  pub glyphs      : &'static[PFXglyph],  //< Glyph array
  pub first       : u16,              //< ASCII extents (first char)
  pub last        : u16,              //< ASCII extents (last char)
  pub y_advance   : u8,               //< Newline distance (y axis)
  pub bpp         : u8 ,              //< bit per pixel, 1 or 4 at the moment
  pub shrinked    : u8,               //< compressed ?
}

extern {
  fn load_font_info(data : &[u8], font : &mut PFXfont) -> bool;
}

