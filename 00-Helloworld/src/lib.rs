#![no_std]
#![crate_type="staticlib"]
#![feature(lang_items)]
#![feature(asm)]

mod reg;

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}

#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr0() -> () { loop {} }
#[no_mangle]
pub unsafe fn __aeabi_unwind_cpp_pr1() { loop {} }

extern crate rlibc;

const USART_FLAG_TXE: u16 = 0x0080;

pub fn puts(s: &str) {
    for c in s.chars() {
        unsafe {
            while !(*(reg::USART2_SR as *mut u32) & USART_FLAG_TXE as u32 != 0) {}
            *(reg::USART2_DR as *mut u32) = c as u32;
        }
    }
}

#[no_mangle]
pub extern fn rust_main() {
    unsafe { *(reg::RCC_APB2ENR as *mut u32) |= 0x00000001 | 0x00000004 };
    unsafe { *(reg::RCC_APB1ENR as *mut u32) |= 0x00020000 };
    unsafe { *(reg::GPIOA_CRL as *mut u32) = 0x00004b00 };
    unsafe { *(reg::GPIOA_CRH as *mut u32) = 0x44444444 };
    unsafe { *(reg::USART2_CR1 as *mut u32) = 0x0000000C };
    unsafe { *(reg::USART2_CR1 as *mut u32) |= 0x2000 };

    puts("Hello World!\n");
}
