use aarch64::ESR_EL1;
use crate::console::kprintln;


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Fault {
    AddressSize,
    Translation,
    AccessFlag,
    Permission,
    Alignment,
    TlbConflict,
    Other(u8),
}

impl From<u32> for Fault {
    fn from(val: u32) -> Fault {
        use self::Fault::*;

        match (val & 0b111111) {
            0b000000 => AddressSize, 
            0b000011 => AddressSize,
            0b000100 => Translation, 
            0b000111 => Translation,
            0b000101 => Translation,
            0b000110 => Translation,
            0b001001 => AccessFlag,
            0b001011 => AccessFlag,
            0b001010 => AccessFlag,
            0b001101 => Permission,
            0b001111 => Permission,
            0b001110 => Permission,
            0b100001 => Alignment,
            0b110000=> TlbConflict,
            _=> Other((val & 0b111111) as u8),
        }

        
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Syndrome {
    Unknown,
    WfiWfe,
    SimdFp,
    IllegalExecutionState,
    Svc(u16),
    Hvc(u16),
    Smc(u16),
    MsrMrsSystem,
    InstructionAbort { kind: Fault, level: u8 },
    PCAlignmentFault,
    DataAbort { kind: Fault, level: u8 },
    SpAlignmentFault,
    TrappedFpu,
    SError,
    Breakpoint,
    Step,
    Watchpoint,
    Brk(u16),
    Other(u32),
}

/// Converts a raw syndrome value (ESR) into a `Syndrome` (ref: D1.10.4).
impl From<u32> for Syndrome {
    fn from(esr: u32) -> Syndrome {
        use self::Syndrome::*;

       // unimplemented!("From<u32> for Syndrome")
        //ec 31:26 cause of exception
        //il 25 indicate whether 32 or 16 for synchronous
        // iss 24-0 instruction specific syndrome field
        let ec = ESR_EL1::get_value(esr as u64, ESR_EL1::EC);
    

        let ec:u8 = (esr >> 26) as u8; 
        let iss = esr & 0xFFFFFF;
    
        //let il = ESR_EL1::get_value(esr,)
        //excluded 32 bit exceptionsc
        let synd:Syndrome = match ec {
            0b000000 => Unknown,
            0b000001 => WfiWfe,
            0b000111 => SimdFp,
            0b001110 => IllegalExecutionState,
            0b010001 => Svc((esr & 0xFFFF) as u16),  //32
            0b010010 => Hvc((esr & 0xFFFF) as u16),  //32
            0b010011 => Smc((esr & 0xFFFF) as u16),  //32
            0b010101 => Svc((esr & 0xFFFF) as u16), //64
            0b010110 => Hvc((esr & 0xFFFF) as u16), //64
            0b010111 => Smc((esr & 0xFFFF) as u16), //64
            0b011000 => MsrMrsSystem,
            0b100000 => InstructionAbort { kind: Fault::from(esr), level: 0 }, //lower
            0b100001 => InstructionAbort { kind: Fault::from(esr), level: 1 }, //same

            0b100010 => PCAlignmentFault,
            0b100100 =>   DataAbort { kind: Fault::from(esr), level: (esr & 0b11 as u32 ) as u8 }, //lower
            0b100101 =>   DataAbort { kind: Fault::from(esr), level: (esr & 0b11 as u32) as u8}, //same

            0b100110 =>   SpAlignmentFault,
            0b101100 =>   TrappedFpu,
            0b101111 =>   SError,
            0b110000 =>   Breakpoint, //lower
            0b110001 => Breakpoint, //same
            0b110010 =>   Step, //lower
            0b110011 => Step, //same
            0b110100 => Watchpoint,
            0b110101 =>   Watchpoint,
            0b111100 =>   Brk((esr & 0xFFFF) as u16),
            _        =>   Other(esr),


        };

        synd





    }
}
