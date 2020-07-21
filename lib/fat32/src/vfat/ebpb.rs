use core::fmt;
use shim::const_assert_size;

use crate::traits::BlockDevice;
use crate::vfat::Error;
use core::mem;


//need to import but already in vfat::Error
use shim::io;


#[repr(C, packed)]
pub struct BiosParameterBlock {
    jmp_short_xx_nop: [u8; 3],
    oem_identifier: [u8; 8], //11
    bytes_per_sector: u16,  //13
    sector_per_cluster: u8,  //14
    reserved_sector: u16,  //16
    number_fat: u8,         //17
    directory_entries: [u8;2],  //19
    total_logical_sectors: u16, //21
    fat_id : u8,                      //22
    sector_per_FAT: [u8;2],            //24
    sector_per_track: [u8;2],      //26
    heads_str_media: [u8;2],          //28  
    hidden_sectors: [u8;4],        //32
    logical_sectors: u32,  //36
    size_FAT_sectors: u32,        //40
    flags: [u8;2],        //42 
    version_number: [u8;2],  //44
    root_cluster: u32,              //48
    fsinfo_struct_sec: [u8;2],  //50
    backup_boot_sec:  [u8;2],  //52
    reserved_vol: [u8;12],  //64
    drive_number: u8,      //65
    windows_nt: u8,  //66
    signature: u8,  //67
    volume_id: [u8;4],   //71
    volume_label: [u8;11],  //82
    system_identifier: [u8;8],  //90
    boot_code: [u8;420],        
    partition_signature: u16,
}

const_assert_size!(BiosParameterBlock, 512);

impl BiosParameterBlock {
    //public functions

    pub fn get_bytes_per_sector(&self) -> u16 {
        self.bytes_per_sector
    }

    pub fn get_sector_per_cluster(&self) -> u8 {
        self.sector_per_cluster
    }

    pub fn get_reserved_sector(&self)-> u16 {
        self.reserved_sector
    }

    pub fn get_sector_per_fat(&self)-> u32 {
        self.size_FAT_sectors
    }

    pub fn get_root_cluster(&self)-> u32 {
        self.root_cluster
    }

    pub fn get_number_FAT(&self)-> u8 {
        self.number_fat
    }

    pub fn get_total_sector(&self)-> u32 {
        self.logical_sectors
    }

    pub fn logical_sector(&self)->u16 {
        self.total_logical_sectors
    }






    /// Reads the FAT32 extended BIOS parameter block from sector `sector` of
    /// device `device`.
    ///
    /// # Errors
    ///
    /// If the EBPB signature is invalid, returns an error of `BadSignature`.
    pub fn from<T: BlockDevice>(mut device: T, sector: u64) -> Result<BiosParameterBlock, Error> {
        
    	let mut buf = [0u8; 512];

    	let value = device.read_sector(sector, &mut buf)?;

    	if value != 512 {
            return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "Device did not read 512 bytes")))

    	}

    	let bpb: BiosParameterBlock = unsafe {mem::transmute(buf)};

    	if bpb.partition_signature != 0xAA55 {
    		return Err(Error::BadSignature);
    	}

    	Ok(bpb)

    }
}

impl fmt::Debug for BiosParameterBlock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("BiosParameterBlock")
        	.field("partition_signature", &{self.partition_signature})
            .finish()


    }
}
