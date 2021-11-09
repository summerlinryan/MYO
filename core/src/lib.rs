#![no_std]
#![feature(custom_test_frameworks)]
#![test_runner(crate::testing::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate lazy_static;
extern crate spin;

pub mod qemu;
pub mod serial;
pub mod test;
pub mod vga;
