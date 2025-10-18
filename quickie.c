#include <H5Tpublic.h>
#include <cstdio>
#include <hdf5.h>


int main() {
  H5open();
  printf("%lld\n", (long long)H5T_NATIVE_DOUBLE_g);
  return 0;
}