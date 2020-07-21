/// Align `addr` downwards to the nearest multiple of `align`.
///
/// The returned usize is always <= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2.
pub fn align_down(addr: usize, align: usize) -> usize {

   if (((align!=0) && ((align & (align - 1))==0)) != true)  {
    	panic!("{:?}", "Align is not a power of 2");
    }  else {
    	let integer = addr % align;
    	let new_addr = addr - integer;
    	new_addr
    }
}

/// Align `addr` upwards to the nearest multiple of `align`.
///
/// The returned `usize` is always >= `addr.`
///
/// # Panics
///
/// Panics if `align` is not a power of 2
/// or aligning up overflows the address.
pub fn align_up(addr: usize, align: usize) -> usize {
    if (((align!=0) && ((align & (align - 1))==0)) != true)  {
    	panic!("{:?}", "Align is not a power of 2");
    } else {
    	let integer = (addr % align);
    	if integer == 0 {
    		return addr;
    	} else {

    		let sum = addr + (align - integer);
    		if (sum < addr) {
    			panic!("{:?}", "Aligning Overflows the Address");
    		}
    		sum
    	}
    }
}