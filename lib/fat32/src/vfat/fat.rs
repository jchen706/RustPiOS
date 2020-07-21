use crate::vfat::*;
use core::fmt;

use self::Status::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    /// The FAT entry corresponds to an unused (free) cluster.
    Free,
    /// The FAT entry/cluster is reserved.
    Reserved,
    /// The FAT entry corresponds to a valid data cluster. The next cluster in
    /// the chain is `Cluster`.
    Data(Cluster),
    /// The FAT entry corresponds to a bad (disk failed) cluster.
    Bad,
    /// The FAT entry corresponds to a valid data cluster. The corresponding
    /// cluster is the last in its chain.
    Eoc(u32),
}

#[repr(C, packed)]
pub struct FatEntry(pub u32);

impl FatEntry {
    /// Returns the `Status` of the FAT entry `self`.
    pub fn status(&self) -> Status {
        

        let num: u32 = self.0;
        //set the ingore four bit to 0
        let mut num2: u32 = num & (!(!0 <<(28)));


        if num2 == 0 {
            return Status::Free;
        } else if num2 == 1 {
            return Status::Reserved;
        } else if num2 >= 2 && num2 <= 0x0FFFFFEF{
            return Status::Data(Cluster::from(num));
        } else if num2 >= 0x0FFFFFF0 && num <= 0x0FFFFFF6{
            return Status::Reserved;
        } else if num2 == 0x0FFFFFF7 {
            return Status::Bad;
        } else if  num2 >= 0x0FFFFFF8 && num <= 0x0FFFFFFf {
            return Status::Eoc(num);
        } else {
            return Status::Bad;

        }
    }


}

impl fmt::Debug for FatEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("FatEntry")
            .field("value", &{ self.0 })
            .field("status", &self.status())
            .finish()
    }
}
