# PalmTree OS

Imagine that you're stuck on a desert island and you're only option to escape is an eithernet cable leading to the mainland and an x86 computer without an operating system. What is the minimal OS needed to connect to the internet.

This chances of this project being completed are low, but hopefully I'll learn something along the way.

## Starting point

I've looked around for minimal OS implementations. I want to write my OS in Rust. I'm basing it of the example in:

https://os.phil-opp.com/

https://github.com/rust-osdev/uart_16550

https://github.com/thepowersgang/rust_os/blob/68ae46366ccf5fe0dcb96d9437dc6a0255ef501a/Kernel/Core/arch/amd64/pci.rs

## Goals
### External connection
- [X] Display output on the screen
- [X] Take input from the user.
- [ ] Can read data from PCI.

### Barebones OS 
- [ ] Basic utility functions. Macros for displaying on the screen.
- [ ] Interrupt and exception handling.
- [ ] Heap allocation and deallocation. 

### Features
- [ ] System calls and user space.
- [ ] Networking stack.
- [ ] Basic file system.
- [ ] File system.
