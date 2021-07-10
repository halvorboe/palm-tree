#![no_std]
#![no_main]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

static DOT: &[u8] = b".";
static SERIAL_IO_PORT: u16 = 0xf4;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    unsafe {
        output(vga_buffer, 0);
    }

    unsafe {
        loop {
            let data = read_from_qemu();
            if data != 0 {
                output(vga_buffer, data as u8);
            }
        }
    }

    loop {}

    unsafe {
        exit_qemu();
    }

    loop {}
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

pub unsafe fn read_from_qemu() -> u32 {
    let mut port = Port::<u32>::new(0x60);
    port.read()
    
}

pub unsafe fn exit_qemu() {
    let mut port = Port::<u32>::new(0xf4);
    port.write(54); // exit code is (54 << 1) | 1 = 109
}

pub unsafe fn output(vga_buffer: *mut u8, byte: u8) {
    const i: usize = 0;
    *vga_buffer.offset(i as isize * 2) = byte;
    *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
}
