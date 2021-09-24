//! This library provides a simple, architecture independent way to detect the current privilege level (CPL).
//!
//! # Example
//!
//! ```rust
//! use privilege_level::{privilege_level, PrivilegeLevel};
//!
//! # fn main() {
//! match privilege_level() {
//!     PrivilegeLevel::Hypervisor => println!("Currently a hypervisor"),
//!     PrivilegeLevel::Kernel => println!("Currently in kernel space"),
//!     PrivilegeLevel::Driver => println!("Currently in a driver"),
//!     PrivilegeLevel::User => println!("Currently in user space"),
//! }
//! # }
//! ```
//!
//! # Details
//!
//! The main entrypoint into this crate is through the [`privilege_level`] function.
//! That will use the architecture dependent code necessary to determine the current privilege level then converts it into the common API.
//!
//! The common API ([`PrivilegeLevel`]) is meant to be usable on any platform.
//! It contains generic privilege levels that are common on most architectures.
//! However, it's not always possible to include support for _every_ architecture.
//! As a compromise, depending on the target architecture, certain functions will be made available that return architecture-specific structures.
//!
//! If you only want the raw privilege level number rather than a symbolic structure, you can also use the [`raw_privilege_level`] function.
//! It uses architecture specific code to read the privilege level, converts it into a u16, then returns it.
//! It's important to note that it doesn't do anything to make the output common in any way!

#![cfg_attr(not(test), no_std)]
#![cfg_attr(doc, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

/// Represents a generic CPU privilege level.
///
/// This creates a common interface for privilege levels.
/// It abstracts away architecture independent structure and allows you to test if you're in specific contexts.
///
/// Hopefully this enum can provide you with everything you need from privilege levels, however it is very possible that it might not.
/// In that case, architecture specific functions are provided to let you get the architecture specific privilege level information.
///
/// # Example
///
/// ```rust
/// # use privilege_level::{privilege_level, PrivilegeLevel};
/// # fn main() {
/// match privilege_level() {
///     PrivilegeLevel::Hypervisor => println!("Hypervisor specific"),
///     PrivilegeLevel::Kernel => println!("Kernel specific"),
///     PrivilegeLevel::Driver => println!("Driver specific"),
///     PrivilegeLevel::User => println!("User specific"),
/// }
/// # }
/// ```
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
#[repr(usize)]
pub enum PrivilegeLevel {
    /// Represents when the processor is in hypervisor mode, with a guest OS running beneath.
    ///
    /// On certain architectures, a separate privilege level is created when hosting a guest OS.
    /// When this happens, the host OS is dropped a privilege level and the guest OS runs where the host previously was.
    /// This represents that circumstance.
    ///
    /// # Architectures
    ///
    ///  - on `x86`, this represents ring -1
    ///
    /// # Example
    ///
    /// ```rust
    /// # use privilege_level::{privilege_level, PrivilegeLevel};
    /// # fn main() {
    /// assert_eq!(privilege_level(), PrivilegeLevel::Hypervisor, "Not in hypervisor level!");
    /// # }
    /// ```
    Hypervisor = 0,
    /// Represents when the processor is at the kernel level.
    ///
    /// This is the lowest privilege level, excluding [`PrivilegeLevel::Hypervisor`].
    /// It has the highest permissions on the system.
    ///
    /// # Architectures
    ///
    ///  - on `x86`, this represents ring 0
    ///
    /// # Example
    ///
    /// ```rust
    /// # use privilege_level::{privilege_level, PrivilegeLevel};
    /// # fn main() {
    /// assert_eq!(privilege_level(), PrivilegeLevel::Kernel, "Not in kernel level!");
    /// # }
    /// ```
    Kernel,
    /// Represents when the processor is at a level where drivers run.
    ///
    /// This level exists somewhere in between [`PrivilegeLevel::Kernel`] and [`PrivilegeLevel::User`].
    /// It it _mostly_ privileged in the sense that it can access supervisor pages, however certain privileged instructions are off limits.
    ///
    /// **Note:** not all architectures support this level.
    /// If you are making use of it in a significant way, it is recommended that you make sure your code will work if this level doesn't exist.
    ///
    /// # Architectures
    ///
    ///  - on `x86`, this represents both rings 1 and 2
    ///
    /// # Example
    ///
    /// ```rust
    /// # use privilege_level::{privilege_level, PrivilegeLevel};
    /// # fn main() {
    /// assert_eq!(privilege_level(), PrivilegeLevel::Driver, "Not in driver level!");
    /// # }
    /// ```
    Driver,
    /// Represents when the processor is in user space.
    ///
    /// This level is the least privileged of all.
    /// It is where user mode programs exist and has no access whatsoever to anything privileged.
    ///
    /// # Architectures
    ///
    ///  - on `x86`, this represents ring 3
    ///
    /// # Example
    ///
    /// ```rust
    /// # use privilege_level::{privilege_level, PrivilegeLevel};
    /// # fn main() {
    /// assert_eq!(privilege_level(), PrivilegeLevel::User, "Not in user level!");
    /// # }
    /// ```
    User,
}

/// This is a test
///
/// # Example
///
/// ```rust
/// fn main() {
///     println!("here");
/// }
/// ```
impl PrivilegeLevel {
    /// Gets the minimum privilege level.
    ///
    /// This is the absolute lowest level privilege.
    /// That is to say that this privilege level has the most permissions.
    ///
    /// As of now, [`PrivilegeLevel::Hypervisor`] returns.
    /// It is possible that it will change in the future, so this is a future-proof facility to get the absolute lowest privilege level.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use privilege_level::{privilege_level, PrivilegeLevel};
    /// # fn main() {
    /// assert!(privilege_level() > PrivilegeLevel::min(), "The privilege level is too low!");
    /// # }
    /// ```
    #[inline]
    pub fn min() -> PrivilegeLevel {
        PrivilegeLevel::Hypervisor
    }

