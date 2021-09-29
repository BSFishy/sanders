//! TODO(BSFishy): document this

use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86::apic::{xapic, x2apic, ApicControl};

/// TODO(BSFishy): document this
pub const PIC_1_OFFSET: u8 = 32;
/// TODO(BSFishy): document this
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    /// TODO(BSFishy): document this
    pub static ref PIC: spin::Mutex<Pic> = {
        // Use x2apic for now until cupid has support for no_std then we detect which is available
        spin::Mutex::new(Pic::X2APIC(x2apic::X2APIC::new()))
    };
    // pub static ref XAPIC: spin::Mutex<xapic::XAPIC> = {
    //     let addr = unsafe { x86::msr::rdmsr(x86::msr::IA32_APIC_BASE) };
    //     let addr = core::ptr::slice_from_raw_parts_mut(addr as *mut u32, 4 * 1024);
    //     let addr = unsafe { addr.as_mut() }.unwrap();
    //
    //     spin::Mutex::new(xapic::XAPIC::new(addr))
    // };
}

/// TODO(BSFishy): document this
pub enum Pic {
    /// TODO(BSFishy): document this
    PIC(ChainedPics),
    /// TODO(BSFishy): document this
    XAPIC(xapic::XAPIC),
    /// TODO(BSFishy): document this
    X2APIC(x2apic::X2APIC),
}

impl Pic {
    /// TODO(BSFishy): document this
    pub unsafe fn enable(&mut self) {
        match self {
            Pic::PIC(pic) => pic.initialize(),
            Pic::XAPIC(xapic) => xapic.attach(),
            Pic::X2APIC(x2apic) => x2apic.attach(),
        }
    }

    /// TODO(BSFishy): document this
    pub fn id(&self) -> Option<usize> {
        match self {
            Pic::PIC(_) => None,
            Pic::XAPIC(xapic) => Some(xapic.id() as usize),
            Pic::X2APIC(x2apic) => Some(x2apic.id() as usize),
        }
    }

    /// TODO(BSFishy): document this
    pub fn bsp(&self) -> Option<bool> {
        match self {
            Pic::PIC(_) => None,
            Pic::XAPIC(xapic) => Some(xapic.bsp()),
            Pic::X2APIC(x2apic) => Some(x2apic.bsp()),
        }
    }
}
