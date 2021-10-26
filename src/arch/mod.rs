//! TODO(BSFishy): document this

use crate::system::System;

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub mod x86;

        pub use self::x86 as current;
    } else if #[cfg(target_arch = "x86_64")] {
        pub mod x86_64;

        pub use self::x86_64 as current;
    } else {
        compile_error!("Unsupported architecture");
    }
}

/// TODO(BSFishy): document this
// pub fn get_system() -> impl System {
pub fn get_system() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86")] {
            // panic!("No system yet");
        } else if #[cfg(target_arch = "x86_64")] {
            // panic!("No system yet");
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}
