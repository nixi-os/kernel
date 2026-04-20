# Getting started

In this chapter we will go through the required steps for building nixi, and getting it running in a Virtual Machine.
You must install the dependencies used throughout this guide yourself, we are intentionally ambiguous about dependencies, as there is no finite dependency list.

## Building nixi
To build nixi from source you will first need to install Rust and Cargo.
It's recommended to install both of these through rustup, read the [rustup install guide](https://rust-lang.org/tools/install/) for more details.

Once Rust and Cargo is installed, you can start by cloning nixi:
```
git clone https://github.com/proxin187/nixi && cd nixi
```

You should now be able to run `cargo build` in the project root. This will place your EFI binary under `target/x86_64-unknown-uefi-kernel/debug/nixi.efi`.


## Running nixi in a Virtual Machine
At the current stage of development nixi only supports UEFI, for this reason the virtual machine setup has a few more hoops then a BIOS setup. Specifically, you will need [OVMF](https://github.com/tianocore/tianocore.github.io/wiki/OVMF) in order to run UEFI firmware inside QEMU.


