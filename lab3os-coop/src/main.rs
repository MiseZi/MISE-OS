//MAIN
#![no_std]                  //不使用标准库
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;



mod lang_items;
mod sbi;
mod console;
mod sync;
mod logging;
mod loader;
mod config;
pub mod syscall;
pub mod trap;
pub mod task;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_user.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clean_bss();
    println!("Hello, AmyYin!");
    println!("Hello, MiseZi!");
    trace!("Run normal.");
    error!("Run normal.");
    warn!("Run normal.");
    info!("Run normal.");
    debug!("Run normal.");
    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Shutdown!");
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