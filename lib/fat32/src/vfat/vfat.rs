use core::fmt::Debug;
use core::marker::PhantomData;
use core::mem::size_of;
use core::str;

use alloc::vec::Vec;
use alloc::string::String;


use shim::io;
use shim::ioerr;
use shim::newioerr;
use shim::path;
use shim::path::Path;
use shim::path::Component;

use crate::mbr::MasterBootRecord;
use crate::traits::{BlockDevice, FileSystem};
use crate::util::SliceExt;
use crate::vfat::{BiosParameterBlock, CachedPartition, Partition};
use crate::vfat::{Cluster, Dir, Entry, Error, FatEntry, File, Status};

/// A generic trait that handles a critical section as a closure
pub trait VFatHandle: Clone + Debug + Send + Sync {
    fn new(val: VFat<Self>) -> Self;
    fn lock<R>(&self, f: impl FnOnce(&mut VFat<Self>) -> R) -> R;
}

#[derive(Debug)]
pub struct VFat<HANDLE: VFatHandle> {
    phantom: PhantomData<HANDLE>,
    device: CachedPartition,
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    sectors_per_fat: u32,
    fat_start_sector: u64,
    data_start_sector: u64,
    pub rootdir_cluster: Cluster,
}

impl<HANDLE: VFatHandle> VFat<HANDLE> {
    pub fn from<T>(mut device: T) -> Result<HANDLE, Error>
    where
        T: BlockDevice + 'static,
    {

        //why phantom data

        //bytes per sector: bpb
        //sector per cluster: bpb
        // sectors per fat: bpb
        // 

        let mbr = MasterBootRecord::from(&mut device)?;

        //get the bpb partition adrress
        let bpb = &mbr.get_partition().unwrap()[0];

        if !bpb.is_fat32() {
            return Err(Error::Io(io::Error::new(io::ErrorKind::UnexpectedEof, "Device did not read 512 bytes")));
        }



        //relative offset, offset in sectors from the start of disk to the partition

        let bpb_sector = bpb.get_relative_sector() as u64;

        let ebpb = BiosParameterBlock::from(&mut device, bpb_sector)?;


        let bytes_per_sector = ebpb.get_bytes_per_sector();
        let rootdir = ebpb.get_root_cluster();
        let sect_per_fat = ebpb.get_sector_per_fat();
        let sect_per_cluster = ebpb.get_sector_per_cluster();

        //fat start sector

        //offset of fat from ebpb
        let number_of_reserve_sec = ebpb.get_reserved_sector();
        let fat_start = number_of_reserve_sec as u64;

         


        //data start sector 

        //number of fats * size of fats + fat offset = first address of data region
        //sector per fact *  32 bytes of 16 bytes + fat_sart
        let data_start = ebpb.get_number_FAT() as u64 * sect_per_fat as u64 + fat_start;

        //number of sectors in parition = sectors of fat + sectors of clutster
        let mut num12 = ebpb.get_total_sector() as u64;
        if num12 <= 65535 {
            num12 = ebpb.logical_sector() as u64;
            if num12 == 0 {
                num12 = ebpb.get_total_sector() as u64;
            }
        }




        let partition1 = Partition {
            start: bpb_sector as u64,
            num_sectors: num12 as u64,
            sector_size: bytes_per_sector as u64,

        };


        let cache_partition = CachedPartition::new(device, partition1);


        Ok(VFatHandle::new(VFat{
            phantom: PhantomData,
            device: cache_partition,
            bytes_per_sector: bytes_per_sector,
            sectors_per_cluster: sect_per_cluster,
            sectors_per_fat: sect_per_fat,
            fat_start_sector: fat_start,
            data_start_sector: data_start as u64,
            rootdir_cluster: Cluster::from(rootdir),

        }))




        
    }

