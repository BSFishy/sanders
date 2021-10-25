//! TODO(BSFishy): document this

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub mod x86;

        pub use x86 as current;
    } else if #[cfg(target_arch = "x86_64")] {
        pub mod x86_64;

        pub use x86_64 as current;
    } else {
        compile_error!("Unsupported architecture");
    }
}
