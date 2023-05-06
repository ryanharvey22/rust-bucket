#![no_std] // disable standard library
#![no_main] // disable entry at main funnction

use core::panic::PanicInfo;

#[no_mangle] // dont magle the name of this function
pub extern "C" fn _start() -> ! {
    // This is the start function aka entry point
    // the linker looks for the _start function by default 
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}