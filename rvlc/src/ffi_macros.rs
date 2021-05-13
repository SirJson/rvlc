#[macro_export]
macro_rules! maybe_null {
    ($ptr:expr) => {
        if $ptr.is_null() {
            return None;
        }
    };
}

#[macro_export]
macro_rules! not_null {
    ($ptr:expr, $what:expr) => {
        use crate::internal_error::VLCError;
        if $ptr.is_null() {
            return Err(VLCError::NullPointer($what.to_owned()));
        }
    };
}

#[macro_export]
macro_rules! cstring_opt {
    ($val:expr) => {
        match (std::ffi::CString::new($val.to_owned())) {
            Ok(v) => v,
            Err(_) => {
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! expect_cstring {
    ($val:expr) => {
        match (std::ffi::CString::new($val.to_owned())) {
            Ok(v) => v,
            Err(_) => {
                return Err(VLCError::StrFFI);
            }
        }
    };
}

#[macro_export]
macro_rules! expect_cpath {
    ($val:expr) => {
        match ($val.to_str()) {
            Some(s) => match (std::ffi::CString::new(s)) {
                Ok(v) => v,
                Err(_) => { return Err(VLCError::PathFFI); }
            },
            None => { return Err(VLCError::PathFFI); }
        }
    };
}

#[macro_export]
macro_rules! cpath_opt {
    ($val:expr) => {
        match ($val.to_str()) {
            Some(s) => match (std::ffi::CString::new(s)) {
                Ok(v) => v,
                Err(_) => {
                    return None;
                }
            },
            None => {
                return None;
            }
        }
    };
}
