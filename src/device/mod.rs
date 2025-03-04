pub mod extensions;
pub mod input;
pub mod output;

use std::marker::PhantomData;
use std::str::from_utf8_unchecked;
use std::{
    ffi::{CStr, CString},
    ptr,
};

use ffi::*;

use crate::Error;

use self::extensions::DeviceIter;

pub struct Info<'a> {
    ptr: *mut AVDeviceInfo,

    _marker: PhantomData<&'a ()>,
}

impl<'a> Info<'a> {
    pub unsafe fn wrap(ptr: *mut AVDeviceInfo) -> Self {
        Info {
            ptr,
            _marker: PhantomData,
        }
    }

    pub unsafe fn as_ptr(&self) -> *const AVDeviceInfo {
        self.ptr as *const _
    }

    pub unsafe fn as_mut_ptr(&mut self) -> *mut AVDeviceInfo {
        self.ptr
    }
}

impl<'a> Info<'a> {
    pub fn name(&self) -> &str {
        unsafe { from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_name).to_bytes()) }
    }

    pub fn description(&self) -> &str {
        unsafe {
            from_utf8_unchecked(CStr::from_ptr((*self.as_ptr()).device_description).to_bytes())
        }
    }
}

pub fn register_all() {
    unsafe {
        avdevice_register_all();
    }
}

pub fn version() -> u32 {
    unsafe { avdevice_version() }
}

pub fn configuration() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_configuration()).to_bytes()) }
}

pub fn license() -> &'static str {
    unsafe { from_utf8_unchecked(CStr::from_ptr(avdevice_license()).to_bytes()) }
}

pub fn sources(device_name: &str) -> Result<DeviceIter, Error> {
    unsafe {
        let mut ptr: *mut AVDeviceInfoList = ptr::null_mut();
        let string = CString::new(device_name).map_err(|_| Error::InvalidData)?;

        match avdevice_list_input_sources(
            ptr::null_mut(),
            string.as_ptr(),
            ptr::null_mut(),
            &mut ptr,
        ) {
            n if n < 0 => Err(Error::from(n)),

            _ => Ok(ptr.into()),
        }
    }
}

pub fn sinks(device_name: &str) -> Result<DeviceIter, Error> {
    unsafe {
        let mut ptr: *mut AVDeviceInfoList = ptr::null_mut();
        let string = CString::new(device_name).map_err(|_| Error::InvalidData)?;

        match avdevice_list_output_sinks(
            ptr::null_mut(),
            string.as_ptr(),
            ptr::null_mut(),
            &mut ptr,
        ) {
            n if n < 0 => Err(Error::from(n)),

            _ => Ok(ptr.into()),
        }
    }
}
