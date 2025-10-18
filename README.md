# h5-tools

[![Crates.io](https://img.shields.io/crates/v/h5-tools.svg)](https://crates.io/crates/h5-tools)
[![Documentation](https://docs.rs/h5-tools/badge.svg)](https://docs.rs/h5-tools)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](./LICENSE)

**h5-tools** is an experimental Rust project aiming to make working with
[HDF5](https://www.hdfgroup.org/solutions/hdf5/) files more ergonomic and accessible.  
Right now this crate is a blank slate — development is just beginning — but the long-term goals are:

- **Safe Rust bindings** to the HDF5 C library  
- **Rustic abstractions** over the HDF5 C API, exposing a tree-like data model for structured access  
- **Command-line utilities** for manipulating HDF5 files:
  - Inspect file contents from the console
  - Stream data into new HDF5 files
  - Export subsets of data for external use

## TODO

- Try pkg-config for HDF5
- Download and build HDF5 if necessary

## Project Status

This project is in **early planning**. None of the above features are implemented yet.  
If you’re interested in contributing, experimenting, or shaping the direction of the crate, feedback
and pull requests are welcome!

## Installation

_Not yet published to crates.io._  
Once available, you’ll be able to add it to your `Cargo.toml`:

```toml
[dependencies]
h5-tools = "0.1"
```

## Usage

Examples will be added as functionality develops.

## Contributing

Contributions, ideas, and discussions are welcome!
If you’re familiar with HDF5, Rust FFI, or CLI design, your input would be especially valuable.

## License

This project is licensed under the terms of the GNU Affero General Public License v3.0 or later
(AGPL-3.0-or-later).  
See the [LICENSE](./LICENSE) file for details.

## Third-Party Licenses

This project bundles the [HDF5 library](https://www.hdfgroup.org/solutions/hdf5/),  
which is licensed separately under the terms described in [LICENSE-HDF5.txt](./LICENSE-HDF5.txt).

The HDF5 license is a permissive license that allows redistribution and use in source and binary
forms, with or without modification, provided that the copyright notice, conditions, and disclaimer
are retained.

Please note that the HDF5 library is **not** licensed under this project’s AGPL-3.0-or-later terms.
