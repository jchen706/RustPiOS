use core::fmt;
use shim::const_assert_size;
use shim::io;
use core::mem;

use crate::traits::BlockDevice;



// head, sector, cylinder, offset 1 hex
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CHS {
    // FIXME: Fill me in.
  
    head: u8,
    cylinder_sector: u8,
    cylinder: u8,

}


impl CHS {
    // add code here

    pub fn get_cylinder(&self) -> u16 {

        //bits 9 and 8
        let mut upper = ((self.cylinder_sector & 0b11000000) as u16) << 2;
        self.cylinder as u16 | upper
    }

    pub fn get_sector(&self)->u8 {
        self.cylinder_sector & 0b00111111
    }



}

//cylinder is 0 to 9
//sector is  5
// FIXME: implement Debug for CHS

impl fmt::Debug for CHS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("CHS")
            .field("cylinder", &self.get_cylinder())
            .field("head", &self.head)
            .field("sector", &self.get_sector())
            .finish()
    }
}



const_assert_size!(CHS, 3);

#[repr(C, packed)]
pub struct PartitionEntry {
    // FIXME: Fill me in.
    status: u8,
    first_chs: CHS,
    partition_type: u8,
    last_chs: CHS,
    relative_sector: u32,
    total_sector: u32,

}

impl PartitionEntry {


    pub fn get_bootflag(&self)-> bool {
        if self.status == 0 {
            false
        } else {
            if self.status == 0x80 {
                return true;   
            }
            false
        }
    }


    pub fn is_fat32(&self)-> bool {
        self.partition_type == 0xB || self.partition_type == 0xC
    }


    pub fn get_relative_sector(&self)-> u32 {
        self.relative_sector
    }


}

// FIXME: implement Debug for PartitionEntry


impl fmt::Debug for PartitionEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PartitionEntry")
            .field("status", &self.status)
            .field("first_chs", &self.first_chs)
            .field("partition_type", &self.partition_type)
            .field("last_chs", &self.last_chs)
            .field("relative_sector", &self.relative_sector)
            .field("total_sector", &self.total_sector)
            .finish()
    }
}

const_assert_size!(PartitionEntry, 16);

/// The master boot record (MBR).
#[repr(C, packed)]
pub struct MasterBootRecord {
    bootstrap: [u8; 436],
    unique_id: [u8; 10],
    partition_table: [PartitionEntry; 4],
    signature: [u8;2],
}

// FIXME: implemente Debug for MaterBootRecord

impl fmt::Debug for MasterBootRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MasterBootRecord")
            //.field("bootstrap", &self.bootstrap)
            .field("unique_id", &{self.unique_id})
            .field("partition_table", &self.partition_table)
            .field("signature", &{self.signature})
            .finish()
    }
}


const_assert_size!(MasterBootRecord, 512);

#[derive(Debug)]
pub enum Error {
    /// There was an I/O error while reading the MBR.
    Io(io::Error),
    /// Partiion `.0` (0-indexed) contains an invalid or unknown boot indicator.
    UnknownBootIndicator(u8),
    /// The MBR magic signature was invalid.
    BadSignature,
}

impl From<io::Error> for Error {
    fn from (error: io::Error)-> Self {
        Error::Io(error)
    }
}

impl MasterBootRecord {
    /// Reads and returns the master boot record (MBR) from `device`.
    ///
    /// # Errors
    ///
    /// Returns `BadSignature` if the MBR contains an invalid magic signature.
    /// Returns `UnknownBootIndicator(n)` if partition `n` contains an invalid
    /// boot indicator. Returns `Io(err)` if the I/O error `err` occured while
    /// reading the MBR.
    pub fn from<T: BlockDevice>(mut device: T) -> Result<MasterBootRecord, Error> {
        let mut sector = [0u8; 512];

        let value = device.read_sector(0, &mut sector)?;

        //println!("{:?}",value);
        const size: usize = 512;
        if value != size {
            return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "Device did not read 512 bytes")))
        } else {


        //io Error

        let mbr: MasterBootRecord = unsafe{mem::transmute(sector)};

        if mbr.signature != [0x55, 0xAA] {
            return Err(Error::BadSignature);
        }

        for i in 0..4 {
            if mbr.partition_table[i].status != 0x00 && mbr.partition_table[i].status !=0x80 {
                return Err(Error::UnknownBootIndicator(i as u8));
            }
        }


        Ok(mbr)
    }

    }


    pub fn get_partition(&self) -> Option<&[PartitionEntry;4]> {
        Some(&self.partition_table)
    }
}
