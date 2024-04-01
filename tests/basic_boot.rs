#![no_std] 
#![no_main] 
#![feature(custom_test_frameworks)] 

#![test_runner(ros::test_runner)]

#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo; 
use ros::println; 

// entry point for the program.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// panic handler for testing.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

// test case for println macro.
#[test_case]
fn test_println() {
    println!("test_println output");
}
