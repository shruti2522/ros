#![no_std] 

// disables the main function for testing, allowing us to define our own entry point.

#![cfg_attr(test, no_main)]

#![feature(custom_test_frameworks)]

#![test_runner(crate::test_runner)]

// Specifies the entry point for the test harness.
#![reexport_test_harness_main = "test_main"]

extern crate rlibc; 

// Module declarations.
pub mod serial;
pub mod vga_buffer;

// Trait definition for types that can be tested.
pub trait Testable {
    fn run(&self) -> ();
}

// Implementation of Testable trait for any type that is a function.
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

// function to run tests.
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

// function to handle panic during testing.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

// enum to represent exit codes for QEMU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

// function to exit QEMU with specified exit code.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/// entry point for `cargo xtest`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// Panic handler for testing.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
