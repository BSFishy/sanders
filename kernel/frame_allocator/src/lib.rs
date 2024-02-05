//! TODO(BSFishy): document this

#![no_std]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use core::{fmt, iter::Iterator, ptr, sync::atomic::{AtomicPtr, Ordering}};
use memory_constants::{MAX_ORDER, PHYSICAL_PAGE_SIZE, BLOCK_SIZES};

#[inline]
const fn find_buddy(base: usize, order: usize) -> usize {
    base ^ (1 << order)
}

/// TODO(BSFishy): document this
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct AllocError;

impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "out of memory")
    }
}

/// TODO(BSFishy): document this
#[derive(Debug)]
pub struct FrameManager {
    free_lists: [AtomicPtr<BlockNode>; MAX_ORDER],
}

impl FrameManager {
    /// TODO(BSFishy): document this
    #[inline]
    pub const fn new() -> FrameManager {
        const INIT: AtomicPtr<BlockNode> = AtomicPtr::new(ptr::null_mut());

        FrameManager {
            free_lists: [INIT; MAX_ORDER],
        }
    }

    /// TODO(BSFishy): document this
    pub unsafe fn init(&self, memory_areas: impl Iterator<Item=(usize, usize)>) {
        for (offset, mut size) in memory_areas {
            for order in 0..MAX_ORDER {
                // use (MAX_ORDER - order - 1) to iterate through the block sizes in reverse
                let order = MAX_ORDER - order - 1;

                let block_size = BLOCK_SIZES[order];
                let physical_block_size = block_size * PHYSICAL_PAGE_SIZE;

                let count = size / physical_block_size;
                let remainder = size % physical_block_size;

                // let p = self.free_lists[order].get_mut();
                for i in 0..count {
                    // use (count - i - 1) to make the loop start from the end instead of the beginning
                    let address = offset + (count - i - 1) * physical_block_size;

                    let block = address as BlockPointer;
                    // SAFETY: this area of memory is guaranteed to be writable as part of the contract
                    (*block).next = self.free_lists[order].load(Ordering::SeqCst);
                    self.free_lists[order].store(block, Ordering::SeqCst);
                }

                size = remainder;
            }
        }
    }

    /// TODO(BSFishy): document this
    #[inline]
    pub fn alloc(&self) -> Result<usize, AllocError> {
        self.alloc_order(0)
    }

    // TODO: log base 2 of the next power of 2 of the size
    // pub fn alloc_size(&mut self, size: usize) -> usize {
    //     self.alloc_order(size.next_power_of_two())
    // }

    /// TODO(BSFishy): document this
    pub fn alloc_order(&self, order: usize) -> Result<usize, AllocError> {
        if order >= MAX_ORDER {
            Err(AllocError)
        } else {
            let block = self.free_lists[order].load(Ordering::SeqCst);
            if !block.is_null() {
                self.free_lists[order].store(unsafe { (*block).next }, Ordering::SeqCst);

                Ok(block as usize)
            } else {
                let block = self.alloc_order(order + 1);
                if let Ok(block) = block {
                    let buddy = find_buddy(block, order) as BlockPointer;

                    unsafe { (*buddy).next = self.free_lists[order].load(Ordering::SeqCst) };
                    self.free_lists[order].store(buddy, Ordering::SeqCst);
                }

                block
            }
        }
    }

    /// TODO(BSFishy): document this
    pub unsafe fn dealloc(&self, address: usize, order: usize) {
        if order >= MAX_ORDER {
            return;
        }

        let buddy = find_buddy(address, order) as BlockPointer;
        let block = address as BlockPointer;

        let mut p = &mut self.free_lists[order].load(Ordering::SeqCst);
        while !p.is_null() && *p != buddy {
            p = &mut (**p).next;
        }

        if *p != buddy {
            // buddy isn't free, just add the block to the free list
            (*block).next = self.free_lists[order].load(Ordering::SeqCst);
            self.free_lists[order].store(block, Ordering::SeqCst);
        } else {
            // buddy is free, remove it from the freelist
            *p = buddy;

            // dealloc the block and its buddy as one block
            if block > buddy {
                self.dealloc(buddy as usize, order + 1);
            } else {
                self.dealloc(block as usize, order + 1);
            }
        }
    }
}

type BlockPointer = *mut BlockNode;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct BlockNode {
    pub(crate) next: BlockPointer,
}

impl Default for BlockNode {
    #[inline]
    fn default() -> Self {
        BlockNode {
            next: ptr::null_mut(),
        }
    }
}
