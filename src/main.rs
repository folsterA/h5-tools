use h5_tools::ffi;

fn main() {
    let double_type: ffi::hid_t;
    unsafe {
        // all of this is to encode a IEEE 754 type without pre-processor macros from HDF5...
        double_type = ffi::H5Tcreate(ffi::H5T_class_t_H5T_FLOAT, 8);
        ffi::H5Tset_ebias(double_type, 1023);
        ffi::H5Tset_norm(double_type, ffi::H5T_norm_t_H5T_NORM_IMPLIED);
        ffi::H5Tset_order(double_type, ffi::H5T_order_t_H5T_ORDER_LE);
        ffi::H5Tset_pad(
            double_type,
            ffi::H5T_pad_t_H5T_PAD_ZERO,
            ffi::H5T_pad_t_H5T_PAD_ZERO,
        );
        ffi::H5Tset_precision(double_type, 64);
        ffi::H5Tset_fields(double_type, 63, 52, 11, 0, 52);
    }
    println!("{double_type}");

    let new_double_type = unsafe { ffi::H5T_NATIVE_DOUBLE_g };
    println!("{new_double_type}");
}
