# ament_cargo
A build script dependency for registering Rust crates as ROS 2 packages in the Ament index.

## Example
build.rs
```rust
use ament_cargo::ament_package;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // CARGO_PKG_NAME: This is the name of _this_ cargo package, this is defined in the `Cargo.toml`
    // CARGO_MANIFEST_DIR: This is the path to the directory that holds the `Cargo.toml`
    ament_package(env!("CARGO_PKG_NAME"), env!("CARGO_MANIFEST_DIR"));
}
```