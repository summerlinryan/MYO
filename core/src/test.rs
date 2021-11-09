use crate::qemu::{exit_qemu, QemuExitCode};
use crate::{serial_print, serial_println};
use core::panic::PanicInfo;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) -> () {
        serial_print!("{}...", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(test_functions: &[&dyn Testable]) {
    serial_println!("Executing {} tests", test_functions.len());
    for test in test_functions {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}
