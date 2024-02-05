# s&ers

> :warning: **s&ers is currently on hold.** :warning:
> I really like the idea I've come up with, however I don't have the time to be working on it right now.
> Eventually, I will come back to it, but for now, it's on hold.
> If you're interested in the project, feel free to reach out to me and I can give you more information about it.

s&ers is a [microkernel](https://en.wikipedia.org/wiki/Microkernel) written in [Rust](https://www.rust-lang.org/).
Its goal is to be simple, secure, and very fast.

s&ers is written all lowercase (never capitalized) and pronounced "sanders".
When writing s&ers in a non-code context, it is always written "s&ers" (with an ampersand).
When writing s&ers in a code context, it is fully written out as "sanders".

s&ers is roughly based upon the [Writing an OS in Rust](https://os.phil-opp.com/) blog series.
Although s&ers has diverged quite a bit and is a microkernel, rather than a monolithic kernel, it has its roots in that blog series.
We strongly recommend you read through that series, as it has a plethora of good information for systems development in Rust.

## Documentation

s&ers code is thoroughly documented and gives numerous examples and notes.
You can read through the code to read it, check out the generated [rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) documentation (TODO: generate this and host it somewhere), or read through our documentation directory.

The [docs](./docs/readme.md) directory contains extensive documentation on numerous topics, including but not limited to project structure and management, safety of the code in s&ers, and systems development in Rust.
If you would like to read more in-depth information about anything related to s&ers, feel free to read through that directory.

## Usage

> Right now, s&ers is still quite early in development and prone to bugs and issues.
> It is not recommended using it in any production system.
> Additionally, development is done on Linux systems.
> It may not necessarily be impossible to work on it using other operating systems, however it is not officially supported.

All the necessary tools for building and running s&ers are included in our [Cargo](https://doc.rust-lang.org/cargo/) configuration.
The only external program you may need to install is [QEMU](https://www.qemu.org/).
QEMU makes it easy and fast to spin up virtual machines running s&ers and is fully integrated into our configuration.

### Rust nightly

s&ers uses certain Rust features that are experimental and only available in [Rust nightly](https://rust-lang.github.io/rustup/concepts/channels.html).
We are watching Rust closely, and excitedly waiting for these features to be stable, but until they are, you will need to install Rust nightly to be able to build and contribute to s&ers.
The easiest way to achieve this is through [rustup](https://rustup.rs/).
rustup is a program that makes it easy and fast to manage multiple Rust installations.

In order to install Rust nightly along with all the necessary components, simply run the following:

```commandline
~$ rustup update
```

From there, you should be able to use s&ers!

### Building

s&ers uses the [`bootimage`](https://crates.io/crates/bootimage) crate to generate bootable executables.
You will need to install it first if you plan on building a bootable image.
This is extremely simple, and all you need to do is run the following:

```commandline
~$ cargo install bootimage
```

To build s&ers into a bootable binary, you can simply run the following:

```commandline
~$ cargo bootimage
```

This will build s&ers in `target/x86_64-unknown-none/debug/bootimage-sanders.bin`.
This is a full-fat x86_64 binary that can be booted from.
You can then boot from it in a virtual machine or burn it to a USB using the following command (on Linux):

```commandline
~$ dd if=target/x86_64-unknown-none/debug/bootimage-sanders.bin of=/dev/sdX && sync
```

Where `sdX` is your USB device.
**NOTE:** make sure that this is your USB device, otherwise bad things can happen!

#### Release builds

Release builds can also be generated using Cargo.
These are builds that have more aggressive optimizations and are better suited for actual real-world usage.
_Again, s&ers is quite early in development, so it is recommended to not use it in the real world._
To generate a release build, simply run the following:

```commandline
~$ cargo bootimage --release
```

This will generate the binary in `target/x86_64-unknown-none/release/bootimage-sanders.bin` this time.

### Running

Running s&ers is extremely simple if you have QEMU installed.
We can let Cargo take care all the building and external command nonsense and just tell it to run our project.
We don't even need to create a build before we run it (although the run command will build the project if you haven't already).
Just keep in mind that running, by default, uses the `bootimage` crate, which is described in the [Building](#building) section.

To run s&ers in a virtual machine, using QEMU, run the following:

```commandline
~$ cargo run
```

This will build s&ers, generate a bootable disk image, then run QEMU using the generated image.

### Testing

Testing, similar to [running](#running), can be taken care of by Cargo.
Our configuration will automatically include all unit tests and integration tests, start them all in headless QEMU instances, and return the result.
To run these tests, simply run the following:

```commandline
~$ cargo test
```

## Status

Currently, s&ers is in early development.
Using the [list from OSDev.org](https://wiki.osdev.org/Creating_an_Operating_System), s&ers is currently in phase 1 out of 5.
It is not recommended to use s&ers in any real world application, however if you're interested in learning about operating systems or want to test out something new, feel free.

## MSRV

s&ers uses features only available in Rust nightly, so at this time, only the most recent nightly build is supported.

# License

s&ers is under the [MIT license](https://choosealicense.com/licenses/mit/).
You are allowed to use s&ers commercially, distribute it, modify it, and use it privately, so long as you include the license and copyright notices.
No liability nor warranty is provided.
More information can be found in the [LICENSE](./LICENSE) file.
