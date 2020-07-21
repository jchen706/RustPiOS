use core::fmt;

use alloc::string::String;

use crate::traits;



/// A date as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Date(u16);

/// Time as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Time(u16);

/// File attributes as represented in FAT32 on-disk structures.
#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct Attributes(u8);

/// A structure containing a date and time.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timestamp {
    pub date: Date,
    pub time: Time,
}

/// Metadata for a directory entry.
#[derive(Default, Debug, Clone)]
pub struct Metadata {
    // FIXME: Fill me in.
    pub created: Timestamp,
    pub accessed: Timestamp,
    pub modified: Timestamp,
    pub attributes: Attributes,

}	


impl Date {

	pub fn new (num: u16)-> Option<Date> {
		Some(Date(num))
	}



	fn year(&self) -> usize {
		((self.0 >> 9) as usize + 1980)   
		
	}

    /// The calendar month, starting at 1 for January. Always in range [1, 12].
    
    /// January is 1, Feburary is 2, ..., December is 12.
    fn month(&self) -> u8 {
    	((self.0 >> 5) & 0b0000000000001111) as u8
   
    }

    /// The calendar day, starting at 1. Always in range [1, 31].
    fn day(&self) -> u8 {
    	self.0 as u8 & !(!0<<5)
    }


}

impl Time {
	/// The 24-hour hour. Always in range [0, 24).

	pub fn new (num: u16)-> Option<Time> {
		Some(Time(num))
	}

    fn hour(&self) -> u8 {
    	(self.0 >> 11) as u8

    }

    /// The minute. Always in range [0, 60).
    fn minute(&self) -> u8 {
    	((self.0 >> 5) & 0b0000000000111111) as u8
    }

    /// The second. Always in range [0, 60).
    fn second(&self) -> u8 {
    	(self.0 as u8 & !(!0<<5)) * 2

    }


}

impl Attributes {

	pub fn new(num:u8)-> Attributes {
		Attributes(num)
	}

	pub fn read_only(&self) -> bool {
		self.0 & 0x01 ==0x01

    }

    pub fn hidden(&self) -> bool {
    	self.0 & 0x02 == 0x02
    }

    pub fn system(&self)-> bool {
    	self.0 & 0x04 ==0x04
    }

    pub fn volume_id(&self) -> bool {
    	self.0 & 0x08 ==0x08
    }

    pub fn directory(&self)-> bool {
    	self.0 & 0x10 == 0x10
    }

    pub fn archive(&self)-> bool {
    	self.0 & 0x20 == 0x20
    }

}

// FIXME: Implement `traits::Timestamp` for `Timestamp`.
impl traits::Timestamp for Timestamp {

    fn year(&self) -> usize {
		self.date.year() 
		
	}

    /// The calendar month, starting at 1 for January. Always in range [1, 12].
    
    /// January is 1, Feburary is 2, ..., December is 12.
    fn month(&self) -> u8 {
		self.date.month()   
    }

    /// The calendar day, starting at 1. Always in range [1, 31].
    fn day(&self) -> u8 {
    	self.date.day()
    }

	
    /// The 24-hour hour. Always in range [0, 24).
    fn hour(&self) -> u8 {
    	 self.time.hour()

    }

    /// The minute. Always in range [0, 60).
    fn minute(&self) -> u8 {
    	   self.time.minute()
    }

    /// The second. Always in range [0, 60).
    fn second(&self) -> u8 {
    	    self.time.second()

    }


}

impl Metadata {

	pub fn default()->Metadata {

		Metadata {
			created: Timestamp {
                            date: Date::new(0).unwrap(),
                            time: Time::new(0).unwrap(),
                       },
                        accessed: Timestamp {
                             date: Date::new(0).unwrap(),
                            time: Time::new(0).unwrap(),
                        },
                        modified: Timestamp {
                             date: Date::new(0).unwrap(),
                            time: Time::new(0).unwrap(),
                        },
                        attributes: Attributes::new(0x4)
		}

	}
		
		
	pub fn new(created: Timestamp, accessed: Timestamp, modified: Timestamp, attributes: Attributes) -> Metadata {
		Metadata {
			created: created,
			accessed: accessed,
			modified: modified,
			attributes: attributes,

		}
	}


}

impl traits::Metadata for Metadata {
	type Timestamp = Timestamp;
	// add code here
	  /// Whether the associated entry is read only.
    fn read_only(&self) -> bool {
    	self.attributes.read_only()
    }

    /// Whether the entry should be "hidden" from directory traversals.
    fn hidden(&self) -> bool {
    	self.attributes.hidden()
    }

    /// The timestamp when the entry was created.
    fn created(&self) -> Self::Timestamp {
    	self.created

    }

    /// The timestamp for the entry's last access.
    fn accessed(&self) -> Self::Timestamp {
    	self.accessed
    }

    /// The timestamp for the entry's last modification.
    fn modified(&self) -> Self::Timestamp {
    	self.modified
    }




}




impl fmt::Display for Timestamp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use traits::Timestamp;
		write!(f, "Month: {0}, Day:{1}, Year: {2}, Minutes: {3}, Seconds:{4}  ", 
		self.month(), self.day(), self.year(), self.minute(), self.second())
	}

}

impl fmt::Display for Metadata {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		use traits::Metadata;
		write!(f, "Attributes:  Read-Only: {0}, Hidden: {1},  , Modified:{2}, Created: {3}, Accessed: {4}  ", 
		self.read_only(), self.hidden(), self.modified,self.created, self.accessed)
	}

}



// FIXME: Implement `traits::Metadata` for `Metadata`.

// FIXME: Implement `fmt::Display` (to your liking) for `Metadata`.
