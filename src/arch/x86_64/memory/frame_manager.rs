//! TODO(BSFishy): document this

use bootloader::bootinfo::{MemoryMap, MemoryRegion, MemoryRegionType};
use x86_64::PhysAddr;
use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, PageSize, PhysFrame};

/// TODO(BSFishy): document this
pub const MAX_MEMORY_MAP_SIZE: usize = 64;

// /// TODO(BSFishy): document this
// pub struct FrameManager {
//     entries: [Option<MemoryRegion>; MAX_MEMORY_MAP_SIZE],
// }
//
// impl FrameManager {
//     pub fn new(memory_map: &'static MemoryMap) -> FrameManager {
//         let mut entries: [Option<MemoryRegion>; MAX_MEMORY_MAP_SIZE] = [None; MAX_MEMORY_MAP_SIZE];
//
//         for (i, region) in memory_map.iter().enumerate() {
//             entries[i] = Some(region.clone());
//         }
//
//         FrameManager {
//             entries,
//         }
//     }
// }
//
// unsafe impl<S: PageSize> FrameAllocator<S> for FrameManager {
//     fn allocate_frame(&mut self) -> Option<PhysFrame<S>> {
//         todo!()
//     }
// }
//
// impl<S: PageSize> FrameDeallocator<S> for FrameManager {
//     unsafe fn deallocate_frame(&mut self, frame: PhysFrame<S>) {
//         todo!()
//     }
// }

/// TODO(BSFishy): document this
pub struct FrameManager {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl FrameManager {
    /// TODO(BSFishy): document this
    pub fn new(memory_map: &'static MemoryMap) -> FrameManager {
        for region in memory_map.iter() {
            log::info!("{:012X}..{:012X} - {:?}", region.range.start_addr(), region.range.end_addr(), region.region_type);
        }

        FrameManager {
            memory_map,
            next: 0,
        }
    }

    /// TODO(BSFishy): document this
    pub fn end_address<S: PageSize>(&self) -> Option<u64> {
        let frames = self.usable_frames::<S>();

        frames.last().map(|frame| frame.start_address().as_u64() + frame.size())
    }

    /// TODO(BSFishy): document this
    pub fn size<S: PageSize>(&self) -> u64 {
        let frames = self.usable_frames::<S>();

        frames.fold(0u64, |acc, frame| acc + frame.size())
    }

    // TODO: make this properly able to support different page sizes
    fn usable_frames<S: PageSize>(&self) -> impl Iterator<Item = PhysFrame<S>> {
        // get usable regions from memory map
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let usable_regions = usable_regions.filter(|r| (r.range.end_addr() - r.range.start_addr()) % S::SIZE == 0);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(S::SIZE as usize));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::<S>::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl<S: PageSize> FrameAllocator<S> for FrameManager {
    fn allocate_frame(&mut self) -> Option<PhysFrame<S>> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}

impl<S: PageSize> FrameDeallocator<S> for FrameManager {
    unsafe fn deallocate_frame(&mut self, _frame: PhysFrame<S>) {
        todo!()
    }
}
