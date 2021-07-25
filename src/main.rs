#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

mod vga;
mod panic;
mod qemu;
mod serial;

use core::panic::PanicInfo;
use qemu::*;

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub trait Testable {
    fn run(&self) -> ();
}

impl <T> Testable for T
where
    T: Fn(),
{ 
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    loop {}
}

#[test_case]
fn passing() {
    assert_eq!(1, 1);
}

/*
// TODO: Stop using the library.
// use pc_keyboard::{layouts, HandleControl, KeyCode, KeyState, Keyboard, ScancodeSet1};
// TODO: Stop using the library.
// use uart_16550::SerialPort;
// use x86_64::instructions::port::Port;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // TODO: Implement a SpinLock.
    // Displaying to the screen.
    // TODO: Refactor into lazy_static and add support for println!.
    let vga_buffer = 0xb8000 as *mut u8;

    // Reading from the PCI bus.
    unsafe {
        let mut pci_port = Port::<u32>::new(0xCF8);

        let data = pci_port.read();

        // display(vga_buffer, data, 2, 0);
    };

    println!("TEST 123");

    // Input from a serial port.
    let mut keyboard_port = unsafe { SerialPort::new(0x60) };
    keyboard_port.init();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

    let mut pointer = 0;
    let mut prev = 0;

    loop {
        unsafe {
            let data = keyboard_port.receive();
            match keyboard.add_byte(data) {
                Ok(Some(event)) => {
                    let code = event.code as u8;
                    match event.state {
                        KeyState::Up => {
                            prev = 0;
                            continue;
                        }
                        KeyState::Down => {}
                    }
                    if prev == code {
                        continue;
                    }
                    prev = code;
                    match event.code {
                        KeyCode::Spacebar => {
                            pointer += 1;
                        }
                        KeyCode::Backspace => {
                            if pointer > 0 {
                                pointer -= 1;
                                write(vga_buffer, pointer, 0);
                            }
                        }
                        KeyCode::Escape => unsafe {
                            exit();
                        },
                        _ => {
                            if code < 78 {
                                continue;
                            }
                            write(vga_buffer, pointer, code - 78 + 97);
                            pointer += 1;
                            prev = code;
                        }
                    }
                }
                Ok(None) => {
                    continue;
                }
                Err(e) => {}
            }
        }
    }
    unsafe {
        exit();
    }

    loop {}
}

/// This function is called on panic.
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        exit();
    }
    loop {}
}

pub unsafe fn read(port: &mut Port<u32>) -> u32 {
    port.read()
}

pub unsafe fn exit() {
    let mut port = Port::<u32>::new(0xf4);
    port.write(54);
}

pub unsafe fn display(buffer: *mut u8, number: u32, base: u32, offset: usize) {
    let mut n = number % 1_000_000_000;
    let mut divisor = 1;
    let mut digits = 1;
    while divisor * base <= n {
        divisor *= base;
        digits += 1;
    }
    for i in 0..digits {
        let digit = n / divisor;
        write(buffer, offset + i, (digit + 48) as u8);
        n %= divisor;
        divisor /= base;
    }
}

pub unsafe fn write(buffer: *mut u8, i: usize, byte: u8) {
    *buffer.offset(i as isize * 2) = byte;
    *buffer.offset(i as isize * 2 + 1) = 0xb;
}
*/

