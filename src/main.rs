#![no_std]
#![no_main]

mod vga;

use crate::vga::test_printing;
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Kernel running!";

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_printing();
    loop {}
}
