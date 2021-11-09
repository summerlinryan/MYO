#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernel_core::qemu;
use kernel_core::serial;
use kernel_core::vga;
use kernel_core::{print, println};

use core::panic::PanicInfo;
extern crate core;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    println!("Kernel starting...");
    loop {}
}
