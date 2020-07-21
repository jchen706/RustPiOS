use crate::atags::raw;

pub use crate::atags::raw::{Core, Mem};


use core::slice;
use core::str::from_utf8;

/// An ATAG.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    /// Returns `Some` if this is a `Core` ATAG. Otherwise returns `None`.
    pub fn core(self) -> Option<Core> {
        match self {
            Atag::Core(core) => {
                Some(core)
            }
            _ => {
                None
            }
        }
    }

    /// Returns `Some` if this is a `Mem` ATAG. Otherwise returns `None`.
    pub fn mem(self) -> Option<Mem> {
        match self {
            Atag::Mem(mem) => {
                Some(mem)
            }
            _ => {
                None
            }
        }
    }

    /// Returns `Some` with the command line string if this is a `Cmd` ATAG.
    /// Otherwise returns `None`.
    pub fn cmd(self) -> Option<&'static str> {
        match self {
            Atag::Cmd(value) => {
                Some(value)
            }
            _ => {
                None
            }
        }
    }
}

// FIXME: Implement `From<&raw::Atag> for `Atag`.
impl From<&'static raw::Atag> for Atag {
    fn from(atag: &'static raw::Atag) -> Atag {
        // FIXME: Complete the implementation below.

        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::Atag::CORE, &raw::Kind { core }) => {
                    Atag::Core(core)
                },
                (raw::Atag::MEM, &raw::Kind { mem }) => Atag::Mem(mem),
                (raw::Atag::CMDLINE, &raw::Kind { ref cmd }) => {

                    // cmd is pointer
                    let mut size = 0;

                    //pointer to union cmd
                    let mut pointer:*const u8 = &cmd.cmd;
                    
                     
                        while *pointer != b'\0' {
                             pointer = pointer.offset(1);
                             size +=1;
                        }
                        


                    let x = from_utf8(slice::from_raw_parts(&cmd.cmd , size)).unwrap();

                    Atag::Cmd(x)
                },
                (raw::Atag::NONE, _) => Atag::None,
                (id, _) => Atag::Unknown(id),
            }
        }
    }
}
