# Introduction

nixi is an operating system written in rust for the x86_64 architecture. nixi is not intended for professional use, rather it's meant for hobbyists to experiment with features which would be unstable/risky to implement in professional grade operating systems.

Moreover, nixi aims to keep its codebase simple and bare bones, a future goal is to remove all dependencies as dependencies are fundamentally against the goal of nixi which is to be built from the ground up and experimental without using the *official* way to do everything. *notably, the UEFI crate is holding us back here*. 

Roadmap:
 - [X] Memory handling (PMA, and chunk allocator)
 - [X] Round-robin scheduler (in the future: different scheduler algorithms)
 - [X] Fast syscalls with *syscall* instruction
 - [ ] Basic graphics (VGA)
 - [ ] TTY/PTY system

## Contribution
All contributions are happily accepted, contributions must be formatted with [rustfmt](https://github.com/rust-lang/rustfmt).

## License
nixi is licensed under the MIT license.

