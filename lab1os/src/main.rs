#![no_std]                  //不使用标准库
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;

use sbi::console_putchar;

use crate::sbi::shutdown;                 //不从main开始，直接编译
mod lang_items;
mod sbi;
mod console;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clean_bss();
    println!("Hello, AmyYin!");
    println!("Hello, MiseZi!");
    panic!("Shutdown!");
    //loop {}
}

fn clean_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}