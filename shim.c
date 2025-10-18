#define HDF5_SHARED_LIBS 0
#define H5_BUILT_AS_STATIC_LIB
#include <hdf5.h>

hid_t H5T_NATIVE_DOUBLE_r(void) { return H5T_NATIVE_DOUBLE; }
