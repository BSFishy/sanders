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
