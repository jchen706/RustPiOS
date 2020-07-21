
use alloc::boxed::Box;
use alloc::collections::vec_deque::VecDeque;
use core::fmt;


use aarch64::*;

use crate::mutex::Mutex;
use crate::param::{PAGE_MASK, PAGE_SIZE, TICK, USER_IMG_BASE};
use crate::process::{Id, Process, State};
use crate::traps::TrapFrame;
//extern crate main;
//use start_shell;
use crate::console::kprintln;
use crate::shell;
use pi::timer::Timer;
use pi::interrupt::{Controller, Interrupt};
use pi::timer::tick_in;

use crate::SCHEDULER;
use crate::IRQ;



use crate::VMM;


/// Process scheduler for the entire machine.
#[derive(Debug)]
pub struct GlobalScheduler(Mutex<Option<Scheduler>>);

impl GlobalScheduler {
    /// Returns an uninitialized wrapper around a local scheduler.
    pub const fn uninitialized() -> GlobalScheduler {
        GlobalScheduler(Mutex::new(None))
    }

    /// Enter a critical region and execute the provided closure with the
    /// internal scheduler.
    pub fn critical<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut Scheduler) -> R,
    {
        let mut guard = self.0.lock();
        f(guard.as_mut().expect("scheduler uninitialized"))
    }


    /// Adds a process to the scheduler's queue and returns that process's ID.
    /// For more details, see the documentation on `Scheduler::add()`.
    pub fn add(&self, process: Process) -> Option<Id> {
        //kprintln!("Add process {}", process.context.tpidr);
        self.critical(move |scheduler| scheduler.add(process))
    }

    /// Performs a context switch using `tf` by setting the state of the current
    /// process to `new_state`, saving `tf` into the current process, and
    /// restoring the next process's trap frame into `tf`. For more details, see
    /// the documentation on `Scheduler::schedule_out()` and `Scheduler::switch_to()`.
    pub fn switch(&self, new_state: State, tf: &mut TrapFrame) -> Id {
        //kprintln!("Switch process State:{:?} ID: {}",new_state, tf.tpidr);
        self.critical(|scheduler| scheduler.schedule_out(new_state, tf));
        self.switch_to(tf)
    }

    pub fn switch_to(&self, tf: &mut TrapFrame) -> Id {
        //kprintln!("Switch_to function process ID: {:?}", tf.tpidr);
        loop {
            let rtn = self.critical(|scheduler| scheduler.switch_to(tf));
            if let Some(id) = rtn {
                return id;
            }
            //kprintln!("right before wfe");
            aarch64::wfe();
        }
    }

    /// Kills currently running process and returns that process's ID.
    /// For more details, see the documentaion on `Scheduler::kill()`.
    #[must_use]
    pub fn kill(&self, tf: &mut TrapFrame) -> Option<Id> {
        kprintln!("Kill process ID: {:?}", tf.tpidr);
        self.critical(|scheduler| scheduler.kill(tf))
    }

    /// Starts executing processes in user space using timer interrupt based
    /// preemptive scheduling. This method should not return under normal conditions.
    pub fn start(&self) -> ! {
        //unimplemented!("GlobalScheduler::start()")

        //let mut scheduler = Scheduler::new();

        //let timer1 = Timer::new();


        //timer1.tick_in(TICK);


        kprintln!("{:?}", "SCHEDULER start function");

        let mut trap_fp = TrapFrame::default();
        Controller::new().enable(Interrupt::Timer1);

        //kprintln!("{:?}", "here");
        IRQ.register(Interrupt::Timer1, Box::new(|trap_fp| {
            tick_in(TICK);
            SCHEDULER.switch(State::Ready, trap_fp);

        }));

        // timer_handler
        tick_in(TICK);




        self.switch_to(&mut trap_fp);


        







        // let mut process1 = Process::new().unwrap();

        // let mut trap_fp;

        
        // let mut process2 = process1;
        // process2.context.sp = process2.stack.top().as_u64();
        // process2.context.spsr = process2.context.spsr & 0b11111111111111111111111101111111;
        // trap_fp = process2.context.clone();

        // process2.context.elr = start_shell as u64;
        // self.add(process2).unwrap();

        //&*self.0.lock().as_mut().unwrap().processes[0].context
        
        
        let stack_p:usize;
        unsafe{
                asm!(

                    "mov sp, $1 
                     bl context_restore
                    
                   
                     mov x1, SP

                     add x1, x1, $0
                     and x2, x1, #0xf
                     sub x1, x1, x2
                     
                     
                     
                    
                     
                     
                     mov sp, x1
                    
                     mov x2, xzr
                     mov x1, xzr
                     mov x0, xzr
                     mov lr, xzr
                   
                     eret         
            
                    "::"r"(PAGE_SIZE), "r"(&trap_fp) 
                     ::"volatile");

                  //kprintln!(" The return stack pointer {:?}" ,stack_p);
                  //kprintln!("{:?}" ,SP.get());


                  //let new_stack_p = (SP.get() as usize + PAGE_SIZE); 
                  //kprintln!("The new stack pointer {:?}", new_stack_p);


                  //&trap_fp as *const TrapFrame

                // asm!("
                //      mov sp, $0
                //      bl context_restore

                //      adr x0, _start
                //      mov sp, x0

                //      mov x0, xzr
                //      mov lr,xzr
                //      eret

                //      "::"r"(&trap_fp)
                //       ::"volatile"



                //     );



        }














        


        loop {
            
        }
    }

    /// Initializes the scheduler and add userspace processes to the Scheduler
    pub unsafe fn initialize(&self) {
        //unimplemented!("GlobalScheduler::initialize()")
        *self.0.lock() = Some(Scheduler::new());


        let process1 = match Process::load("/fib") {
            Ok(process) => process,
            Err(error) => panic!("Initalize a process on load {:?}", error),
            _=> panic!("Initalize a process on load {:?}", "Panic process1"),
        };

       

        self.add(process1);


        let process2 = match Process::load("/fib") {
            Ok(process) => process,
            Err(error) => panic!("Initalize a process on load {:?}", error),
            _=> panic!("Initalize a process on load {:?}", "Panic process1"),
        };


        kprintln!("Adding Process 2");
        self.add(process2);


        let process3 = match Process::load("/fib") {
            Ok(process) => process,
            Err(error) => panic!("Initalize a process on load {:?}", error),
            _=> panic!("Initalize a process on load {:?}", "Panic process1"),
        };


        kprintln!("Adding Process 3");
        self.add(process3);


        let process4 = match Process::load("/fib") {
            Ok(process) => process,
            Err(error) => panic!("Initalize a process on load {:?}", error),
            _=> panic!("Initalize a process on load {:?}", "Panic process1"),
        };


        kprintln!("Adding Process 4");
        self.add(process4);
        //kprintln!("Adding process {}", id.unwrap());







        // let mut process1 = Process::new().unwrap();

        // let mut trap_fp;

        
        // let mut process2 = process1;
        // process2.context.sp = process2.stack.top().as_u64();
        // //process2.context.spsr = process2.context.spsr & 0b11111111111111111111111101111111;
        // //

        // // process2.context.elr = start_shell as u64;
        // // self.add(process2).unwrap();

        // process2.context.spsr = process2.context.spsr | SPSR_EL1::D|SPSR_EL1::A | SPSR_EL1:: F;
        // process2.context.ttbr0 = VMM.get_baddr().as_u64();
        // process2.context.ttbr1 = process2.vmap.get_baddr().as_u64();

        // self.test_phase_3(&mut process2);

        // trap_fp = process2.context.clone();

        // process2.context.elr = USER_IMG_BASE as u64;


        // self.add(process2).unwrap();












    }


   

    // The following method may be useful for testing Phase 3:
    //
    // * A method to load a extern function to the user process's page table.
    //
    pub fn test_phase_3(&self, proc: &mut Process){
        use crate::vm::{VirtualAddr, PagePerm};
    
        let mut page = proc.vmap.alloc(
            VirtualAddr::from(USER_IMG_BASE as u64), PagePerm::RWX);
    
        let text = unsafe {
            core::slice::from_raw_parts(test_user_process as *const u8, 24)
        };
    
        page[0..24].copy_from_slice(text);
    }
}

