#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!!");
    println!("Hello World{}{}", "!!", "2");
    println!("The numbers are {0} and {1}", 42, 1.337);

    panic!("Oh no!");

    loop {}
}