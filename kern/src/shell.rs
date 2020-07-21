use shim::io;
use shim::path::{Path, PathBuf};

use stack_vec::StackVec;
use alloc::string::String;
use alloc::vec::Vec;
use pi::atags::Atags;

use fat32::traits::FileSystem;
use fat32::traits::{Dir, Entry, Metadata};
use fat32::traits::File;
//use fat32::traits::File::Read;
//use fat32::vfat::file;
use shim::io::Read;

use crate::console::{kprint, kprintln, CONSOLE};
use crate::ALLOCATOR;
use crate::FILESYSTEM;

use pi::timer::spin_sleep;
use core::time::Duration;
use kernel_api::syscall::sleep;
use kernel_api::{OsError, OsResult};


use crate::alloc::string::ToString;


/// Error type for `Command` parse failures.
#[derive(Debug)]
enum Error {
    Empty,
    TooManyArgs,
}

/// A structure representing a single shell command.
struct Command<'a> {
    args: StackVec<'a, &'a str>,
}

impl<'a> Command<'a> {
    /// Parse a command from a string `s` using `buf` as storage for the
    /// arguments.
    ///
    /// # Errors
    ///
    /// If `s` contains no arguments, returns `Error::Empty`. If there are more
    /// arguments than `buf` can hold, returns `Error::TooManyArgs`.
    fn parse(s: &'a str, buf: &'a mut [&'a str]) -> Result<Command<'a>, Error> {
        let mut args = StackVec::new(buf);
        for arg in s.split(' ').filter(|a| !a.is_empty()) {
            args.push(arg).map_err(|_| Error::TooManyArgs)?;
        }

        if args.is_empty() {
            return Err(Error::Empty);
        }

        Ok(Command { args })
    }

    /// Returns this command's path. This is equivalent to the first argument.
    fn path(&self) -> &str {
        self.args[0]
    }


    fn first(&self) -> &str {
        self.args[1]
    }
}

/// Starts a shell using `prefix` as the prefix for each line. This function
/// never returns.
pub fn shell(prefix: &str) {
    //unimplemented!();
    //spin_sleep(Duration::new(5,0));

    let bell:u8 = 0x7;
    let backspace: u8    = 8;
    let delete: u8   = 127;

        

    let mut cwd = PathBuf::from("/");

    
    
    // /r /n
    //loop through entire script
    loop {
        let mut storage = [0u8; 512];
        let max_length = storage.len();
        let mut stack = StackVec::new(&mut storage);




    
        //let current_length = 0;
        
         // prints > prefix
        kprint!("({}) " ,cwd.to_str().unwrap()); 
        kprint!("{} ", prefix);
         
        //let mut console1 = CONSOLE.lock();
        //console.write_byte(b'\n');
        //loop through each line

        let mut exitin = false;


        loop{
    
                let mut console = CONSOLE.lock();
                let input_byte = console.read_byte();
                //let x = CONSOLE.lock();
                //console.write_byte(input_byte);
                //debug //kprint
                if input_byte == b'\r' || input_byte == b'\n' {
                    //enter
                    let mut str_buffer: [&str; 64] = ["";64];
                    match Command::parse(core::str::from_utf8(stack.into_slice()).unwrap(), &mut str_buffer) {
                        Ok(a) => {
                            if a.path() == "echo" {
                                //kprintln!("{:?}",a.args[1..args.len]);
                                let mut x  = 1;
                                 kprint!("{}","\r\n");
                                for each in a.args {
                                    if x==1{
                                        x=2;
                                        continue;
                                    }
                                    kprint!("{} ", each);
                                }
                                kprint!("{}","\r\n");
                            } else if a.path() =="sleep" {
                                kprint!("{}","\r\n");

                                if a.args.len() != 2 {
                                    kprintln!("Incorrect");

                                    break
                                }


                                match a.args[1].parse::<u64>() {
                                    Ok(num) => {
                                        let error1 = sleep(Duration::from_millis(num as u64));
                                            match error1 {

                                                Ok(_)=> {
                                                    kprintln!("{}", "Sleep function success");

                                                },
                                                _=> {
                                                    kprintln!("{:?}", error1);
                                                }
                                            }
                                    },
                                    Err(e)=> {

                                        kprintln!("{}", "Input value cannot convert to number.");

                                    },
                                };

                                

                            }



                            else if a.path() == "exit" {
                                kprint!("{}","\r\n");
                                kprintln!("Exiting");
                                exitin = true;
                                break


                            }else if a.path() == "pwd" {

                                //let file = FILESYSTEM.open();

                                //access 
                                //list all the entries in the filesystem
                                kprint!("{}","\r\n");


                                kprint!("{}", cwd.to_string_lossy());


                                kprint!("{}","\r\n");
                            

                            } else if a.path() == "cd" {

                                let  cwd_reserve = cwd.clone();

                                if a.args.len() > 2 {
                                    kprint!("{}","\r\n");


                                    kprint!("{:?}", "bash: cd to many arguments");


                                    kprint!("{}","\r\n");
                                }else  {

                                //kprint!("{:?}", "bash: cd to many arguments");

                                    

                                
                                    if a.args.len() == 1 {
                                        cwd.push("/");
                                    } else if a.args[1] == ".." {
                                        cwd.pop();

                                    } else if a.args[1] == "/" {
                                        cwd.push("/");
                                    } else if a.args[1] == "." {
                                        //kprint!("{:?}", "into here .");


                                        cwd = cwd;
                                
            
                                    } else  {
                                        cwd.push(a.args[1]);
                                        //kprint!("{:?}", a.args[1]);

                                    }

                                    //kprint!("{:?}", cwd.to_str().unwrap());

                                    //let entry = ;

                                     //kprint!("{:?}", "into here vefore match");
                                     let mut is_dir = true;
                                     let newcwd = match FILESYSTEM.open(cwd.clone()).ok() {
                                        Some(x) => {
                                            match x.as_dir() {
                                                Some(b) => {
                                                    cwd
                                                },
                                                None=> {
                                                    kprint!("{}","\r\n");
                                                     kprint!("{:?}", "bash: cd argument invalid");
                                                    kprint!("{}","\r\n");
                                                    is_dir=false;
                                                    cwd_reserve
                                                },

                                            }    
                                        
                                        },
                                        None=> {
                                            kprint!("{}","\r\n");
                                            kprint!("{:?}", "bash: cd argument invalid");
                                            kprint!("{}","\r\n");
                                            is_dir=false;
                                            cwd_reserve

                                        },
                                    };

                                cwd = newcwd;

                                if is_dir {
                                //kprint!("{}","\r\n");


                                //kprint!("{}", cwd.to_string_lossy());


                                kprint!("{}","\r\n");
                                }
                            }

                            



                        } else if a.path() == "ls" {

                                //list all the files
                                kprint!("{}","\r\n");



                                //let x = FILESYSTEM.open(cwd.clone().into_os_string()).unwrap();

                            


                                if a.args.len() == 2 && a.args[1] == "-a" {

                                    //show with the hidden files 

                                    //metadata
                                    let mut newpath = cwd.clone();

                                    if a.args.len() == 3 {
                                        //there is a directory
                                        newpath.push(a.args[2]);
                                    
                                    } 


                                     //kprintln!("{}", "LS in hidden fields");

                                    let x1 = match FILESYSTEM.open(newpath.clone()).ok() {
                                            Some(x) => {
                                                match x.into_dir() {
                                                    Some(y) => {
                                                         for each in y.entries().unwrap() {
                                                            if each.metadata().attributes.hidden() {
                                                                kprintln!("{:?}", each.name());

                                                            }
                                                        }
                
                                                    },
                                                    None => {
                                                        kprintln!("{}", "Invalid ls command for not a directory.");

                                                    },
                                                }
                                            
                                               

                                            },
                                            None => {
                                                kprintln!("{}", "Invalid ls command for not a directory.");
                                            },
                                    }; 


                                }  else {

                                    let mut newpath = cwd.clone();

                                    if a.args.len()==2 {
                                        

                                        newpath.push(a.args[1]);   
                                    } 


                                  //kprintln!("{}", "LS Before the file open in 2 argument");


                                    let x1 = match FILESYSTEM.open(newpath.clone()).ok() {
                                            Some(x) => {
                                                //kprintln!("{}", "into directory");

                                                match x.into_dir() {
                                                    Some(y) => {
                                                         for each in y.entries().unwrap() {
                                                            if !each.metadata().attributes.hidden() {
                                                                kprintln!("{:?}", each.name());

                                                            }
                                                        }
                
                                                    },
                                                    None => {
                                                        kprintln!("{}", "Invalid ls command for not a directory.");

                                                    },
                                                }
                                            
                                               

                                            },
                                            None => {
                                                kprintln!("{}", "Invalid ls command for not a directory.");
                                            },
                                    }; 
                                }



                            } else if a.path() == "cat" {
                                kprint!("{}","\r\n");


                                //kprintln!("{}", "At least a file argument is required.");



                                if a.args.len() < 2 {

                                    
                                    kprint!("{}", "At least a file argument is required.");


                                } else {

                                    for i in 1..a.args.len() {
                                        let each = a.args[i];
                                        let mut newpath = cwd.clone();
                                        newpath.push(each);


                                        let each2 = PathBuf::from(each); 
                                        if each2.is_absolute() {
                                            newpath = PathBuf::from(each);
                                        } 

                                        match FILESYSTEM.open(newpath.clone()).ok() {
                                            Some(cx) => {
                                                match cx.into_file() {
                                                     Some(mut x) => {
                                                            
                                                            //let mut vec = Vec::new();
                                                            let x1 = x.size() as u64;
                                                            let mut brake = false; 
                                                            loop {
                                                            let mut buf = [0u8; 512];
                                                            use fat32::traits::File;
                                                            //kprint!("{}", "Into the Loop Here");
                                                            //let x12:() = x;
                                                            let sizeread = match x.read(&mut buf) {
                                                                Ok(x) => {

                                                                    let mut counter = 0;
                                                        

                                                                     for each in buf.iter(){
                                                                             if *each == 0 {
                                                                                    brake = true;
                                                                                    break;
                                                                              }
                                                                        counter+=1;
                                                                    }




                                                                    let string1 = String::from_utf8(buf[..counter].to_vec()).unwrap_or("Invalid UTF-8 strings".to_string());

                                                                    kprint!("{:?}", string1);

                                                                    if brake {
                                                                        break;
                                                                    }

                                                                    x

                                                                },
                                                                Err(_)=> {
                                                                    kprint!("{:?}", "File cannot be read.");
                                                                    0

                                                                },

                                                            };
                                                        }

                                                           
                                                    },
                                                    None => {
                                                             continue;
                                                    },

                                                }
                                            },
                                            None => {
                                                continue;
                                            },

                                        }


                                    }




                                }



                            //kprint!("{}", "Into the Loop Here");



                            kprint!("{}","\r\n");





                            } else {
                                kprint!("{}","\r\n");
                                kprintln!("unknown command: {}", a.path());

                            }


                            break;
                        },
                        Err(Error::TooManyArgs) => {
                            kprint!("{}","\r\n");
                            kprintln!("{}","error: too many arguments");
                            break;
                        },
                        Err(Error::Empty) => {
                            kprint!("{}","\r\n");
                            break;
                        }


                    }
    
                } else if input_byte == delete || input_byte == backspace {

                    match stack.pop() {
                        None => {
                            kprint!("{}",bell as char);    
                        }
                        Some(a) => {
                            kprint!("{}{}{}", "\u{8}"," ", backspace as char);
                        }
                    };
                    



                } else {
                    

                    if input_byte > 127 {
                            kprint!("{}",bell as char);    
                    } else {
                        
                        match stack.push(input_byte) {
                            Ok(())=> {  

                                kprint!("{}",input_byte as char);    

                            }, 
                            Err(())=> {
                                kprint!("{}",bell as char);    
                                    
                            },
                        }
                    }
                }
            }
            if exitin {
            kprintln!("{:?}", exitin);
            return
            }
    
        }
        
    
    
    }
    

