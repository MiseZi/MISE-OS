//! The Entry of rCore

#![no_std]                  //不使用标准库
#![no_main]                 //不使用自带main函数，使用rust_main()
//#![deny(missing_docs)]
//#![deny(warnings)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;

#[macro_use]
extern crate bitflags;

use core::arch::global_asm;

#[macro_use]
mod console;
#[macro_use]
mod logging;
mod config;
mod lang_items;
mod loader;
mod mm;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
mod timer;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    clean_bss();
    println!("Hello, AmyYin!");
    println!("Hello, MiseZi!");
    mm::init();
    debug!("mm init done");
    mm::remap_test();
    debug!("remap_test done");
    trap::init();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    trace!("Run normal.");
    error!("Run normal.");
    warn!("Run normal.");
    info!("Run normal.");
    debug!("Run normal.");
    task::run_first_task();
    panic!("Shutdown!");
}

fn clean_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}