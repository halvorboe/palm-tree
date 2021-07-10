#![no_std]
#![no_main]

use core::panic::PanicInfo;
use pc_keyboard::{layouts, HandleControl, KeyState, Keyboard, ScancodeSet2};
use x86_64::instructions::port::Port;

static SERIAL_IO_PORT: u16 = 0xf4;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let mut keyboard = Port::<u32>::new(0x60);
    /*
    let mut keyboard = Keyboard::new(
        layouts::Uk105Key,
        ScancodeSet2,
        HandleControl::MapLettersToUnicode,
    );*/

    unsafe {
        write(vga_buffer, 0, 0);
    }

    unsafe {
        loop {
            let data = read(&mut keyboard);
            display(vga_buffer, data);
            /*
            match keyboard.add_byte(data) {
                Ok(Some(event)) => match event.state {
                    KeyState::Down => {
                        let event_code = event.code as u32;
                        output_number(vga_buffer, event_code);
                    }
                    _ => {
                        break;
                    }
                },
                Ok(None) => {
                    continue;
                }
                Err(e) => {
                    output(vga_buffer, 10, 42);
                }
            }
            */
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

pub unsafe fn display(buffer: *mut u8, number: u32) {
    let mut n = number;
    let mut divisor = 1;
    let mut digits = 1;
    while n >= divisor * 10 {
        divisor *= 10;
        digits += 1;
    }
    for i in 0..digits {
        // let digit = n / divisor;
        let digit = number % 10;
        write(buffer, i, (digit + 48) as u8);
        // n %= divisor;
        // divisor /= 10;
    }
}

pub unsafe fn write(buffer: *mut u8, i: usize, byte: u8) {
    *buffer.offset(i as isize * 2) = byte;
    *buffer.offset(i as isize * 2 + 1) = 0xb;
}
