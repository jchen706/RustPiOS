use alloc::boxed::Box;
use pi::interrupt::Interrupt;

use crate::mutex::Mutex;
use crate::traps::TrapFrame;

pub type IrqHandler = Box<dyn FnMut(&mut TrapFrame) + Send>;
pub type IrqHandlers = [Option<IrqHandler>; Interrupt::MAX];

pub struct Irq(Mutex<Option<IrqHandlers>>);

use crate::console::kprintln;


impl Irq {
    pub const fn uninitialized() -> Irq {
        Irq(Mutex::new(None))
    }

    pub fn initialize(&self) {
        *self.0.lock() = Some([None, None, None, None, None, None, None, None]);
    }

    /// Register an irq handler for an interrupt.
    /// The caller should assure that `initialize()` has been called before calling this function.
    pub fn register(&self, int: Interrupt, handler: IrqHandler) {
        //unimplemented!("Irq::register()")

        self.0.lock().as_mut().unwrap()[Interrupt::to_index(int)] = Some(handler); 



    }

    /// Executes an irq handler for the givven interrupt.
    /// The caller should assure that `initialize()` has been called before calling this function.
    pub fn invoke(&self, int: Interrupt, tf: &mut TrapFrame) {

        //unimplemented!("Irq::register()")
        //let b:_ = self.0.lock().as_mut().unwrap()[Interrupt::to_index(int)];

        //let _:() = self.0.lock().as_mut().unwrap()[Interrupt::to_index(int)].unwrap();
        //self.0.lock().as_mut().unwrap()[Interrupt::to_index(int)].unwrap()(tf);

        // match &self.0.lock().as_mut().unwrap()[Interrupt::to_index(int)] {

        //     Some(mut x)=> {
        //         let b:_ = x;
        //         x(tf);


        //     },
        //     _=> return,


        // }

        match *self.0.lock() {
            Some(ref mut handlers) =>{

                //let _:()= handlers;
                //&mut [core::option::Option<alloc::boxed::Box<(dyn for<'r> core::ops::FnMut(&'r mut traps::frame::TrapFrame) + core::marker::Send + 'static)>>; 8]

                //let b = handlers;//[ rrupt::to_index(int)];
                //function
                handlers[Interrupt::to_index(int)].as_mut().unwrap()(tf);

            },

            _=>return,

        }




    }
}