#[derive(Debug)]
pub struct Scheduler {
    processes: VecDeque<Process>,
    last_id: Option<Id>,
}

impl Scheduler {
    /// Returns a new `Scheduler` with an empty queue.
    fn new() -> Scheduler {
        //unimplemented!("Scheduler::new()")
        Scheduler {
            processes:VecDeque::new(),
            last_id:None,
        }
    }

    /// Adds a process to the scheduler's queue and returns that process's ID if
    /// a new process can be scheduled. The process ID is newly allocated for
    /// the process and saved in its `trap_frame`. If no further processes can
    /// be scheduled, returns `None`.
    ///
    /// It is the caller's responsibility to ensure that the first time `switch`
    /// is called, that process is executing on the CPU.
    fn add(&mut self, mut process: Process) -> Option<Id> {


        //unimplemented!("Scheduler::add()")


        //assign unique id
        if self.last_id == None {
            self.last_id = Some(0);
        } else {
            let x1 = self.last_id.unwrap();
            let x2 = x1.checked_add(1);
            if x2 == None {
                self.last_id = Some(0);
            } else{
                self.last_id = x2;
            }
        }
        process.context.tpidr = self.last_id.unwrap();
        // kprintln!("Schedule OUT ID:{:x?}",  process.context.tpidr);
        // kprintln!("Out ELR: {:x?} ",  process.context.elr);
        // kprintln!("Out spsr: {:x?}",  process.context.spsr);
        // kprintln!("Out sp: {:x?} ",  process.context.sp);
        // kprintln!("Out tpidr: {:x?} ",  process.context.tpidr);
        // kprintln!("Out ttbr1: {:x?}",  process.context.ttbr1);
        // kprintln!("Out ttbr0: {:x?} ", process.context.ttbr0);
        // kprintln!("Out x: {:x?}",  process.context.x[0]);
        // //kprintln!("Out lr: {:x?}",  process.context.lr);
        //kprintln!("Out xzr: {:x?} ",  process.context.xzr);

        self.processes.push_back(process);

        //kprintln!("Add Process ID: {:?}", self.last_id);

     


        self.last_id

    }

