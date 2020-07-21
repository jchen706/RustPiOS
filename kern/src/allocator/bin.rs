use core::alloc::Layout;
use core::fmt;
use core::ptr;

use crate::allocator::linked_list::LinkedList;
use crate::allocator::util::*;
use crate::allocator::LocalAlloc;

use crate::console::kprintln;

/// A simple allocator that allocates based on size classes.
///   bin 0 (2^3 bytes)    : handles allocations in (0, 2^3]
///   bin 1 (2^4 bytes)    : handles allocations in (2^3, 2^4]
///   ...5  , 6   , 7 , 8 , 9 , 10
///   bin 29 (2^22 bytes): handles allocations in (2^31, 2^32]
///   
///   map_to_bin(size) -> k
///   


//plan 
// 1. we need to have start and end nodes 
//32 bins // less calculation
// n from 3 to k 
//represent the free memory


//global memory is from memory map
pub struct Allocator {
    list:[LinkedList; 30],


    global_start: usize,
    global_end: usize,
    relative_start: usize,

    //what happens when a bin is exhausted
    total_size:usize


}

impl Allocator {
    /// Creates a new bin allocator that will allocate memory from the region
    /// starting at address `start` and ending at address `end`.
    pub fn new(start: usize, end: usize) -> Allocator {

        let size = end - start;
        
        let mut list1:[LinkedList; 30] = [LinkedList::new(); 30];



       Allocator {
        list: list1,
        global_start: start,
        relative_start: start,
        global_end: end,
        total_size: size

       }
       
        
    }


    pub fn map_to_bin(mut size: usize) -> (usize, usize) {


        let mut count = 0;
        //println!("{:?}", size );

        let mut next_power = size.next_power_of_two();

        while next_power > 0 {
            count+=1;
            next_power = next_power >> 1;
        }

        count = count -1;

        let mut bin_number = 0;

        if count <= 3 {
            bin_number = 0;
        } else {
            bin_number = count - 3;
        }

        //println!("{:?}", count);
        (bin_number, 1<<(count))



    }
}

impl LocalAlloc for Allocator {
    /// Allocates memory. Returns a pointer meeting the size and alignment
    /// properties of `layout.size()` and `layout.align()`.
    ///
    /// If this method returns an `Ok(addr)`, `addr` will be non-null address
    /// pointing to a block of storage suitable for holding an instance of
    /// `layout`. In particular, the block will be at least `layout.size()`
    /// bytes large and will be aligned to `layout.align()`. The returned block
    /// of storage may or may not have its contents initialized or zeroed.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure that `layout.size() > 0` and that
    /// `layout.align()` is a power of two. Parameters not meeting these
    /// conditions may result in undefined behavior.
    ///
    /// # Errors
    ///
    /// Returning null pointer (`core::ptr::null_mut`)
    /// indicates that either memory is exhausted
    /// or `layout` does not meet this allocator's
    /// size or alignment constraints.
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        
        if (self.total_size < layout.size()) {
            return core::ptr::null_mut() as *mut u8;
        }





        //turn layout.size to the closest power of 2

        //determine bin from size:
        //

        //max of layout align and smallest bin possible 



        //size = 256 
        /// alignment is 128

        let mut max_size =  layout.size();

        let mut change_size = false;
        if layout.size() < layout.align() {
             max_size = layout.align();
            // println!("{:?}", "align changed" );
             change_size = true;
        }
        




        if max_size*2 != max_size.next_power_of_two() {
            max_size = max_size.next_power_of_two();
        }

        if max_size < 8  {
            max_size = 8;
        }

        let bin_num = (max_size.trailing_zeros().saturating_sub(3)) as usize;

        let nearest_size = max_size;

        //let (bin_num , nearest_size) = Allocator::map_to_bin(max_size);
        //println!("nearest size{:?}", nearest_size);
        //println!("determine bin {:?}", bin_num);



        // if change_size {
        //     let mut x1 = Allocator::map_to_bin(max_size);

        // } 

        //check if bin is empty

        let is_empty_list = self.list[bin_num].is_empty();

