//! TODO(BSFishy): document this

#![no_std]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

// use core::ptr;

/// TODO(BSFishy): document this
macro_rules! per_cpu {
    ($item:item) => {
        #[link_section = ".data..per_cpu"]
        $item
    };
    ($item:item $($tt:tt)+) => {
        $crate::per_cpu!($item);

        $crate::per_cpu!($($tt)+);
    };
}

per_cpu! {
    /// TODO(BSFishy): document this
    pub static NUM: i32 = 1;
}

/// TODO(BSFishy): document this
pub fn prepare_per_cpu() {
    todo!()
}

/// TODO(BSFishy): document this
#[derive(Debug)]
#[repr(transparent)]
pub struct PerCpu<T>(T);

impl<T> PerCpu<T> {
    /// TODO(BSFishy): document this
    pub const fn new(value: T) -> PerCpu<T> {
        PerCpu(value)
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn read(&self) -> &T {
        let _address = (&self.0) as *const T;
        todo!()
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn write(&self, _value: T) {
        todo!()
    }
}

impl<T> Default for PerCpu<T> where T: Default {
    fn default() -> Self {
        PerCpu(T::default())
    }
}