    /// Finds the currently running process, sets the current process's state
    /// to `new_state`, prepares the context switch on `tf` by saving `tf`
    /// into the current process, and push the current process back to the
    /// end of `processes` queue.
    ///
    /// If the `processes` queue is empty or there is no current process,
    /// returns `false`. Otherwise, returns `true`.
    fn schedule_out(&mut self, new_state: State, tf: &mut TrapFrame) -> bool {
        //unimplemented!("Scheduler::schedule_out()")
        if self.processes.is_empty() {
            return false;
        } 

        //find the current running process based on trap frame
        let unique_id = tf.tpidr;

        // kprintln!("Schedule OUT ID:{:x?}", tf.tpidr);
        // kprintln!("Out ELR: {:x?} ", tf.elr);
        // kprintln!("Out spsr: {:x?} ", tf.spsr);
        // kprintln!("Out sp: {:x?} ", tf.sp);
        // kprintln!("Out tpidr: {:x?}", tf.tpidr);
        // kprintln!("Out ttbr1:{:x?} ", tf.ttbr1);
        // kprintln!("Out ttbr0: {:x?} ", tf.ttbr0);
        // kprintln!("Out x: {:x?} ", tf.x[0]);
        //kprintln!("Out lr: {:x?}", tf.lr);
        //kprintln!("Out xzr: {:x?}", tf.xzr);

    


        for i in 0..self.processes.len() {
            let process2 = self.processes.get_mut(i).unwrap();

            if unique_id == process2.context.tpidr {
                let mut current = self.processes.remove(i).unwrap();
                current.state = new_state;

                current.context = Box::new(*tf);
                self.processes.push_back(current);
                return true;

            } else {

              continue;
            }


        }


        return false;

        //let mut current = self.processes.pop_front().unwrap();

    





    }

