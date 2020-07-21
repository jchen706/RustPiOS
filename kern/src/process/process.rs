use alloc::boxed::Box;
use shim::io;
use shim::path::Path;
use core::mem;

use aarch64;

use crate::param::*;
use crate::process::{Stack, State};
use crate::traps::TrapFrame;
use crate::vm::*;
use kernel_api::{OsError, OsResult};

use crate::console::kprintln;
use crate::FILESYSTEM;

use fat32::traits::FileSystem;
use fat32::traits::Entry;
use fat32::vfat::File;
//use fat32::vfat::Entry;
//use fat32::traits::File;
use crate::fs::PiVFatHandle;
use shim::io::Read;

use core::ops::AddAssign;
use shim::path::Component;



/// Type alias for the type of a process ID.
pub type Id = u64;

/// A structure that represents the complete state of a process.
#[derive(Debug)]
pub struct Process {
    /// The saved trap frame of a process.
    pub context: Box<TrapFrame>,
    /// The memory allocation used for the process's stack.
    pub stack: Stack,
    /// The page table describing the Virtual Memory of the process
    pub vmap: Box<UserPageTable>,
    /// The scheduling state of the process.
    pub state: State,

}

impl Process {
    /// Creates a new process with a zeroed `TrapFrame` (the default), a zeroed
    /// stack of the default size, and a state of `Ready`.
    ///
    /// If enough memory could not be allocated to start the process, returns
    /// `None`. Otherwise returns `Some` of the new `Process`.
    pub fn new() -> OsResult<Process> {

        //unimplemented!("Process::new()")

        let stack1 = Stack::new();

        //kprintln!("{:?}", Process::get_max_va());
        //kprintln!("stack base {:?}", );
        //kprintln!("{:x?}", Process::get_stack_base());



        match stack1 {

            Some(x)=> {
                let trap = TrapFrame::default();

                Ok(

                Process {
                context:Box::new(trap),
                stack: x,
                state: State::Ready,
                vmap:Box::new(UserPageTable::new()),
               
                 }
                )

            },
            _=> return Err(OsError::NoMemory),

        }






       




    }

    /// Load a program stored in the given path by calling `do_load()` method.
    /// Set trapframe `context` corresponding to the its page table.
    /// `sp` - the address of stack top
    /// `elr` - the address of image base.
    /// `ttbr0` - the base address of kernel page table
    /// `ttbr1` - the base address of user page table
    /// `spsr` - `F`, `A`, `D` bit should be set.
    ///
    /// Returns Os Error if do_load fails.
    pub fn load<P: AsRef<Path>>(pn: P) -> OsResult<Process> {
        use crate::VMM;

        let mut p = Process::do_load(pn)?;

        //FIXME: Set trapframe for the process.
        p.context.sp = Process::get_stack_top().as_u64();
        p.context.elr = Process::get_image_base().as_u64();
        p.context.ttbr0 = VMM.get_baddr().as_u64();
        p.context.ttbr1 = p.vmap.get_baddr().as_u64();
        p.context.spsr = p.context.spsr | aarch64::SPSR_EL1::D | aarch64::SPSR_EL1::A | aarch64::SPSR_EL1::F;
        Ok(p)
    }

    /// Creates a process and open a file with given path.
    /// Allocates one page for stack with read/write permission, and N pages with read/write/execute
    /// permission to load file's contents.
    fn do_load<P: AsRef<Path>>(pn: P) -> OsResult<Process> {
        //unimplemented!();


        kprintln!("Loading the Process in DO LOAD path: {:?} ", pn.as_ref().as_os_str());

        let mut process = match Process::new() {
            Ok(process1) => process1,
            Err(error) => panic!("Error Do Load Process {:?}", error),
        };

        kprintln!("DO LOAD: Process New created");

        process.vmap.alloc(Process::get_stack_base(), PagePerm::RW);

        kprintln!("DO LOAD: stack allocated");


        let file_entry = match FILESYSTEM.open(pn.as_ref()) {
            Ok(entry)=> entry,
            Err(error)=> panic!("File System Error in Process Open {:?}",error),

        };

        //kprintln!("DO LOAD: FILESYSTEM OPEN");

        let mut file = match file_entry.into_file() {
            Some(file1)=> file1,
            None => return Err(OsError::IoErrorInvalidData),
        };


        //kprintln!("DO LOAD: FILE INTO FILE");

        let mut virtual_add = Process::get_image_base();

        loop {

            let bytes_read = file.read(process.vmap.alloc(virtual_add, PagePerm::RWX))?;
            virtual_add.add_assign(VirtualAddr::from(Page::SIZE));
            if bytes_read == 0 {
                break;
            } 
            


        }

        Ok(process)



    }

    /// Returns the highest `VirtualAddr` that is supported by this system.
    pub fn get_max_va() -> VirtualAddr {
        VirtualAddr::from((USER_MAX_VM_SIZE) + (USER_IMG_BASE -1))
        //VirtualAddr::from((USER_MAX_VM_SIZE))

       // unimplemented!();
    }

    /// Returns the `VirtualAddr` represents the base address of the user
    /// memory space.
    pub fn get_image_base() -> VirtualAddr {
        VirtualAddr::from(USER_IMG_BASE)
        //unimplemented!();
    }

    /// Returns the `VirtualAddr` represents the base address of the user
    /// process's stack.
    pub fn get_stack_base() -> VirtualAddr {
        VirtualAddr::from(USER_STACK_BASE)
        //unimplemented!();
    }

    /// Returns the `VirtualAddr` represents the top of the user process's
    /// stack.
    pub fn get_stack_top() -> VirtualAddr {
        VirtualAddr::from(USER_STACK_BASE + (Page::SIZE-16))
        //unimplemented!();
    }

    /// Returns `true` if this process is ready to be scheduled.
    ///
    /// This functions returns `true` only if one of the following holds:
    ///
    ///   * The state is currently `Ready`.
    ///
    ///   * An event being waited for has arrived.
    ///
    ///     If the process is currently waiting, the corresponding event
    ///     function is polled to determine if the event being waiting for has
    ///     occured. If it has, the state is switched to `Ready` and this
    ///     function returns `true`.
    ///
    /// Returns `false` in all other cases.
    pub fn is_ready(&mut self) -> bool {
        //unimplemented!("Process::is_ready()")
        match &self.state {
            State::Ready => return true,


            State::Waiting(fab)=> {

                 let mut state1 = mem::replace(&mut self.state, State::Ready);

                    match state1 {
                        State::Waiting(mut x) => {
                            let xb = x.as_mut()(self);
                            //kprintln!("Waiting");
                            if xb {
                                return true;
                            } else {
                                self.state = State::Waiting(x);
                                return false;
                            }
                        },
                        _=> return false,
                    }

            }

            _=> return false,


        }
      
        // if self.state == State::Ready {
        //     return true;
        // } else if self.state == State:Waiting(_) {
           

        // } else {
        //     return false;
        // }



    }
}


pub fn align_down(addr: usize, align: usize) -> usize {

   if (((align!=0) && ((align & (align - 1))==0)) != true)  {
        panic!("{:?}", "Align is not a power of 2");
    }  else {

        let integer = addr % align;
        //kprintln!(" align ment {}", align);

        //kprintln!(" align ment {}", integer);
        let new_addr = addr - integer;
        new_addr
    }
}