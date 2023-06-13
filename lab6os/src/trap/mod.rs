//! System trap handler.


use core::arch::{global_asm, asm};
use riscv::register::{stvec, mtvec::TrapMode, scause::{self, Trap, Exception, Interrupt}, stval, sie, sstatus};
use crate::{syscall::syscall, timer::set_next_trigger, task::{suspend_current_and_run_next, exit_current_and_run_next, processor::{current_trap_cx, current_user_token}}, println, config::{TRAMPOLINE, TRAP_CONTEXT}};
pub use context::TrapContext;

mod context;

global_asm!(include_str!("trap.S"));

pub fn init() {
    set_kernel_trap_entry();
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
/// handle an interrupt, exception, or system call from user space
pub fn trap_handler() -> ! {
    set_kernel_trap_entry();
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            let mut cx = current_trap_cx();
            cx.sepc += 4;
            let result =syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
            cx = current_trap_cx();
            cx.x[10] = result;
        }
        Trap::Exception(Exception::StoreFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::LoadFault)
        | Trap::Exception(Exception::LoadPageFault) => {
            println!("[kernel] {:?} in application, bad addr = {:#x}, bad instruction = {:#x}, core dumped.", 
            scause.cause(), 
            stval, 
            current_trap_cx().sepc
        );
            exit_current_and_run_next(-2);
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next(-3);
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    trap_return();
}

#[no_mangle]
/// set the new addr of __restore asm function in TRAMPOLINE page,
/// set the reg a0 = trap_cx_ptr, reg a1 = phy addr of usr page table,
/// finally, jump to new addr of __restore asm function
pub fn trap_return() -> ! {
    set_user_trap_entry();
    let trap_cx_ptr = TRAP_CONTEXT;
    let user_satp = current_user_token();
    extern "C" {
        fn __alltraps();
        fn __restore();
    }
    let restore_va = __restore as usize - __alltraps as usize + TRAMPOLINE;
    unsafe {
        asm!(
            "fence.i",
            "jr {restore_va}",             // jump to new addr of __restore asm function
            restore_va = in(reg) restore_va,
            in("a0") trap_cx_ptr,      // a0 = virt addr of Trap Context
            in("a1") user_satp,        // a1 = phy addr of usr page table
            options(noreturn)
        );
    }
}

fn set_user_trap_entry() {
    unsafe {
        stvec::write(TRAMPOLINE as usize, TrapMode::Direct);
    }
}


fn set_kernel_trap_entry() {
    unsafe {
        stvec::write(trap_from_kernel as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_from_kernel() -> ! {
    panic!("a trap from kernel!");
}