    // TODO: The following methods may be useful here:
    //
    //  * A method to read from an offset of a cluster into a buffer.
    //  read the bluster 
    pub fn read_cluster(
           &mut self,
           cluster: Cluster,
           offset: usize,
           buf: &mut [u8]
       ) -> io::Result<usize> {
        
        //check for the valid of cluster number 
        if cluster.get_clusterValue() < 2 {
            //println!("Cluster Value ERRor: {:?}", cluster.get_clusterValue() );
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cluster is less than 2"));
        }

     

        //println!("Inside Cluster  Reading  Function {:?}", "Hello");
        let cluster_start = self.data_start_sector + ((cluster.get_clusterValue()-2) as u64) * self.sectors_per_cluster as u64;





        // // cluster
        // //gets the sector 

        let mut sector = offset / self.bytes_per_sector as usize;

        let mut byte_offset = offset as u64 % self.bytes_per_sector as u64;


        let bytes_left:u64 = self.sectors_per_cluster as u64 * self.bytes_per_sector as u64  - offset as u64;

        //get the entry of the of fat32 which contains the next 



        //number of bytes to read from the sector 
        let mut bytesread:u64 = 0;

        //buf len or whole cluster 

        //read all the sectors in the cluster 
        let mut bytes_need:u64 = 0;

        // bytes need to be read 
        if (buf.len() as u64) < bytes_left {
            bytes_need = buf.len() as u64;
        } else {
            bytes_need = bytes_left;
        }



        loop {


            //current sector
            //let mut sector = sector_offset + (bytesread / self.bytes_per_sector as usize); 


            //exit the loop
            if (sector >= self.sectors_per_cluster as usize) || bytesread >= bytes_need{
                break;
            } else {

            //current byte offset
            //
            //read from a sector
           //println!("Cluster Start + sector {:?}", cluster_start + sector as u64);
            //println!("Cluster Start + sector times bytes per sector {:?}", (cluster_start + sector as u64)* self.bytes_per_sector as u64);


            let value:&[u8] = self.device.get(cluster_start + sector as u64)?; 


            let end = if bytes_need - bytesread <  self.bytes_per_sector as u64 - byte_offset {
                bytes_need - bytesread

            } else {
                self.bytes_per_sector as u64 - byte_offset
            };

            //println!("Byte Offset Value + end {:?}", byte_offset + end);
            sector+=1;


            //size of return
            let mut sector_size = value.len();
            //println!("Value Read Size: {:?}", sector_size);

            //copy to the buf
            buf[bytesread as usize..(bytesread + end) as usize].copy_from_slice(&value[byte_offset as usize..(byte_offset+ end) as usize]);

            //println!("Buffer Length: {:?}", buf.len());

            //add the number of bytes read
            bytesread += end;
            //sector = sector  + (bytesread / self.bytes_per_sector as u64) as usize;
            byte_offset = (offset as u64 + bytesread) % self.bytes_per_sector as u64;

           // println!("Total Bytes Read: {:?}", bytesread);
            //println!("Sector Value {:?}", sector);
            //println!("Byte Offset Value {:?}", byte_offset);





            }
           
        
        }


        //return bytes copy
        Ok(bytesread as usize) 
   }
    


    //  * A method to read all of the clusters chained from a starting cluster
    //    into a vector.
    //
      pub fn read_chain(
           &mut self,
           start: Cluster,
           buf: &mut Vec<u8>
       ) -> io::Result<usize> {

        //so we have cluster starting, we read all the sectors from the cluster

        //see the fat entry table -- find the next cluster, repeat the read

        //break on Eoc, or invalid reserve, bad, status

        //println!("Inside Cluster  Reading  Chaining  Function {:?}", start.get_clusterValue());


        if start.get_clusterValue() < 2 {
             //println!("Cluster value in read chain: {:?}", start.get_clusterValue() );

            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cluster is less than 2"));
        }


        //cluster
        //gets the sector 
        let cluster_start = self.data_start_sector + (start.get_clusterValue() as u64) * self.sectors_per_cluster as u64;


        //4
        let mut cluster_current = start; 

        let mut bytesread = 0;
       
        let mut number_of_clusters = 0;
         

        loop {
            //
            number_of_clusters+=1;

            let mut newwrite_size = (number_of_clusters*self.sectors_per_cluster as u64 * self.bytes_per_sector as u64) as usize;
            
            buf.resize(newwrite_size, 0);

          
              //read 4
            let mut byte_size = self.read_cluster(cluster_current, 0, &mut buf[bytesread..])?;
            

            bytesread += byte_size as usize;

            match self.fat_entry(cluster_current)?.status() {
                Status::Data(x) => {

                    cluster_current = x;
                    //println!("Current Cluster {:?}", x );
                },
                Status::Eoc(y) => {
                    //byte_size = self.read_cluster(Cluster::from(cluster_current as u32), 0, &mut buf[bytesread..])?;
                    //bytesread += byte_size as usize;
                    return Ok(bytesread);
                    break;
                },
                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Fat entry determine invalid entry"));
                    //mabye Err
                    break;
                }

            }
           

        }

