#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Welcome to TripleFault OS!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer_address = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer_address.offset(i as isize * 2) = byte;
            // set color metadata
            // no blink, foreground color of 1011, cyan
            *vga_buffer_address.offset(i as isize * 2 + 1) = 0x0b;
        }
    }
    loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}