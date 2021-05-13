use std::ffi::CString;

use crate::internal_error::VLCResult;
use crate::not_null;
use crate::traits::PointerAccess;
use libvlc_sys::*;
use std::fmt::Debug;

type VLCPointer = *mut libvlc_instance_t;

#[derive(Debug)]
pub struct VLCInstance(VLCPointer);

#[derive(Debug)]
pub enum VLCInterface {
    Dummy,
    RC,
}

impl VLCInstance {
    pub fn new() -> VLCResult<VLCInstance> {
        let ptr = unsafe { libvlc_new(0, std::ptr::null()) };
        not_null!(ptr, "VLC init");
        Ok(VLCInstance(ptr))
    }

    pub fn with_interface(self, interface_type: VLCInterface) -> Self {
        let magicstr = match interface_type {
            VLCInterface::Dummy => "dummy",
            VLCInterface::RC => "rc",
        };
        unsafe {
            let name = CString::new(magicstr.to_owned()).unwrap();
            let result = libvlc_add_intf(self.0, name.as_ptr());
            log::debug!("Dummy interface result = {}", result);
        }
        self
    }
}

impl Drop for VLCInstance {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                libvlc_release(self.0);
            }
        }
    }
}

impl PointerAccess<VLCPointer> for VLCInstance {
    fn get_ptr(&self) -> VLCPointer {
        self.0
    }
}
