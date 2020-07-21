use core::iter::Chain;
use core::ops::{Deref, DerefMut};
use core::slice::Iter;

use alloc::boxed::Box;
use alloc::fmt;
use core::alloc::{GlobalAlloc, Layout};

use crate::allocator;
use crate::param::*;
use crate::vm::{PhysicalAddr, VirtualAddr};
use crate::ALLOCATOR;

use aarch64::vmsa::*;
use shim::const_assert_size;
use core::fmt::write;

#[repr(C)]
pub struct Page([u8; PAGE_SIZE]);
const_assert_size!(Page, PAGE_SIZE);

impl Page {
    pub const SIZE: usize = PAGE_SIZE;
    pub const ALIGN: usize = PAGE_SIZE;

    fn layout() -> Layout {
        unsafe { Layout::from_size_align_unchecked(Self::SIZE, Self::ALIGN) }
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct L2PageTable {
    pub entries: [RawL2Entry; 8192],
}
const_assert_size!(L2PageTable, PAGE_SIZE);

impl L2PageTable {
    /// Returns a new `L2PageTable`
    fn new() -> L2PageTable {


        let entry = [RawL2Entry::new(0);8192];



        L2PageTable {

            entries: entry,

        }



    }

    /// Returns a `PhysicalAddr` of the pagetable.
    pub fn as_ptr(&self) -> PhysicalAddr {
        PhysicalAddr::from(self as *const L2PageTable)
        //unimplemented!("L2PageTable::as_ptr()")
    }
}

#[derive(Copy, Clone)]
pub struct L3Entry(RawL3Entry);

impl L3Entry {
    /// Returns a new `L3Entry`.
    fn new() -> L3Entry {
        //unimplemented!("L3Entry::new()")


        L3Entry(RawL3Entry::new(0))



    }

    /// Returns `true` if the L3Entry is valid and `false` otherwise.
    fn is_valid(&self) -> bool {
        //unimplemented!("L3Entry::is_valid()")
        self.0.get_value(RawL3Entry::VALID)==EntryValid::Valid
    
    }

    /// Extracts `ADDR` field of the L3Entry and returns as a `PhysicalAddr`
    /// if valid. Otherwise, return `None`.
    fn get_page_addr(&self) -> Option<PhysicalAddr> {
        //unimplemented!("LeEntry::get_page_add()")

        if self.is_valid() {
            Some(PhysicalAddr::from(self.0.get_masked(RawL3Entry::ADDR)))
        } else {
            None
        }
    }

      
}

#[repr(C)]
#[repr(align(65536))]
pub struct L3PageTable {
    pub entries: [L3Entry; 8192],
}
const_assert_size!(L3PageTable, PAGE_SIZE);

impl L3PageTable {
    /// Returns a new `L3PageTable`.
    fn new() -> L3PageTable {
        //unimplemented!("L3PageTable::new()")
        L3PageTable {
            entries: [L3Entry::new(); 8192]
        }
    }

    /// Returns a `PhysicalAddr` of the pagetable.
    pub fn as_ptr(&self) -> PhysicalAddr {
        //unimplemented!("L3PageTable::as_ptr()")
        PhysicalAddr::from(self as *const L3PageTable)
    }
}

#[repr(C)]
#[repr(align(65536))]
pub struct PageTable {
    pub l2: L2PageTable,
    pub l3: [L3PageTable; 2],
}

impl PageTable {
    /// Returns a new `Box` containing `PageTable`.
    /// Entries in L2PageTable should be initialized properly before return.
    fn new(perm: u64) -> Box<PageTable> {
        //unimplemented!("PageTable::new()")

        let mut pt = Box::new(
            PageTable {
                l2:  L2PageTable::new(),
                l3: [L3PageTable::new(), L3PageTable::new()],
            });


        pt.l2.entries[0].set_masked(pt.l3[0].as_ptr().as_u64(), RawL2Entry::ADDR);
        pt.l2.entries[1].set_masked(pt.l3[1].as_ptr().as_u64(), RawL2Entry::ADDR);

        pt.l2.entries[0].set_value(EntryType::Table, RawL2Entry::TYPE);
        pt.l2.entries[0].set_value(EntryValid::Valid, RawL2Entry::VALID);
        pt.l2.entries[0].set_value(perm, RawL2Entry::AP);

        pt.l2.entries[1].set_value(EntryType::Table, RawL2Entry::TYPE);
        pt.l2.entries[1].set_value(EntryValid::Valid, RawL2Entry::VALID);
        pt.l2.entries[1].set_value(perm, RawL2Entry::AP);



        pt


    }

    /// Returns the (L2index, L3index) extracted from the given virtual address.
    /// Since we are only supporting 1GB virtual memory in this system, L2index
    /// should be smaller than 2.
    ///
    /// # Panics
    ///
    /// Panics if the virtual address is not properly aligned to page size.
    /// Panics if extracted L2index exceeds the number of L3PageTable.
    fn locate(va: VirtualAddr) -> (usize, usize) {
        //unimplemented!("PageTable::localte()")


        //check if va is ttbr1 or 0
        //index into l2
        // round the  bits 0 to the page size as a multiple
         if va.as_usize()!=(va.as_usize() & !(Page::SIZE-1)) {
            panic!("{:?}", "VirtualAddress is not align with page size");
         }

         //bit 29 l2 address
         let l2_i = ((va.as_usize() >> 29 as usize ) & 0b1) as usize;

         if l2_i > 1 {
            panic!("{:?}", "L2 Index exeeds the number of L3PageTable" );
         }


         let l3_i = (va.as_usize() & ((0x1FFF as usize) << 16 as usize)) >> 16;

        (l2_i, l3_i)

        //[28:16] = va into l3 table


    }

    /// Returns `true` if the L3entry indicated by the given virtual address is valid.
    /// Otherwise, `false` is returned.
    pub fn is_valid(&self, va: VirtualAddr) -> bool {
        //unimplemented!("PageTable::is_valid()")
        let (l2_index, l3_index) = PageTable::locate(va);
        let l2_entry = self.l2.entries[l2_index];

        let l3_table = l2_entry.get_masked(RawL2Entry::ADDR) as usize;

        if self.l3[0].as_ptr().as_usize() == l3_table {
            return self.l3[0].entries[l3_index].is_valid()
        } else {
            return self.l3[1].entries[l3_index].is_valid()
        }





    }

    /// Returns `true` if the L3entry indicated by the given virtual address is invalid.
    /// Otherwise, `true` is returned.
    pub fn is_invalid(&self, va: VirtualAddr) -> bool {
        //unimplemented!("PageTable::is_invalid()")
        !self.is_valid(va)
    }

    /// Set the given RawL3Entry `entry` to the L3Entry indicated by the given virtual
    /// address.
    pub fn set_entry(&mut self, va: VirtualAddr, entry: RawL3Entry) -> &mut Self {
        //unimplemented!("PageTable::set_entry()")
        let (l2_index, l3_index) = PageTable::locate(va);

        let l3_table = self.l2.entries[l2_index].get_masked(RawL2Entry::ADDR) as usize;

        if self.l3[0].as_ptr().as_usize() == l3_table {
            self.l3[0].entries[l3_index].0.set(entry.get());
        } else {
            self.l3[1].entries[l3_index].0.set(entry.get());
        }

        self


    }

    /// Returns a base address of the pagetable. The returned `PhysicalAddr` value
    /// will point the start address of the L2PageTable.
    pub fn get_baddr(&self) -> PhysicalAddr {
        //unimplemented!("PageTable::get_baddr()")
        PhysicalAddr::from(&self.l2.entries as *const _ as usize)

    }
}

// FIXME: Implement `IntoIterator` for `&PageTable`.

impl <'a> IntoIterator for &'a PageTable{

    // add code here
    type Item = &'a L3Entry;
    type IntoIter = Chain<Iter<'a, L3Entry>, Iter<'a, L3Entry>>;

    fn into_iter(self) -> Chain<Iter<'a, L3Entry>, Iter<'a,L3Entry>> {
        self.l3[0].entries.iter().chain(self.l3[1].entries.iter())
    }
}

pub struct KernPageTable(Box<PageTable>);

impl KernPageTable {
    /// Returns a new `KernPageTable`. `KernPageTable` should have a `Pagetable`
    /// created with `KERN_RW` permission.
    ///
    /// Set L3entry of ARM physical address starting at 0x00000000 for RAM and
    /// physical address range from `IO_BASE` to `IO_BASE_END` for peripherals.
    /// Each L3 entry should have correct value for lower attributes[10:0] as well
    /// as address[47:16]. Refer to the definition of `RawL3Entry` in `vmsa.rs` for
    /// more details.
    pub fn new() -> KernPageTable {
        //unimplemented!("KernPageTable::new()")
        let mut pt = PageTable::new(EntryPerm::KERN_RW);

        let (begin, end) = allocator::memory_map().unwrap();


        let start = 0x00000000;
        for x in (start..end).step_by(Page::SIZE) {
            let mut new_l3_entry = (L3Entry::new()).0;
            new_l3_entry.set_masked(x as u64, RawL3Entry::ADDR);


            new_l3_entry.set_value(PageType::Page, RawL3Entry::TYPE);
            new_l3_entry.set_value(EntryPerm::KERN_RW, RawL3Entry::AP);
            new_l3_entry.set_value(EntrySh::ISh, RawL3Entry::SH);
            new_l3_entry.set_value(EntryValid::Valid, RawL3Entry::VALID);
            new_l3_entry.set_value(EntryAttr::Mem, RawL3Entry::ATTR);

            new_l3_entry.set_bit(RawL3Entry::AF);

            pt.set_entry(VirtualAddr::from(x), new_l3_entry);
        }

         for y in (IO_BASE..IO_BASE_END).step_by(Page::SIZE) {
            let mut new_l3_entry = (L3Entry::new()).0;
            new_l3_entry.set_masked(y as u64, RawL3Entry::ADDR);


            new_l3_entry.set_value(PageType::Page, RawL3Entry::TYPE);
            new_l3_entry.set_value(EntryPerm::KERN_RW, RawL3Entry::AP);
            new_l3_entry.set_value(EntrySh::OSh, RawL3Entry::SH);
            new_l3_entry.set_value(EntryValid::Valid, RawL3Entry::VALID);
            new_l3_entry.set_value(EntryAttr::Dev, RawL3Entry::ATTR);

            new_l3_entry.set_bit(RawL3Entry::AF);

            pt.set_entry(VirtualAddr::from(y), new_l3_entry);
        }



        KernPageTable (
            pt
            )
            
        
    }
}

pub enum PagePerm {
    RW,
    RO,
    RWX,
}

pub struct UserPageTable(Box<PageTable>);

impl UserPageTable {
    /// Returns a new `UserPageTable` containing a `PageTable` created with
    /// `USER_RW` permission.
    pub fn new() -> UserPageTable {
        //unimplemented!("UserPageTable::new()")
        UserPageTable(PageTable::new(EntryPerm::USER_RW))
    }

    /// Allocates a page and set an L3 entry translates given virtual address to the
    /// physical address of the allocated page. Returns the allocated page.
    ///
    /// # Panics
    /// Panics if the virtual address is lower than `USER_IMG_BASE`.
    /// Panics if the virtual address has already been allocated.
    /// Panics if allocator fails to allocate a page.
    ///
    /// TODO. use Result<T> and make it failurable
    /// TODO. use perm properly
    pub fn alloc(&mut self, va: VirtualAddr, _perm: PagePerm) -> &mut [u8] {
        //unimplemented!("alloc()");

        if va.as_usize() < USER_IMG_BASE {
            panic!("{:?}", "Virtual Address is lower than USER_IMG_BASE" );
        }

        let new_va = VirtualAddr::from(va.as_u64() - USER_IMG_BASE as u64);


        if self.is_valid(new_va) {
            panic!("{:?}", "Already Allocated");
        }

        //*mut u8

        let ptr = unsafe{ALLOCATOR.alloc(Page::layout())};



        let mut new_l3_entry = (L3Entry::new()).0;
        new_l3_entry.set_masked(ptr as u64, RawL3Entry::ADDR);


        new_l3_entry.set_value(PageType::Page, RawL3Entry::TYPE);
        match _perm {

            PagePerm::RWX => {
                use aarch64::EntryPerm::USER_RW;  
                new_l3_entry.set_value(USER_RW, RawL3Entry::AP);
            },
            
            PagePerm::RW => {
                
                use aarch64::EntryPerm::USER_RW;
                new_l3_entry.set_value(USER_RW, RawL3Entry::AP);
            },
            PagePerm::RO => {
                
                use aarch64::EntryPerm::USER_RO;
                new_l3_entry.set_value(USER_RO, RawL3Entry::AP);
            },
        };
        
        new_l3_entry.set_value(EntrySh::ISh, RawL3Entry::SH);
        new_l3_entry.set_value(EntryValid::Valid, RawL3Entry::VALID);
        new_l3_entry.set_value(EntryAttr::Mem, RawL3Entry::ATTR);
        new_l3_entry.set_bit(RawL3Entry::AF);
        self.set_entry(new_va, new_l3_entry);




        let return_page = unsafe {core::slice::from_raw_parts_mut(ptr, Page::SIZE)};


        return_page










    }
}

impl Deref for KernPageTable {
    type Target = PageTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for UserPageTable {
    type Target = PageTable;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KernPageTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DerefMut for UserPageTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// FIXME: Implement `Drop` for `UserPageTable`.
impl Drop for UserPageTable {
    fn drop(&mut self) {
        //loop through drop all the entries
        //w//
        for entry in &*self.0 {
            if entry.is_valid() {
                unsafe{ALLOCATOR.dealloc(PhysicalAddr::from(entry.0.get_masked(RawL3Entry::ADDR)).as_mut_ptr(), Page::layout())};
            }

        }
    }
}



// FIXME: Implement `fmt::Debug` as you need.
impl fmt::Debug for UserPageTable {
    // add code here
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{:?}", self.0.get_baddr())        


    }

}


impl fmt::Debug for L2PageTable {
    // add code here
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_ptr())        
    }

}


impl fmt::Debug for L3PageTable {
    // add code here
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.as_ptr())

    }

}

