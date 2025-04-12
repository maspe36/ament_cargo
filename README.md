# ament_cargo
A build script dependency for registering Rust crates as ROS 2 packages in the Ament index.

The associated package needs to have a package.xml. Also, there are some additional changes required for colcon-cargo 
to set environment variables that we would use within the macro. Still a WIP.

## Example
build.rs
```rust
use ament_cargo::ament_package;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    
    ament_package!();
}
```

Ideally, this is published to crates.io. This shouldn't need to be vendored in a workspace.