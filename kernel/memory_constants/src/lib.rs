//! TODO(BSFishy): document this

#![no_std]
#![feature(const_panic)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

/// TODO(BSFishy): document this
// TODO: make this configurable through environment variables
// TODO: maybe use a NonZeroUsize?
pub const MAX_ORDER: usize = 10;

const _: () = assert!(MAX_ORDER > 0, "MAX_ORDER must be greater than zero");

/// The smallest size of pages in KiB.
///
/// # Example
///
/// ```no_run
/// # use memory_constants::PAGE_SIZE;
/// # fn main() {
/// println!("The smallest page size is {}KiB", PAGE_SIZE);
/// # }
/// ```
// TODO: make this configurable through environment variables
pub const PAGE_SIZE: usize = 4;

/// The smallest size of pages in bytes.
///
/// This is the same thing as multiplying [`PAGE_SIZE`] by `1024`.
///
/// # Example
///
/// ```
/// # use memory_constants::{PAGE_SIZE, PHYSICAL_PAGE_SIZE};
/// # fn main() {
/// assert_eq!(PHYSICAL_PAGE_SIZE, PAGE_SIZE * 1024);
/// # }
/// ```
pub const PHYSICAL_PAGE_SIZE: usize = PAGE_SIZE * 1024;

/// TODO(BSFishy): document this
pub const BLOCK_SIZES: [usize; MAX_ORDER] = {
    let mut sizes = [0; MAX_ORDER];

    // We can't use iterators or anything like that in const fn's,
    // so this is a quick and easy alternative
    let mut i: usize = 0;
    while i < MAX_ORDER {
        sizes[i] = 2usize.pow(i as u32);
        i += 1;
    }

    sizes
};

/// TODO(BSFishy): document this
pub const REVERSED_BLOCK_SIZES: [usize; MAX_ORDER] = {
    let mut sizes = [0; MAX_ORDER];

    let mut i: usize = 0;
    while i < MAX_ORDER {
        sizes[i] = BLOCK_SIZES[MAX_ORDER - i - 1];
        i += 1;
    }

    sizes
};
