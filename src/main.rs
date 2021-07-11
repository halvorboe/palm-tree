#![no_std]
#![no_main]

use core::panic::PanicInfo;
use pc_keyboard::{layouts, HandleControl, KeyState, Keyboard, ScancodeSet1};
use uart_16550::SerialPort;
use x86_64::instructions::port::Port;

static SERIAL_IO_PORT: u16 = 0xf4;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut keyboard_port = unsafe { SerialPort::new(0x60) };
    keyboard_port.init();
    let mut keyboard = Keyboard::new(
        layouts::Us104Key,
        ScancodeSet1,
        HandleControl::MapLettersToUnicode,
    );

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
                    if code < 78 || prev == code {
                        continue;
                    }
                    write(vga_buffer, pointer, code - 78 + 97);
                    pointer += 1;
                    prev = code;
                }
                Ok(None) => {
                    continue;
                }
                Err(e) => {
                    write(vga_buffer, 40, 42);
                }
            }

            //display(vga_buffer, data, 2, 100);
            //display(vga_buffer, data >> 24, 2, 0);
            //display(vga_buffer, data >> 24, 10, 40);
            //display(vga_buffer, (data >> 16) % 256, 2, 9);
            //display(vga_buffer, (data >> 8) % 256, 2, 18);
            // display(vga_buffer, data, 2, 0);
            // display(vga_buffer, data, 10, 10);
        }
    }
    unsafe {
        exit();
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
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
