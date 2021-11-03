//! TODO(BSFishy): document this

use crate::system::System;

cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "x86_64", target_arch = "x86"))] {
        pub mod x86_64;

        pub use self::x86_64 as current;
    } else {
        compile_error!("Unsupported architecture");
    }
}

/// TODO(BSFishy): document this
pub fn get_system() -> impl System {
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "x86_64", target_arch = "x86"))] {
            x86_64::SystemX86_64
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}
