# PalmTree OS

Imagine that you're stuck on a desert island and you're only option to escape is an eithernet cable leading to the mainland and an x86 computer without an operating system. 

The point of this project is to get a deeper understanding of how OS are implemented and the full networking stack from hardware to HTTP.

I'm using this post as a reference to begin with:
https://os.phil-opp.com/

## 
x86_64 - basic x86 instructions.
bootloader - library to boot rust code.
lazy_static - allows for static values that are loaded at run time.

I'll use some libraries while developing, but I want to try to implement as much as possible from scratch. 

## Goals
### Setup
- [X] Display output on the screen
- [X] Take input from the user.
- [X] Can read data from bus.

### Barebones OS 
- [ ] Basic utility functions. Macros for displaying on the screen.
- [ ] Interrupt and exception handling.
- [ ] Heap allocation and deallocation. 
- [ ] Task running.

### Features
- [ ] Basic shell.
- [ ] PCI interface. Discovery, initialization.
- [ ] Serial interface. (using library for now)
- [ ] Keyboard input. (using library for now)
- [ ] System calls and user space.
- [ ] Networking stack.
- [ ] Basic file system.
- [ ] File system.
