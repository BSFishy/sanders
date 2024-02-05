//! TODO(BSFishy): document this

#![no_std]
#![cfg_attr(doc, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

pub use system::System;

cfg_if::cfg_if! {
    if #[cfg(any(doc, target_arch = "x86", target_arch = "x86_64"))] {
        #[cfg_attr(doc, doc(cfg(any(target_arch = "x86", target_arch = "x86_64"))))]
        pub use arch_x86;

        #[cfg(not(doc))]
        pub use arch_x86 as current;
    }
}

/// TODO(BSFishy): document this
pub fn get_system() -> impl System {
    cfg_if::cfg_if! {
        if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
            arch_x86::SystemX86
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}