    /// Gets the maximum privilege level.
    ///
    /// This is the absolute highest level privilege.
    /// That is to say that this privilege level has the least permissions.
    ///
    /// As of now, [`PrivilegeLevel::User`] returns.
    /// It is possible that it will change in the future, so this is a future-proof facility to get the absolute highest privilege level.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use privilege_level::{privilege_level, PrivilegeLevel};
    /// # fn main() {
    /// assert!(privilege_level() < PrivilegeLevel::max(), "The privilege level must be lower than the maximum!");
    /// # }
    /// ```
    #[inline]
    pub fn max() -> PrivilegeLevel {
        PrivilegeLevel::User
    }

    /// Gets the current privilege level.
    ///
    /// This is a simple wrapper around the [`privilege_level`] function.
    /// It is meant to allow you a more "rusty" way to get the privilege level.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use privilege_level::{privilege_level, PrivilegeLevel};
    /// # fn main() {
    /// assert_eq!(privilege_level(), PrivilegeLevel::current(), "Did not get the same privilege level!");
    /// # }
    /// ```
    #[inline]
    pub fn current() -> PrivilegeLevel {
        privilege_level()
    }
}

/// Gets the current privilege level.
///
/// This will check the target architecture and call the correct architecture-specific function to get its corresponding privilege level.
/// After that, it simply wraps it in the [`PrivilegeLevel`] enum for convenience.
/// For more information about architecture-specific functions, take a look at the root module for architecture-specific functions.
///
/// # Example
///
/// ```rust
/// # use privilege_level::privilege_level;
/// # fn main() {
/// println!("Currently running at privilege level: {:?}", privilege_level());
/// # }
/// ```
#[inline]
pub fn privilege_level() -> PrivilegeLevel {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            PrivilegeLevel::from(x86_64_privilege_level())
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}

// TODO: potentially move these sorts of functions to a separate module and just export them?
#[cfg(any(target_arch = "x86_64", doc))]
#[cfg_attr(doc, doc(cfg(target_arch = "x86_64")))]
#[doc(inline)]
pub use x86_64::PrivilegeLevel as x86_64PrivilegeLevel;

// TODO: potentially move these sorts of functions to a separate module and just export them?
#[cfg(any(target_arch = "x86_64", doc))]
#[cfg_attr(doc, doc(cfg(target_arch = "x86_64")))]
impl<T> From<T> for PrivilegeLevel
where
    T: Into<x86_64PrivilegeLevel>,
{
    #[inline]
    fn from(other: T) -> Self {
        // TODO: be able to detect if we are currently running in a virtual machine
        match other.into() {
            x86_64PrivilegeLevel::Ring0 => PrivilegeLevel::Kernel,
            x86_64PrivilegeLevel::Ring1 | x86_64PrivilegeLevel::Ring2 => PrivilegeLevel::Driver,
            x86_64PrivilegeLevel::Ring3 => PrivilegeLevel::User,
        }
    }
}

// TODO: potentially move these sorts of functions to a separate module and just export them?
/// Gets the current privilege level for `x86_64` systems.
///
/// This will get the raw privilege level from [`raw_privilege_level`] then convert it into the x86_64 privilege level enum.
/// The [`x86_64PrivilegeLevel`] enum is actually a reexport of the privilege level enum from the [`x86_64`] crate.
/// It is added for convenience so that you don't have to worry about including that crate in your project.
///
/// # Example
///
/// ```rust
/// # use privilege_level::{x86_64_privilege_level, x86_64PrivilegeLevel};
/// # fn main() {
/// assert_eq!(x86_64_privilege_level(), x86_64PrivilegeLevel::Ring0, "This must be running in ring 0!");
/// # }
/// ```
#[cfg(any(target_arch = "x86_64", doc))]
#[cfg_attr(doc, doc(cfg(target_arch = "x86_64")))]
#[inline]
pub fn x86_64_privilege_level() -> x86_64PrivilegeLevel {
    x86_64PrivilegeLevel::from_u16(raw_privilege_level())
}

/// Gets the current privilege level as a raw number.
///
/// This will detect the target architecture then use architecture-specific code to read the current privilege level and return it.
/// No effort is made to convert the resulting number into anything common or architecture independent.
/// If you want that functionality, use [`privilege_level`] instead.
///
/// **Note:** since this function does no conversion into a common format, things can break on different architectures.
/// Please take that into consideration when using this function!
/// If you need to represent a privilege level as a number, [`PrivilegeLevel`] can be converted into a `usize`.
///
/// # Example
///
/// ```rust
/// # use privilege_level::raw_privilege_level;
/// # fn main() {
/// println!("Raw privilege level is {}", raw_privilege_level());
/// # }
/// ```
#[inline]
pub fn raw_privilege_level() -> u16 {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            use bit_field::BitField;
            use x86_64::instructions::segmentation::{CS, Segment};

            CS::get_reg().0.get_bits(0..2)
        } else {
            compile_error!("Unsupported architecture");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn has_4_levels() {
        use core::convert::TryFrom;

        assert_ne!(
            PrivilegeLevel::try_from(0),
            Err,
            "Does not contain privilege level 0"
        );
        assert_ne!(
            PrivilegeLevel::try_from(1),
            Err,
            "Does not contain privilege level 1"
        );
        assert_ne!(
            PrivilegeLevel::try_from(2),
            Err,
            "Does not contain privilege level 2"
        );
        assert_ne!(
            PrivilegeLevel::try_from(3),
            Err,
            "Does not contain privilege level 3"
        );
    }
}
