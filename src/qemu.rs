const QEMU_EXIT_DEVICE_PORT: u16 = 0xf4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    // Results in exit status of `(0x10 << 1) | 1 = 100`
    Success = 0x10,

    // Results in exit status of `(0x11 << 1) | 1 = 101`
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(QEMU_EXIT_DEVICE_PORT);
        port.write(exit_code as u32);
    }
}
