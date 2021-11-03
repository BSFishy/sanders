//! TODO(BSFishy): document this

use core::ops::Deref;
use bootloader::BootInfo;
use bootloader::bootinfo::MemoryMap;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageSize, PageTable, PageTableFlags, PhysFrame, Size4KiB};
use x86_64::structures::paging::mapper::MapToError;
use x86_64::VirtAddr;

pub mod frame_manager;

pub use frame_manager::FrameManager;

// TODO: ideally, these shouldn't all need to be lazy statics
lazy_static! {
    static ref BOOT_INFO: Mutex<Option<&'static BootInfo>> = Mutex::new(None);
    static ref MAPPER: Mutex<OffsetPageTable<'static>> = {
        let physical_memory_offset = VirtAddr::new(BOOT_INFO.lock().expect("Boot info is uninitialized").physical_memory_offset);
        Mutex::new(unsafe { init_mapper::<Size4KiB>(physical_memory_offset) })
    };
    static ref FRAME_MANAGER: Mutex<FrameManager> = Mutex::new(FrameManager::new(&(BOOT_INFO.lock().expect("Boot info is uninitialized")).memory_map));
}

/// TODO(BSFishy): document this
pub fn init(boot_info: &'static BootInfo) {
    *BOOT_INFO.lock() = Some(boot_info);

    // init_heap::<Size4KiB>().expect("Heap allocation failed");
}

// TODO: support recursive page table. Probably need to create a new mapper that allows either recursive or offset and just smartly switch between the two
unsafe fn init_mapper<S: PageSize>(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_page_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_page_table, physical_memory_offset)
}

fn init_heap<S: PageSize>() -> Result<(), MapToError<S>> where OffsetPageTable<'static>: Mapper<S> {
    let mapper = &mut *MAPPER.lock();
    let frame_allocator = &mut *FRAME_MANAGER.lock();

    let mut start: u64 = frame_allocator.end_address::<S>().ok_or(MapToError::FrameAllocationFailed)?;
    let mut size: u64 = 0;

    loop {
        match frame_allocator.allocate_frame() {
            Some(frame) => {
                log::debug!("Mapping frame {:?}", frame);

                let start_addr = VirtAddr::new(start + frame.start_address().as_u64());
                let page = Page::<S>::containing_address(start_addr);
                let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
                unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };

                size += frame.size();
            },
            None => break,
        }
    }

    log::info!("Mapped from {:012X} to {:012X}", start, start + size);

    Ok(())
}

/// TODO(BSFishy): document this
pub fn start_size() -> (usize, usize) {
    let frame_manager = &*FRAME_MANAGER.lock();

    (frame_manager.end_address::<Size4KiB>().expect("Unable to get end of frames") as usize, frame_manager.size::<Size4KiB>() as usize)
}

/// TODO(BSFishy): document this
pub fn map<S: PageSize>(pointer: *mut u8, size: usize) -> Result<(), MapToError<S>> where OffsetPageTable<'static>: Mapper<S> {
    let mapper = &mut *MAPPER.lock();
    let frame_allocator = &mut *FRAME_MANAGER.lock();

    let frame = frame_allocator.allocate_frame().ok_or(MapToError::FrameAllocationFailed)?;

    let address = pointer as u64;
    let address = VirtAddr::new(address);
    let page = Page::<S>::containing_address(address);
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe { mapper.map_to(page, frame, flags, frame_allocator)?.flush() };

    Ok(())
}

/// Returns a mutable reference to the active level 4 table.
///
/// # Safety
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
