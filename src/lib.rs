use std::{env, io};
use std::fs;
use std::path::PathBuf;

/// Top level interface. This is the function we expect crate authors to add to their build.rs
/// if they want their crate registered in the ament index. Registering a crate in the ament index
/// requires a crate to have a `package.xml` and for cargo to be called from colcon. If the
/// `COLCON_WS_BASE` and/or `COLCON_INSTALL_BASE` environment variables are unset, then this
/// function will return early without panic-ing.
///
/// Registering a ROS 2 package in the ament index requires two distinct steps
/// 1. Create a marker file in `install/share/ament_index/resource_index/$CARGO_PKG_NAME`
/// 2. Copy the package.xml to `install/share/$CARGO_PKG_NAME`
///
/// This enables things like running binary targets with the `ros2 run` command.
/// NOTE: This installation is not something that is generally recommended for rust build scripts.
/// This is rather poor build.rs hygiene, but it's required to exist in the ROS 2 ecosystem.
pub fn ament_package(pkg_name: &str, pkg_path: &str) {
    let colcon_ws_root = env::var("COLCON_WS_BASE").unwrap_or_default();
    let install_base = env::var("COLCON_INSTALL_BASE").unwrap_or_default();

    // If these are unset, we just do nothing. This generally means that this is a
    // pure cargo build. We do not want to panic because the build script should succeed
    // even if we aren't building with colcon.
    if colcon_ws_root.is_empty() || install_base.is_empty() {
        return;
    }

    let marker_path = PathBuf::from(&colcon_ws_root)
        .join(&install_base)
        .join("share/ament_index/resource_index")
        .join(pkg_name);

    create_resource_index(&marker_path).unwrap_or_else(|e| {
        panic!(
            "Could not create marker file in: {}\n{}",
            marker_path.display(),
            e
        )
    });

    let pkg_xml_src = PathBuf::from(pkg_path).join("package.xml");
    let pkg_xml_dest = PathBuf::from(colcon_ws_root)
        .join(install_base)
        .join(pkg_name)
        .join("package.xml");

    copy_package_xml(&pkg_xml_src, &pkg_xml_dest).unwrap_or_else(|e| {
        // Make sure we clean up in case this copy fails. Ideally, we'd never be in a situation
        // the marker file exists, but the package.xml was not copied.
        fs::remove_file(marker_path).unwrap();

        panic!("Failed to copy: {:?} to {:?}\n{}", pkg_xml_src, pkg_xml_dest, e);
    });
}

fn create_resource_index(marker_path: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(marker_path.parent().unwrap())?;
    let _ = fs::File::create(marker_path)?;

    Ok(())
}

fn copy_package_xml(src: &PathBuf, dest: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(dest.parent().unwrap())?;
    fs::copy(src, dest)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
