#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pixie_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pixie_os::{print,println};

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pixie_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    print!("Test will run now...");
    println!("If you see this, Pixie OS is booting correctly!");

    pixie_os::init();

    // x86_64::instructions::interrupts::int3();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    #[cfg(test)]
    test_main();

    loop {}
}
