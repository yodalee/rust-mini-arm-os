#![no_std]
#![feature(lang_items)]

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () { loop {} }

#[no_mangle]
pub extern fn rust_main() {
    loop {}
}
