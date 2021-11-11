#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_core::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use kernel_core::println;

use core::panic::PanicInfo;
use kernel_core::qemu::{exit_qemu, QemuExitCode};
use kernel_core::serial_println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    println!("Kernel starting...");

    kernel_core::init();

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    stack_overflow();

    loop {}
}
