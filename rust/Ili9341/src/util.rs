pub fn xabs(x: isize) -> isize
{
    if x < 0         {return -x;}
    x
}
pub fn xswap( a: &mut isize, b : &mut isize)
{
    let z: isize = *a;
    *a=*b;
    *b=z;
}
pub fn xmin(a : isize, b: isize) -> isize
{
    if a< b { return a;}
    b
}