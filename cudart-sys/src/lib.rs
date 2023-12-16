#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod path;

use std::backtrace::Backtrace;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{Debug, Display, Formatter};
use std::mem::MaybeUninit;

include!("bindings.rs");

impl CudaError {
    pub fn eprint_error(self) {
        if self != CudaError::Success {
            eprintln!("CUDA Error: {self}");
        }
    }

    pub fn eprint_error_and_backtrace(self) {
        if self != CudaError::Success {
            self.eprint_error();
            let backtrace = Backtrace::capture();
            eprintln!("Backtrace: {backtrace}");
        }
    }
}

impl Display for CudaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = unsafe { CStr::from_ptr(cudaGetErrorName(*self)) };
        name.fmt(f)
    }
}

impl Error for CudaError {}

impl Default for CudaMemPoolProperties {
    fn default() -> Self {
        let mut s = MaybeUninit::<Self>::uninit();
        unsafe {
            std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
