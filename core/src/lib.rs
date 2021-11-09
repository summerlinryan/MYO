#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod qemu;
pub mod serial;
pub mod test;
pub mod vga;

pub fn init() {
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3();
}
