#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pixie_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use pixie_os::println;

extern crate alloc;

use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use pixie_os::{
        allocator,
        memory::{self, BootInfoFrameAllocator},
    };
    use x86_64::VirtAddr;

    println!("If you see this, Pixie OS is booting correctly!");
    pixie_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x = Box::new(41);
    println!("heap_value: {:p}", x);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }

    println!("{:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference_counted = reference_counted.clone();
    println!("Current reference count: {}", Rc::strong_count(&reference_counted));
    core::mem::drop(reference_counted);
    println!("After dropping one reference: {}", Rc::strong_count(&cloned_reference_counted));

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    pixie_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pixie_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    pixie_os::test_panic_handler(info);
}
