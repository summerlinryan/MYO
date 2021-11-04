#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Kernel running!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;

    for i in 0..50 {
        println!("Row: {}", i);
    }

    loop {}
}
