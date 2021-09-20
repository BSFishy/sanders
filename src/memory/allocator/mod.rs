//! TODO(BSFishy): document this
//! TODO: implement some allocators from https://wiki.osdev.org/Memory_Allocation#Choosing_a_Memory_Allocator
//! TODO: convert this module into an abstraction layer in front of architecture-specific code

use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

cfg_if::cfg_if! {
    if #[cfg(feature = "linked_list_allocation")] {
        use linked_list_allocator::LockedHeap;

        #[global_allocator]
        static ALLOCATOR: LockedHeap = LockedHeap::empty();
    } else if #[cfg(feature = "bump_allocation")] {
        pub mod bump;

        #[global_allocator]
        static ALLOCATOR: locked::Locked<bump::BumpAllocator> = locked::Locked::new(bump::BumpAllocator::new());
    } else if #[cfg(feature = "fixed_sized_allocation")] {
        pub mod fixed_size;

        #[global_allocator]
        static ALLOCATOR: locked::Locked<fixed_size::FixedSizeBlockAllocator> = locked::Locked::new(fixed_size::FixedSizeBlockAllocator::new());
    } else {
        compile_error!("An allocation method must be specified");
    }
}

#[inline(always)]
fn init_allocator() {
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(feature = "linked_list_allocation")] {
                ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
            } else if #[cfg(feature = "bump_allocation")] {
                ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
            } else if #[cfg(feature = "fixed_sized_allocation")] {
                ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
            } else {
                compile_error!("An allocation method must be specified");
            }
        }
    }
}

pub mod locked;

/// TODO(BSFishy): document this
pub const HEAP_START: usize = 0x_4444_4444_0000;
/// TODO(BSFishy): document this
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// TODO(BSFishy): document this
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };
    }

    init_allocator();

    Ok(())
}

/// Align the given address `addr` upwards to alignment `align`.
///
/// Requires that `align` is a power of two.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    ///
    /// # Safety
    ///
    /// This function is unsafe because the caller must guarantee that the passed
    /// memory map is valid. The main requirement is that all frames that are marked
    /// as `USABLE` in it are really unused.
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
