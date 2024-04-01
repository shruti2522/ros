#![no_std] 

#![no_main] 

#![feature(custom_test_frameworks)] 

// specifies the test runner function for the crate.
#![test_runner(ros::test_runner)]

// specifies the entry point for the test harness.
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo; 
use ros::println;.

// panic handler for non-test mode.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler for test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

// entry point for the program.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    
    #[cfg(test)]
    test_main();
    
    loop{}
}

// test case demonstrating a trivial assertion.
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
