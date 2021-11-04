#![no_std]
#![no_main]

mod vga;

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Kernel running!";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    panic!("Panic message");
}
