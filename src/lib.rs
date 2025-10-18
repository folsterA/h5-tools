pub mod ffi {
    #![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(test)]
mod tests {
    use super::ffi;
    use std::ffi::CString;

    #[test]
    fn round_trip_h5() {
        let file_name = CString::new("../test.h5").unwrap();
        unsafe {
            ffi::H5Eset_auto2(ffi::H5E_DEFAULT.into(), None, std::ptr::null_mut());
            let err = ffi::H5Fopen(file_name.as_ptr(), 0, ffi::H5P_DEFAULT.into());
            assert!(err == 0);
        }
    }
}
