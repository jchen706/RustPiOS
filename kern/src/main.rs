#![feature(alloc_error_handler)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(optin_builtin_traits)]
#![feature(ptr_internals)]
#![feature(raw_vec_internals)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(panic_info_message)]


#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod fs;
pub mod mutex;
pub mod shell;
pub mod param;
pub mod process;
pub mod traps;
pub mod vm;

use console::kprintln;
use console::kprint;
use alloc::vec::Vec;
use alloc::string::String;


use allocator::Allocator;
use fs::FileSystem;
use process::GlobalScheduler;
use traps::irq::Irq;
use vm::VMManager;


//use fat32::traits::FileSystem;
use fat32::traits::{Dir, Entry};
use aarch64::*;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();
pub static FILESYSTEM: FileSystem = FileSystem::uninitialized();
pub static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();
pub static VMM: VMManager = VMManager::uninitialized();
pub static IRQ: Irq = Irq::uninitialized();

fn kmain() -> ! {
    
    spin_sleep(Duration::new(2,0));
    //panic!("Working Panic");

    unsafe {
        ALLOCATOR.initialize();
        FILESYSTEM.initialize();
        IRQ.initialize();
        VMM.initialize();
        //SCHEDULER.initialize();
        //SCHEDULER.start()
    }

    use core::fmt::Write;

    use pi::timer::spin_sleep;
    use core::time::Duration;
    use pi::gpio::Gpio;
    use pi::uart::MiniUart;

    use crate::shell::shell;

    //String::from("Hi!");



    // let s1 = String::from("helllo");
    // let s2 = "h";
    // kprintln!("{}",&s2[..]);
    // use core::arch::aarch64::brk;


    // unsafe{
    //     kprintln!("{}",current_el());
    // }




    // const GPIO_BASE: usize = 0x3F000000 + 0x200000;
    //
    // const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
    // const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
    // const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;





    
        // FIXME: Start the shell.
        //

        //Gpio::Tests
        // let mut test1 = Gpio::new(5).into_output();
        // let mut test2 = Gpio::new(6).into_output();
        // let mut test3 = Gpio::new(13).into_output();
        // let mut test4 = Gpio::new(19).into_output();
        // let mut test5 = Gpio::new(26).into_output();

        //for atags in pi::atags::Atags::get() {
          //  kprintln!("{:?}", atags);
        //}

        //Gpio::new(15).into_output();


        //let mut m = MiniUart::new();

        
            //gpio tests

            // test1.set();
            // spin_sleep(Duration::new(1,0));
            // test1.clear();

            // test2.set();
            // spin_sleep(Duration::new(1,0));
            // test2.clear();

            // test3.set();
            // spin_sleep(Duration::new(1,0));
            // test3.clear();

            // test4.set();
            // spin_sleep(Duration::new(1,0));
            // test4.clear();

            // test5.set();
            // spin_sleep(Duration::new(1,0));
            // test5.clear();

            //use fat32::traits::FileSystem;
            // use fat32::traits::{FileSystem,Dir, Entry};

            // let t = FILESYSTEM.open("/").unwrap();
            // //kprintln!("{}", t.as_dir().unwrap().entries().unwrap();
            // for each in t.as_dir().unwrap().entries().unwrap() {
            //     kprintln!("{:?}", each.name());
            //    // kprintln!("{:?}", 1);
                
                    
                


            // }




            // test1.set();
            // spin_sleep(Duration::new(1,0));
            // test1.clear();
            // GPIO_CLR0.write_volatile(GPIO_CLR0.read_volatile() | (1<<16));
            // spin_sleep(Duration::new(1,0));
            //let byte = m.read_byte();
            //kprintln!("{} receive", byte);
            //kprint!("good");
            //m.write_byte(byte);
            //kprintln!("{}","Start");

        loop {

            kprintln!("Welcome to cs3210!");
            shell::shell("> ");
            unsafe{asm!("brk 2" :::: "volatile");}

        }
}


pub extern fn start_shell() {

    unsafe { asm!("brk 1" :::: "volatile"); }
    unsafe { asm!("brk 2" :::: "volatile"); }
    shell::shell("user0> ");
    unsafe { asm!("brk 3" :::: "volatile"); }
    loop { shell::shell("user1> "); }

    loop {
        shell::shell("extern>$ ");
    }

}