        Ok(bytesread)


 


    
       }



      pub fn read_chain_offset(
           &mut self,
           start: Cluster,
           offset: u64,
           buf: &mut Vec<u8>
       ) -> io::Result<usize> {

        //so we have cluster starting, we read all the sectors from the cluster

        //see the fat entry table -- find the next cluster, repeat the read

        //break on Eoc, or invalid reserve, bad, status


        // cluster
        //gets the sector 

        //call cluster

        //then call cluster chaiin
        
        if start.get_clusterValue() < 2 {
             //println!("Cluster value in read chain: {:?}", start.get_clusterValue() );

            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Cluster is less than 2"));
        }

        //println!("Cluster value in read chain offset: {:?}", start.get_clusterValue() );

        //cluster
        //gets the sector 
        let cluster_start = self.data_start_sector + (start.get_clusterValue() as u64) * self.sectors_per_cluster as u64;


        //4
        let mut cluster_current = start; 


        
        let bytes_read_from_offset = 0; 

        let mut bytesread = 0;
       
        let mut number_of_clusters = 0;
         

        loop {
            //
           

          
              //read 4
            let mut byte_size = self.read_cluster(cluster_current, 0, &mut buf[bytesread..])?;
            number_of_clusters+=1;
            let mut newwrite_size = (number_of_clusters*self.sectors_per_cluster as u64 * self.bytes_per_sector as u64) as usize;

            bytesread += byte_size as usize;

            match self.fat_entry(cluster_current)?.status() {
                Status::Data(x) => {

                    cluster_current = x;
                    buf.resize(newwrite_size, 0);

                },
                Status::Eoc(y) => {
                    //byte_size = self.read_cluster(Cluster::from(cluster_current as u32), 0, &mut buf[bytesread..])?;
                    //bytesread += byte_size as usize;
                    return Ok(bytesread);
                    break;
                },
                _ => {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Fat entry determine invalid entry"));
                    //mabye Err
                    //break;
                }

            }
           

        }

        Ok(bytesread)


    
       }

    //
    //  * A method to return a reference to a `FatEntry` for a cluster where the
    //    reference points directly into a cached sector.
    //     
       pub fn fat_entry(&mut self, cluster: Cluster) -> io::Result<&FatEntry> {

        //find the cluster 
        //cluster number
        let clusternum = cluster.get_clusterValue();

        //map cluster 

        
        let (fat_sector_num, fat_entry_offset) = self.map_cluster_entry(clusternum);

        //have the cluster number 

        //get the logical sector specified by the ebpb to physical sectors 
        //virtual to physical
        //logical sector number 
        //println!("{:?}", "here" );
    
        let value = self.device.get(fat_sector_num)?;

        //let f_entry: &[FatEntry] = unsafe{value.cast()};


        Ok(unsafe {&value[fat_entry_offset as usize..(fat_entry_offset as usize+4)].cast()[0]})

       }


       pub fn map_cluster_entry(&self, num: u32)-> (u64, u64) {
           

            let fatsecnum =  self.fat_start_sector + ((num) as u64 * 4) / (self.bytes_per_sector as u64);   
            let fatentryoffset = ((num) as u64 *4) % self.bytes_per_sector as u64;
            (fatsecnum, fatentryoffset)
       }




}

impl<'a, HANDLE: VFatHandle> FileSystem for &'a HANDLE {
    type File = File<HANDLE>;
    type Dir = Dir<HANDLE>;
    type Entry = Entry<HANDLE>;

    fn open<P: AsRef<Path>>(self, path: P) -> io::Result<Self::Entry> {
        
        use crate::traits::Entry;
        use crate::vfat::Metadata;
        //get the root cluster
        let path1 = path.as_ref().components();


        if !path.as_ref().is_absolute() {
            return Err((io::Error::new(io::ErrorKind::InvalidInput,"Not a Absolute Path")));
        }

        let root_dir = self.lock(|vfat| vfat.rootdir_cluster);

        //have the root directory, taverse from the root
        //need the root entry
        //println!("File System Path Components # {:#?}", root_dir);

        

        let mut current_dir = self::Entry::Dir(
            Dir {
                    vfat: self.clone(),
                     // FIXME: Fill me in.
                     //first cluster
                    start_cluster: root_dir,
                    name: String::from("/"),
                    metadata: Metadata::default(),
            }); 


        for components in path1 {

            match components {



                    // A normal component, e.g., `a` and `b` in `a/b`.
                    //
                    // This variant is the most common one, it represents references to files
                    // or directories.
                    Component::Normal(file_dir)=> {
                        
                        // current_dir = match current_dir.as_dir() {
                        //     Some(direct) {
                        //         direct    
                        //     },
                        //     None => {return Err(io::Error::new(io::ErrorKind::NotFound, "File Not Found"));},
                        //     _ => {return Err(io::Error::new(io::ErrorKind::NotFound, "File Not Found"));},

                        // };
                        //println!("Normal File Component {:?}", components.as_os_str());


                        current_dir = current_dir.into_dir().ok_or(io::Error::new(io::ErrorKind::NotFound, "File not Found"))?.find(file_dir)?;

                    

                        },
                        _=> {},

                    }



        }

        
        Ok(current_dir)       


    }
}
