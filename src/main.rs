
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;



#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from TripleFault OS!");
    println!("This is version {}.", 0.03);
    // panic!("Triple Fault.");
    loop {}
}


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}