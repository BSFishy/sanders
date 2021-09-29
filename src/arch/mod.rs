//! TODO(BSFishy): document this

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub mod x86;
    } else if #[cfg(target_arch = "x86_64")] {
        pub mod x86_64;
    } else {
        compile_error!("Architecture not supported");
    }
}

/// TODO(BSFishy): document this
pub trait CPU {
    /// TODO(BSFishy): document this
    fn id(&self) -> usize;
}

/// TODO(BSFishy): document this
pub fn get_cpu() -> impl CPU {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            x86_64::x86_64CPU
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}
