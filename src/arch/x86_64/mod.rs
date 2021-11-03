//! TODO(BSFishy): document this

pub mod apic;
pub mod memory;
pub mod system;

#[doc(inline)]
pub use apic::Apic;

#[doc(inline)]
pub use system::SystemX86_64;
