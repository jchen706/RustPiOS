use alloc::boxed::Box;
use core::time::Duration;

use crate::console::CONSOLE;
use crate::process::State;
use crate::traps::TrapFrame;
use crate::SCHEDULER;
use kernel_api::*;
use pi::timer::current_time;
use crate::process::Process;

use crate::console::kprintln;
use crate::console::kprint;

/// Sleep for `ms` milliseconds.
///
/// This system call takes one parameter: the number of milliseconds to sleep.
///
/// In addition to the usual status value, this system call returns one
/// parameter: the approximate true elapsed time from when `sleep` was called to
/// when `sleep` returned.
pub fn sys_sleep(ms: u32, tf: &mut TrapFrame) {
    //unimplemented!("sys_sleep()");

    let start = current_time();
    let release_time = start.as_micros() + Duration::from_micros(ms as u64 * 1000).as_micros();

    //Box<dyn FnMut(&mut Process) + Send> 


   kprintln!("Staring system Sleep {} ", start.as_secs() as u64);
   kprintln!("Staring system Sleep End Time {} ", Duration::from_micros(release_time as u64).as_secs() as u64);

   let boxed_fnmut= Box::new(move |p: &mut Process|-> bool {

   	 

    

         let time1 = current_time();


          if time1.as_micros() >= release_time {
            let elapsed_time = time1.as_millis() - start.as_millis();
                p.context.x[0] =  elapsed_time as u64;
                p.context.x[7] =  OsError::Ok as u64;
                kprintln!("End system Sleep End Time {:?} ", release_time as u64);

                return true

      } else {
            return false

      }


      

   	 



   });


   SCHEDULER.switch(State::Waiting(boxed_fnmut), tf);






}

/// Returns current time.
///
/// This system call does not take parameter.
///
/// In addition to the usual status value, this system call returns two
/// parameter:
///  - current time as seconds
///  - fractional part of the current time, in nanoseconds.
pub fn sys_time(tf: &mut TrapFrame) {

   let current = current_time();
   let seconds = current.as_secs();
   let nano = (current-Duration::from_secs(seconds)).as_nanos() as u64;
   

   //.as_nanos();

    //unimplemented!("sys_time()");

  tf.x[0] = seconds;
  tf.x[1] = nano;
  tf.x[7] = OsError::Ok as u64;
     
}

/// Kills current process.
///
/// This system call does not take paramer and does not return any value.
pub fn sys_exit(tf: &mut TrapFrame) {
   //unimplemented!("sys_exit()");

 kprintln!("Process: {:?} is switch into dead state", tf.tpidr); 
 let a = SCHEDULER.switch(State::Dead, tf);
   
}

/// Write to console.
///
/// This system call takes one parameter: a u8 character to print.
///
/// It only returns the usual status value.
pub fn sys_write(b: u8, tf: &mut TrapFrame) {
    //unimplemented!("sys_write()");
    
 
      //kprintln!("System writing {}", b);
      if b >= 0 && b <=127 {
        let string = b as char;
        kprint!("{:?}", string);
        tf.x[7] = OsError::Ok as u64;

      } else {
        //kprintln!("System writing Error {}", b as char);
        tf.x[7] = OsError::IoErrorInvalidInput as u64;
      }
      

    
   


}

/// Returns current process's ID.
///
/// This system call does not take parameter.
///
/// In addition to the usual status value, this system call returns a
/// parameter: the current process's ID.
pub fn sys_getpid(tf: &mut TrapFrame) {
    //unimplemented!("sys_getpid()");
    //let id = tf.tpidr;
    tf.x[0]= tf.tpidr;
    tf.x[7] = OsError::Ok as u64;
    
}

pub fn handle_syscall(num: u16, tf: &mut TrapFrame) {
    use crate::console::kprintln;
    //unimplemented!("handle_syscall()")


    if num == 1 {
      kprintln!(" Before calling SLEEP x[0] parameter {:?}", tf.x[0]);
    	sys_sleep(tf.x[0] as u32,tf);
    
    } else if num == 2 {
      kprintln!(" Before calling TIMER");

      sys_time(tf);
    } else if num == 3 {
      kprintln!(" Before sys_exit");

      sys_exit(tf);
    }  else if num == 4 {
      //kprintln!(" Before calling writing x[0] parameter {:?}", tf.x[0]);

      sys_write(tf.x[0] as u8 , tf);
    }  else if num == 5 {
      kprintln!("Get PID");

      sys_getpid(tf);
    }

    else {
    	tf.x[7] = OsError::Ok as u64;
    }







}





