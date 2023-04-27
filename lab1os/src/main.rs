#![no_std]                  //不使用标准库
#![no_main]

use core::arch::global_asm;                 //不从main开始，直接编译
mod lang_items;

global_asm!(include_str!("entry.asm"));
