//! System trap handler.


use core::arch::global_asm;
use riscv::register::{stvec, mtvec::TrapMode, scause::{self, Trap, Exception}, stval, sie, sstatus};
use crate::{syscall::syscall, warn, timer::set_next_trigger, task::suspend_current_and_run_next, println};
pub use context::TrapContext;

mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    extern "C" { fn __alltraps(); }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

pub fn enable_kernel_interrupt() {
    unsafe { sstatus::set_sie(); }
}

pub fn disable_kernel_interrupt() {
    unsafe { sstatus::clear_sie(); }
}

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer(); }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    match sstatus::read().spp() {
        sstatus::SPP::Supervisor => kernel_trap_handler(cx),
        sstatus::SPP::User => user_trap_handler(cx),
    }
}

static mut KERNEL_INTERRUPT_TRIGGERED: bool = false;

/// 检查内核中断是否触发
pub fn check_kernel_interrupt() -> bool {
    unsafe { (&mut KERNEL_INTERRUPT_TRIGGERED as *mut bool).read_volatile() }
}

/// 标记内核中断已触发
pub fn trigger_kernel_interrupt() {
    unsafe {
        (&mut KERNEL_INTERRUPT_TRIGGERED as *mut bool).write_volatile(true);
    }
}

pub fn kernel_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Interrupt(scause::Interrupt::SupervisorTimer) => {
            println!("kernel interrupt: from timer");
            set_next_trigger();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            panic!("[kernel] PageFault in kernel, bad addr = {:#x}, bad instruction = {:#x}, kernel killed it.", stval, cx.sepc);
        }
        _ => {
            panic!("unknown kernel exception or interrupt");
        }
    }
    trigger_kernel_interrupt();
    cx
}

pub fn user_trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    crate::task::user_time_end();
    let scause = scause::read();
    let stval = stval::read();

    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            warn!("[kernel] PageFault in application, kernel killed it.");
            crate::task::exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            warn!("[kernel] IllegalInstruction in application, kernel killed it.");
            crate::task::exit_current_and_run_next();
        }
        Trap::Interrupt(scause::Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    crate::task::user_time_start();
    cx
}