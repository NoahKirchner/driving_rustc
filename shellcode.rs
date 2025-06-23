#![no_std]
#![no_main]
#![feature(start)]
use core::panic::PanicInfo;

#[no_mangle]
#[link_section = ".text.payload"]
pub unsafe extern "C" fn _start()->i32{
    return 0;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