        //println!("{:?}", is_empty_list);


        if is_empty_list {

        //     //if self.relative_start < self.global_start || self.relative_start > self.global_end {
        //     //     return core::ptr::null_mut() as *mut u8;
        //     // } else {



        //         //loop through all the bins
        //         //if the bin not empty merge the size

                

                    //println!("BIN NUMBER {:?}", bin_num);

        //             for i in (bin_num+1)..self.list.len() {

        //                  if !self.list[i].is_empty() {
        //                     let based1:u64 = 2;

        //                     let  size_chunks = based1.pow(i as u32 +3);


        //                 //split it to separate chunk                    }

                                                        
                
     
        //                     let large_address: *mut usize = self.list[i].pop().unwrap();

        //                     //println!(" larger address number {:?}", large_address  as u64);
        //                     let mut new_start_addr = align_up(large_address as usize, layout.align());
        //                     //println!(" new starter address{:?}", new_start_addr );

        //                     let next_start = new_start_addr.saturating_add(nearest_size);

        //                     //let dividedparts = size_chunks / nearest_size as u64;
        //                     let mut new_starter = new_start_addr as u64;
        //                     let mut size_chunks1 = size_chunks - nearest_size as u64;


        //                     let based1:u64 = 2;
        //                    // println!("   Size Chunk {:?}", size_chunks);

        //                     //println!("Real Size : {:?}", nearest_size);

        //                     for j in (3..30).rev() {

        //                         if size_chunks1 <= based1.pow(j as u32) {
        //                             continue;
        //                         } else {
        //                             unsafe{
        //                             self.list[j].push(new_starter as *mut usize);
        //                         }
        //                             //println!("Power of 2: {:?}", j );
        //                             //println!("Ran {:?}", new_starter );

        //                             size_chunks1-= based1.pow(j as u32);
        //                             new_starter += based1.pow(j as u32);


        //                         }
        //                     }

        //                     //println!("{:?}", new_starter);
        //                     //println!("{:?}", size_chunks1);
        //                     //println!("{:?}", self.list);



        //                 //println!("GLOBAL NEXT bin: {0}, nearest size1: {1}, size: {2}, align: {6}, pop relative_start: {3}, start_addr: {4}, next_start: {5}", bin_num, nearest_size, layout.size(), large_address as usize, new_start_addr, next_start, layout.align());

                    




        // //                     // for j in 0.. dividedparts - 1 {



        // //                     //      new_starter += nearest_size;

        // //                     //      self.list[bin_num].push(new_starter as *mut usize);
        // //                     //      //println!("new starter {:?}", new_starter);


        // //                     // }
                            
        // //                     //println!("Size chunks {:?}", size_chunks);
        // //                     //println!("larger bin_number {:?}", i);
        // //                     //println!("subtraction {:?}", size_chunks as usize - nearest_size);




        // //                     //let (next_bin, next_nearest_size1) = Allocator::map_to_bin(size_chunks as usize - nearest_size);

        // //                     //self.list[next_bin].push(next_start as *mut usize);



        //                     return new_start_addr as *mut u8;


                   

        //                 }

                    // }
                
                //loop through the bins
                //loop 



                let mut start_addr = align_up(self.relative_start, layout.align());


                if self.global_end.saturating_sub(nearest_size) < start_addr {
                    return core::ptr::null_mut() as *mut u8;
                }

               

                //if change_align {
                  //  start_addr = align_up(self.relative_start, nearest_size);
                //}

                // if start_addr > self.global_end {
                //    return core::ptr::null_mut() as *mut u8;
                // }


                let padd = start_addr - self.relative_start;

                // let end_addr = start_addr.saturating_add(nearest_size);

                self.relative_start = self.relative_start + nearest_size + padd;

                //kprintln!("GLOBAL: {0}, Return Start Addr: {1},  Size: {2}, Bin:{3}", self.relative_start, start_addr, nearest_size, bin_num);

                //println!("GLOBAL bin: {0}, nearest size: {1}, size: {2}, align: {6}, relative_start: {3}, start_addr: {4}, end_addr: {5}", bin_num, nearest_size, layout.size(), self.relative_start, start_addr, end_addr, layout.align());


                return start_addr as *mut u8;


            //}

        } else {


            //allocation problem here 
            let start_address: *mut usize = self.list[bin_num].pop().unwrap();

            //let start_addr = start_address as usize;

            //kprintln!("BIN: {0}, Return Start Addr: {1},  Size: {2}, Bin: {3}", self.relative_start, start_address as usize, nearest_size, bin_num);


            //println!("POP OFF bin: {0}, nearest size: {1}, size: {2}, align: {5}, relative_start: {3}, start_addr: {4}", bin_num, nearest_size, layout.size(), self.relative_start,start_addr ,layout.align());



            return start_address as *mut u8;






        }

    

    }

    /// Deallocates the memory referenced by `ptr`.
    ///
    /// # Safety
    ///
    /// The _caller_ must ensure the following:
    ///
    ///   * `ptr` must denote a block of memory currently allocated via this
    ///     allocator
    ///   * `layout` must properly represent the original layout used in the
    ///     allocation call that returned `ptr`
    ///
    /// Parameters not meeting these conditions may result in undefined
    /// behavior.
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {


        let mut max_size =  layout.size();
        if layout.size() < layout.align() {
            max_size = layout.align();
        }


        //find max of size and alisgn
        //let (mut bin_num , mut nearest_size) = Allocator::map_to_bin(max_size);
        if max_size*2 != max_size.next_power_of_two() {
            max_size = max_size.next_power_of_two();
        }

        if max_size < 8  {
            max_size = 8;
        }


        let bin_num = (max_size.trailing_zeros().saturating_sub(3)) as usize;
        //kprintln!("PUSH: {0},  Size: {1}, Bin: {2}", ptr as usize, max_size, bin_num);


        //same 
        //merge bins. 

        // let based:u64 = 2;


        // let mut pointer = ptr as usize;


        // // //merge the bins together
        // let mut found = false;
        // for i in 3..self.list.len() {

            // for each in self.list[i].iter_mut() {

        // // //         //each is type linked list node
        // // //         //let x1:() = each;
        //           let size_chunks = based.pow(i as u32);
        // //          println!("{:?}", each.value());
        //           if ptr as usize + nearest_size == each.v`alue() as usize{

        // //              //pop off for merging

        //                 if nearest_size >= based.pow(32 as u32) as usize {
        //                     found = true;
        //                     break
        //                 }
        //                 let large_address: *mut usize = each.pop();


        // //              //get the new size, map to new bin

        //                 let (new_bin,new_size) = Allocator::map_to_bin(nearest_size + size_chunks as usize);

        //                 nearest_size += size_chunks as usize;
        // //              //push the new merged block into the bin

        //                 bin_num = new_bin;
        //                 //pointer = (pointer as usize + size_chunks as usize);
        //             }
        //         }

        //         if found {
        //              break;
        //          }
        //      }



        
         self.list[bin_num].push(ptr as *mut usize);
        





        //          }

       




        //     }




        //  }

        //cases for the lowest level of the address


        //merge the nearest addresses based on bin_size





        //println!("PUSH INTO bin: {0}, nearest size: {1}, size: {2}, align: {4}, relative_start: {3}", bin_num, nearest_size, layout.size(), self.relative_start, layout.align());

        



        //self.list[bin_num].push(ptr as *mut usize);



    }
}

// FIXME: Implement `Debug` for `Allocator`.
impl fmt::Debug for Allocator {
    fn fmt(&self, f: &mut fmt::Formatter<'_> )-> fmt::Result {
        writeln!(f, "Bin Allocator Test")?;

        writeln!(f, "self.global_start: {}", &{self.global_start})?;
        writeln!(f, "self.global_end: {}", &{self.global_end})?;
        writeln!(f, "self.relative_start: {}", &{self.relative_start})?;
        writeln!(f, "self.total_size: {}", &{self.total_size})?;

      




        Ok(())
    }
}