    /// Finds the next process to switch to, brings the next process to the
    /// front of the `processes` queue, changes the next process's state to
    /// `Running`, and performs context switch by restoring the next process`s
    /// trap frame into `tf`.
    ///
    /// If there is no process to switch to, returns `None`. Otherwise, returns
    /// `Some` of the next process`s process ID.
    fn switch_to(&mut self, tf: &mut TrapFrame) -> Option<Id> {
        //unimplemented!("Scheduler::switch_to()")

        //kprintln!("Switch to function ID:{:x?}", tf.tpidr);
        // kprintln!("Out ELR: {:x?} ", tf.elr);
        // kprintln!("Out spsr: {:x?}", tf.spsr);
        // kprintln!("Out sp: {:x?}", tf.sp);
        // kprintln!("Out tpidr: {:x?} ", tf.tpidr);
        // kprintln!("Out ttbr1: {:x?} ", tf.ttbr1);
        // kprintln!("Out ttbr0: {:x?}", tf.ttbr0);
        // kprintln!("Out x: {:x?}", tf.x[0]);
        //kprintln!("Out lr: {:x?}", tf.lr);
        //kprintln!("Out xzr: {:x?}", tf.xzr);


        for i in 0..self.processes.len() {
            if self.processes[i].is_ready() {
                let mut current = self.processes.remove(i).unwrap();
                current.state = State::Running;

                *tf = *current.context;

                self.processes.push_front(current);

                return Some(tf.tpidr);






            }
        }

        return None;
        


    }

    /// Kills currently running process by scheduling out the current process
    /// as `Dead` state. Removes the dead process from the queue, drop the
    /// dead process's instance, and returns the dead process's process ID.
    fn kill(&mut self, tf: &mut TrapFrame) -> Option<Id> {
        //unimplemented!("Scheduler::kill()")

       

        //find the current running process based on trap frame
        let unique_id = tf.tpidr;

        kprintln!("Schduler kills the process with ID {}", unique_id);


        for i in 0..self.processes.len() {
            let process2 = self.processes.get_mut(i).unwrap();

            if unique_id == process2.context.tpidr {
                let truef = self.schedule_out(State::Dead, tf);
                if truef {

                    let mut current = self.processes.pop_back();


                    drop(current);

                    return Some(unique_id);


                }

                

            } else {

              continue;
            }


        }


        return None;




    }
}

pub extern "C" fn  test_user_process() -> ! {
    loop {
        let ms = 10000;
        let error: u64;
        let elapsed_ms: u64;

        unsafe {
            asm!("mov x0, $2
                  svc 1
                  mov $0, x0
                  mov $1, x7"
                 : "=r"(elapsed_ms), "=r"(error)
                 : "r"(ms)
                 : "x0", "x7"
                 : "volatile");
        }
    }
}







pub extern fn start_shell() {

    unsafe { asm!("brk 1" :::: "volatile"); }
    unsafe { asm!("brk 2" :::: "volatile"); }
    shell::shell("user0> ");
    unsafe { asm!("brk 3" :::: "volatile"); }
    loop { shell::shell("user111111> "); }

    loop {
        shell::shell("extern111111>$ ");
    }

}


pub extern fn start_shell1() {

    unsafe { asm!("brk 1" :::: "volatile"); }
    unsafe { asm!("brk 2" :::: "volatile"); }
    shell::shell("user0> ");
    unsafe { asm!("brk 3" :::: "volatile"); }
    loop { shell::shell("user77777> "); }

    loop {
        shell::shell("extern77777>$ ");
    }

}


pub extern fn start_shell2() {

    unsafe { asm!("brk 1" :::: "volatile"); }
    unsafe { asm!("brk 2" :::: "volatile"); }
    shell::shell("user0888> ");
    unsafe { asm!("brk 3" :::: "volatile"); }
    loop { shell::shell("user18888> "); }

    loop {
        shell::shell("extern8888>$ ");
    }

}


pub extern fn start_shell3() {

    unsafe { asm!("brk 1" :::: "volatile"); }
    unsafe { asm!("brk 2" :::: "volatile"); }
    shell::shell("user999> ");
    unsafe { asm!("brk 3" :::: "volatile"); }
    loop { shell::shell("user9999> "); }

    loop {
        shell::shell("extern999>$ ");
    }

}

