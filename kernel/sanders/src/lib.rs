//! TODO(BSFishy): document this

#![no_std]
#![allow(clippy::empty_loop)]
#![deny(missing_docs)]
#![deny(missing_doc_code_examples)]

use arch::{get_system, System};
use bootloader::BootInfo;

pub mod error;

/// Initialize the system as well as any related kernel submodules.
///
/// The logger is prepared first so that logging can take place in initialization and afterwards.
/// Next, architecture-specific setup occurs to prepare anything like multicore processing.
/// Then, memory, multitasking, and IPC are initialized.
/// Finally, architecture-specific initialization occurs.
///
/// The `boot_info` parameter provides the kernel with valuable system information.
/// It helps with setting up system memory and drivers and things of that sort.
/// The actual type of the parameter is subject to change to allow for future scalability.
///
/// No actual user code is run from this function.
/// All that occurs is system functionality is initialized for future use.
///
/// # Examples
///
/// **Initialize and run the kernel:**
/// ```no_run
/// fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
///     sanders::init(boot_info).expect("Unable to initialize kernel");
///
///     sanders::run();
/// }
/// ```
///
/// **Initialize the kernel and run custom code:**
/// ```no_run
/// fn kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
///     sanders::init(boot_info).expect("Unable to initialize kernel");
///
///     // Custom code utilizing the kernel
///
///     // Empty loop for when the code has finished running
///     loop {}
/// }
/// ```
///
/// TODO: set up drivers somewhere
pub fn init(boot_info: &'static BootInfo) -> Result<(), error::InitError> {
    // TODO: maybe create a feature flag for setting up the logger?
    logging::prepare_logger()?;

    // Prepare the system before the individual modules are initialized.
    // This allows for architecture-specific initialization to occur that is
    // necessary for the individual modules to be initialized. For example,
    // certain registers or tables might need to be set up before initialization
    // can occur.
    let sys = get_system();
    sys.prepare()
        .map_err(error::InitError::from_system_prepare)?;
        // .map_err(|e| error::InitError::SystemPrepare(e))?;

    // Initialize all of the individual modules with any necessary information
    // they might need.
    memory::init(boot_info)?;
    multitasking::init(boot_info)?;
    ipc::init(boot_info)?;

    // TODO: initialize drivers somewhere, probably around here

    // Allow the architecture-specific system to initialize itself. This allows
    // for any architecture-specific preparations or initializations to be made.
    // For example, certain registers or tables might need to be set up for a
    // proper running environment.
    sys.init(boot_info)
        .map_err(error::InitError::from_system_init)?;
        // .map_err(|e| error::InitError::SystemInit(e))?;

    Ok(())
}

/// TODO(BSFishy): document this
pub fn run() -> ! {
    log::info!("Running!");

    let sys = get_system();
    loop {
        // TODO: abstract this away somehow
        sys.pause();
    }
}
