//! Build script for the example application.
//!
//! This points the macros at the example translation directory.

fn main() {
    localization_build::set_root("./examples/basic/translations");
}
