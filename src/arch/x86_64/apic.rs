//! TODO(BSFishy): document this

use x86::apic::{x2apic, xapic, ApicControl, ApicId, Icr};

/// TODO(BSFishy): document this
#[derive(Debug)]
pub enum Apic {
    /// TODO(BSFishy): document this
    XAPIC(xapic::XAPIC),
    /// TODO(BSFishy): document this
    X2APIC(x2apic::X2APIC),
}

impl Apic {
    /// TODO(BSFishy): document this
    pub fn get() -> Option<Apic> {
        use x86::cpuid::CpuId;

        if let Some(feature_info) = CpuId::new().get_feature_info() {
            if feature_info.has_x2apic() {
                Some(Apic::X2APIC(x2apic::X2APIC::new()))
            } else if feature_info.has_apic() {
                let addr = unsafe { x86::msr::rdmsr(x86::msr::IA32_APIC_BASE) };
                let addr = core::ptr::slice_from_raw_parts_mut(addr as *mut u32, 4 * 1024);
                let addr = unsafe { addr.as_mut() }.unwrap();

                Some(Apic::XAPIC(xapic::XAPIC::new(addr)))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ApicControl for Apic {
    #[inline]
    fn bsp(&self) -> bool {
        match self {
            Apic::XAPIC(xapic) => xapic.bsp(),
            Apic::X2APIC(x2apic) => x2apic.bsp(),
        }
    }

    #[inline]
    fn id(&self) -> u32 {
        match self {
            Apic::XAPIC(xapic) => xapic.id(),
            Apic::X2APIC(x2apic) => x2apic.id(),
        }
    }

    #[inline]
    fn logical_id(&self) -> u32 {
        match self {
            Apic::XAPIC(xapic) => xapic.logical_id(),
            Apic::X2APIC(x2apic) => x2apic.logical_id(),
        }
    }

    #[inline]
    fn version(&self) -> u32 {
        match self {
            Apic::XAPIC(xapic) => xapic.version(),
            Apic::X2APIC(x2apic) => x2apic.version(),
        }
    }

    #[inline]
    fn eoi(&mut self) {
        match self {
            Apic::XAPIC(xapic) => xapic.eoi(),
            Apic::X2APIC(x2apic) => x2apic.eoi(),
        }
    }

    #[inline]
    fn tsc_enable(&mut self, vector: u8) {
        match self {
            Apic::XAPIC(xapic) => xapic.tsc_enable(vector),
            Apic::X2APIC(x2apic) => x2apic.tsc_enable(vector),
        }
    }

    #[inline]
    fn tsc_set(&self, value: u64) {
        match self {
            Apic::XAPIC(xapic) => xapic.tsc_set(value),
            Apic::X2APIC(x2apic) => x2apic.tsc_set(value),
        }
    }

    #[inline]
    unsafe fn ipi_init(&mut self, core: ApicId) {
        match self {
            Apic::XAPIC(xapic) => xapic.ipi_init(core),
            Apic::X2APIC(x2apic) => x2apic.ipi_init(core),
        }
    }

    #[inline]
    unsafe fn ipi_init_deassert(&mut self) {
        match self {
            Apic::XAPIC(xapic) => xapic.ipi_init_deassert(),
            Apic::X2APIC(x2apic) => x2apic.ipi_init_deassert(),
        }
    }

    #[inline]
    unsafe fn ipi_startup(&mut self, core: ApicId, start_page: u8) {
        match self {
            Apic::XAPIC(xapic) => xapic.ipi_startup(core, start_page),
            Apic::X2APIC(x2apic) => x2apic.ipi_startup(core, start_page),
        }
    }

    #[inline]
    unsafe fn send_ipi(&mut self, icr: Icr) {
        match self {
            Apic::XAPIC(xapic) => xapic.send_ipi(icr),
            Apic::X2APIC(x2apic) => x2apic.send_ipi(icr),
        }
    }
}
