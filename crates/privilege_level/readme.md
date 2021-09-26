# privilege_level

[![License](https://img.shields.io/github/license/BSFishy/sanders)](https://github.com/BSFishy/sanders/blob/develop/LICENSE)
[![Build status](https://img.shields.io/github/workflow/status/BSFishy/sanders/Build?logo=github-actions&logoColor=white)](https://github.com/BSFishy/sanders/actions/workflows/build.yml)
[![Version](https://img.shields.io/crates/v/privilege_level?logo=rust)](https://crates.io/crates/privilege_level)
[![Downloads](https://img.shields.io/crates/d/privilege_level?logo=rust)](https://crates.io/crates/privilege_level)

This crate is a quick and simple tool to fetch the current privilege level of the CPU.
The main use case of this is probably in kernels, where you might need to know what your privilege level is so that you can do different things depending on what it is.

## Supported architectures

A simple, general API is defined that allows you to get the current privilege level on any architecture.
The main API doesn't distinguish between different architectures, and any architecture specific code is converted into architecture independent structures.
This allows you to only need to focus on what context your code is running in without having to worry about supporting every single architecture.

Here are the currently supported architectures:

 - `x86_64`

### Looking for help!

Currently, only `x86_64` is supported.
This project is developed for the [_s&ers project_](https://github.com/BSFishy/sanders/), and right now it is simply just trying to get off the ground.
While I'm still building it up and adding features, I'm only focusing on `x86_64` for simplicity.
Eventually, I'll begin adding support for more architectures, however, for the time being I will only focus on `x86_64`.

However, that doesn't prevent other people from contributing!
If you know other architectures or need support for a different architecture, feel free to add support for it with a pull request!
Any help will be greatly appreciated!

## Platform-specific

If you want or need architecture-specific privilege level distinction, facilities are provided!
All architectures provide functions and structures that are specific to them, so that you have symbolic facilities to distinguish the current privilege level.
The idea is to allow you to distinguish between architecture-specific details that the common API doesn't show (such as the difference between rings 1 and 2 on `x86`).

**Note:** These functions and structures are removed from platforms that they aren't supported on, using configuration flags, so be sure to set up your calling code correctly.

## Examples

**Print the current privilege level**

```rust
use privilege_level::{privilege_level, PrivilegeLevel};

fn print_privilege_level() {
    match privilege_level() {
        PrivilegeLevel::Hypervisor => println!("Currently running as the hypervisor"),
        PrivilegeLevel::Kernel => println!("Currently running as the kernel"),
        PrivilegeLevel::Driver => println!("Currently running as a driver"),
        PrivilegeLevel::User => println!("Currently running as a user program"),
    }
}
```

**Get the current `x86` ring level**

```rust
use privilege_level::{x86_64_privilege_level, x86_64PrivilegeLevel};

fn print_ring_level() {
    match x86_64_privilege_level() {
        x86_64PrivilegeLevel::Ring0 => println!("Currently in ring 0"),
        x86_64PrivilegeLevel::Ring1 => println!("Currently in ring 1"),
        x86_64PrivilegeLevel::Ring2 => println!("Currently in ring 2"),
        x86_64PrivilegeLevel::Ring3 => println!("Currently in ring 3"),
    }
}
```

**Protect a function from unauthorized access**

```rust
use privilege_level::{privilege_level, PrivilegeLevel};

fn protected_function() {
    if privilege_level() > PrivilegeLevel::Driver {
        panic!("This function needs to run at driver level or higher!");
    }
    
    // do things with driver level permissions...
}
```

# License

The `privilege_level` crate and [_s&ers project_](https://github.com/BSFishy/sanders/) is licensed under the MIT license.
