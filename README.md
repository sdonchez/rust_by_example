# Rust By Example Tutorials
This repository contains source code and executables relating to the [Rust By Example][RBE] set of tutorials provided by the [Rust Team](https://www.rust-lang.org/). It's the result of my (in progress) efforts to learn Rust.

## Contents
This repository's structure mirrors the structure of the [Rust By Example][RBE] documentation. Primary Headings are indicated below. See the individual README page in each directory for detailed contents thereof.

### Directory Structure
Each top-level directory contains the following assets:
- `README.md` : Describes the contents of the top-level directory in more detail
- `src/`: contains the `.rs` source code files associated with the directory
- `bin/`: contains the executables corresponding to the source, as compiled with `rustc`.
- `log/`: contains the output produced by each of the binaries in `bin/`.

Note that this structure is subject to change in the future as complexity increases.

### Top-Level Directory Listing
A filled checkbox next to a directory indicates that the section is complete. An unfilled checkbox indicates that work is still in progress.

|Directory|Description|
|---|---|
|✔️ [Hello World](./01_hello_world/README.md)| Contains a basic "Hello World" Application, as well as various programs that demonstrate the concepts of comments and formatted print.|
|✔️ [Primitives](./02_primitives/README.md)| Contains applications that explore the various primitive types, as well as tuples, arrays, and slices.
|✔️ [Custom Types](./03_custom_types/README.md)| Contains applications that demonstrate the various types of custom datatypes supported by rust, including structures, enums, and constants.|
|✔️ [Variable Bindings](./04_variable_bindings/README.md)|Contains applications that explore variable mutability and scope.

<!--Links-->
[RBE]:(https://doc.rust-lang.org/stable/rust-by-example/index.html)