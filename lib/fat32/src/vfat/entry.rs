use crate::traits;
use crate::vfat::{Dir, File, Metadata, VFatHandle};
use core::fmt;

// You can change this definition if you want
#[derive(Debug)]
pub enum Entry<HANDLE: VFatHandle> {
    File(File<HANDLE>),
    Dir(Dir<HANDLE>),
}

// TODO: Implement any useful helper methods on `Entry`.

impl<HANDLE: VFatHandle> traits::Entry for Entry<HANDLE> {
    // FIXME: Implement `traits::Entry` for `Entry
    type File = File<HANDLE>;
    type Dir = Dir<HANDLE>;
    type Metadata = Metadata;

    fn name(&self) -> &str {


    	match self {
    		Entry::File(x) => {
    			return &x.name;
    		},
    		Entry::Dir(y) => {
    			return &y.name;
    		}


    	}

    }

    fn metadata(&self) -> &Self::Metadata {
    	match self {
    		Entry::File(x) => {
    			return &x.metadata;
    		},
    		Entry::Dir(y) => {
    			return &y.metadata;
    		}


    	}
      
    }

    fn as_file(&self) -> Option<&File<HANDLE>> {
        match self {
    		Entry::File(x) => {
    			return Some(&x);
    		},
    		_ => {
    			return None;
    		}


    	}
    }
    fn as_dir(&self) -> Option<&Dir<HANDLE>> {
    	match self {
    		Entry::Dir(x) => {
    			return Some(&x);
    		},
    		_ => {
    			return None;
    		}


    	}
        
    }
    fn into_file(self) -> Option<File<HANDLE>> {
       match self {
    		Entry::File(x) => {
    			return Some(x);
    		},
    		_ => {
    			return None;
    		}


    	}
    }
    fn into_dir(self) -> Option<Dir<HANDLE>> {
        match self {
    		Entry::Dir(x) => {
    			return Some(x);
    		},
    		_ => {
    			return None;
    		}


    	}
    }





}
