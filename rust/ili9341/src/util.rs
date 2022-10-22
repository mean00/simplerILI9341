use alloc::alloc::Layout as Layout;
use alloc::alloc::alloc as alloc;
//use alloc::alloc::dealloc as dealloc;


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

//
//https://stackoverflow.com/questions/59232877/how-to-allocate-structs-on-the-heap-without-taking-up-space-on-the-stack-in-stab

pub fn unsafe_box_allocate<T>() ->  *mut T
{
    
    let layout = Layout::new::<T>();
    unsafe {           
        let ptr = alloc(layout) as *mut T;             
        ptr
    }
}
/*
pub fn unsafe_box_deallocate<T>(instance : &mut T)
{        
    let layout = Layout::new::<T>();
    unsafe {        
        //let ptr : *mut u8= instance.as_mut() as *mut u8;
        //dealloc(ptr,layout);
    }
}
*/
//-----------
pub fn unsafe_array_alloc<T>(count : usize ) -> *mut T 
{
    
        let layout = Layout::new::<T>();
        let unit = layout.size();
        let align = layout.align();
    unsafe {  
        let big_layout = Layout::from_size_align_unchecked(unit * count, align);

            
        let ptr = alloc(big_layout) as *mut T;     
        ptr
    }    
}