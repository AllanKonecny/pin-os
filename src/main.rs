#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pine_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use pine_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Type anything");

    pine_os::init();

    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    // #[cfg(test)]
    // test_main();
    pine_os::halt();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    pine_os::halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
