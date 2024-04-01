#![no_std] 
#![no_main] 

use core::panic::PanicInfo; 
use ros::{QemuExitCode, exit_qemu, serial_println, serial_print}; 

// entry point for the program.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    
    // loop indefinitely to prevent the function from returning.
    loop {}
}

// function that is expected to fail.
fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

// panic handler for the program.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    
    loop {}
}
