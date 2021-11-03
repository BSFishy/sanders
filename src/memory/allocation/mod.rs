//! TODO(BSFishy): document this

use core::{alloc::{GlobalAlloc, Layout}, ptr};
use crate::{get_system, System};

mod locked;

#[cfg(feature = "bump_allocation")]
pub mod bump;
#[cfg(feature = "fixed_sized_allocation")]
pub mod fixed_size;

#[cfg(feature = "linked_list_allocation")]
use linked_list_allocator::LockedHeap;
use crate::system::Memory;

/// TODO(BSFishy): document this
pub struct Allocator {
    #[cfg(feature = "bump_allocation")]
    bump: locked::Locked<bump::BumpAllocator>,
    #[cfg(feature = "fixed_sized_allocation")]
    fixed_size: locked::Locked<fixed_size::FixedSizeBlockAllocator>,
    #[cfg(feature = "linked_list_allocation")]
    linked_list: LockedHeap,
}

impl Allocator {
    /// TODO(BSFishy): document this
    pub const fn new() -> Allocator {
        Allocator {
            #[cfg(feature = "bump_allocation")]
            bump: locked::Locked::new(bump::BumpAllocator::new()),
            #[cfg(feature = "fixed_sized_allocation")]
            fixed_size: locked::Locked::new(fixed_size::FixedSizeBlockAllocator::new()),
            #[cfg(feature = "linked_list_allocation")]
            linked_list: LockedHeap::empty(),
        }
    }

    /// TODO(BSFishy): document this
    pub fn init(&self, start: usize, size: usize) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "linked_list_allocation")] {
                unsafe { self.linked_list.lock().init(start, size) };
            } else if #[cfg(feature = "fixed_sized_allocation")] {
                unsafe { self.fixed_size.lock().init(start, size) };
            } else if #[cfg(feature = "bump_allocation")] {
                unsafe { self.bump.lock().init(start, size) };
            }
        }
    }
}

unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut pointer: *mut u8 = ptr::null_mut();

        cfg_if::cfg_if! {
            if #[cfg(feature = "linked_list_allocation")] {
                pointer = self.linked_list.alloc(layout);
            } else if #[cfg(feature = "fixed_sized_allocation")] {
                pointer = self.fixed_size.alloc(layout);
            } else if #[cfg(feature = "bump_allocation")] {
                pointer = self.bump.alloc(layout);
            }
        }

        get_system().memory().ensure_mapped(pointer, &layout);

        pointer
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        cfg_if::cfg_if! {
            if #[cfg(feature = "linked_list_allocation")] {
                self.linked_list.dealloc(ptr, layout);
            } else if #[cfg(feature = "fixed_sized_allocation")] {
                self.fixed_size.dealloc(ptr, layout);
            } else if #[cfg(feature = "bump_allocation")] {
                self.bump.dealloc(ptr, layout);
            }
        }
    }
}
