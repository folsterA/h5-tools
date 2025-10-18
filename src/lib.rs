#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;

    #[test]
    fn round_trip_h5() {
        let file_name = CString::new("../test.h5").unwrap();
        unsafe {
            let err = H5Fopen(file_name.as_ptr(), 0, H5P_DEFAULT.into());
            assert!(err == 0);
        }
    }
}
