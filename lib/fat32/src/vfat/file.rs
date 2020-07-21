use alloc::string::String;

use shim::io::{self, SeekFrom};

use crate::traits;
use crate::vfat::{Cluster, Metadata, VFatHandle};

use alloc::vec::Vec;


#[derive(Debug)]
pub struct File<HANDLE: VFatHandle> {
    pub vfat: HANDLE,
    // FIXME: Fill me in.
    pub start_cluster:Cluster,
    pub name: String,
    pub metadata: Metadata,
    pub size:u64,
    pub current_offset:u64,
    pub current_cluster: Cluster,
    

}


impl <HANDLE: VFatHandle> File<HANDLE>  {


    pub fn name(&self)-> &str {
        &self.name
    }

    pub fn metadata(&self)-> &Metadata {
        &self.metadata
    }
}

// FIXME: Implement `traits::File` (and its supertraits) for `File`.

impl<HANDLE: VFatHandle> io::Seek for File<HANDLE> {
    /// Seek to offset `pos` in the file.
    ///
    /// A seek to the end of the file is allowed. A seek _beyond_ the end of the
    /// file returns an `InvalidInput` error.
    ///
    /// If the seek operation completes successfully, this method returns the
    /// new position from the start of the stream. That position can be used
    /// later with SeekFrom::Start.
    ///
    /// # Errors
    ///
    /// Seeking before the start of a file or beyond the end of the file results
    /// in an `InvalidInput` error.
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {

        //move within a stream of bytes 

        let mut offset = 0;


        //println!("Inside Files SEEK Function in Read Chain{:?}", "funtion");

        //start(i64)
        match _pos {

            SeekFrom::Start(num) => {
                if num as u64 > self.size || num < 0 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Seek Input"));
                } else {
                    offset = num  as u64;
                }

            },
            SeekFrom::Current(num2) => {
                // add code here
                if (num2 < 0) {


                     match self.size.checked_sub((-num2) as u64) {
                        Some(x) => {
                            offset = self.current_offset + x;
                            if (offset > self.size) {
                                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Seek Input"));                        

                            }

                        },
                        None => {
                           return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Seek Input"));                        
                      }
                     }


                } else {
                     if (num2 as u64) + self.current_offset  > self.size {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Seek Input"));
                     } else {
                         offset = self.current_offset + num2 as u64
                     }

                }
            },
            SeekFrom::End(num3)=> {
                if (num3 < 0) {

                     match self.size.checked_sub((-num3) as u64) {
                        Some(x) => {
                            offset = self.size + x;
                            if (offset > self.size) {
                                return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Seek Input"));                        

                            }

                        },
                        None => {
                           return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Seek Input"));                        
                      }
                     }
                     

                } else {

                     if (num3) as u64 + self.size  > self.size {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid Seek Input"));
                     } else {
                         offset= self.size + num3 as u64;
                     }

                }


            }
        }


        //set the file to the offset
        self.current_offset = offset;     
        Ok(offset)



        
    }

   
}

impl<HANDLE:VFatHandle> traits::File for File<HANDLE> {
    // add code here

     /// Writes any buffered data to disk.
    fn sync(&mut self) -> io::Result<()> {
        unimplemented!("read only file system")
    }

    /// Returns the size of the file in bytes.
    fn size(&self) -> u64 {
        self.size
    }
}


impl<HANDLE: VFatHandle> io::Read for File<HANDLE> {

    fn read(&mut self, buf: &mut [u8])-> io::Result<usize> {
        


        if self.size == 0 {
            return Ok(0);
        }


        let mut storage = Vec::new();
        

        self.vfat.lock(|vfat| vfat.read_chain(self.start_cluster,&mut storage))?;

        //     println!("Inside Files Read Function in Read Chain File Size{:?}", self.size);
        //     println!("Inside Files Read Function in Read Chain File Size{:?}", self.current_offset);


        let current_bytes = self.size - self.current_offset;



        let bytesread = if current_bytes > buf.len() as u64 {
            buf.len()
        }  else {
            current_bytes as usize
        };



        buf[..bytesread as usize].copy_from_slice(&storage[self.current_offset as usize..self.current_offset as usize + bytesread as usize]);
        

        self.current_offset += bytesread as u64;



        Ok(bytesread as usize)

    
    }

}



impl<HANDLE: VFatHandle> io::Write for File<HANDLE> {

    fn write(&mut self, buf:&[u8]) -> io::Result<usize> {
        unimplemented!("read only file system")
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!("read only file system")
    }


}



