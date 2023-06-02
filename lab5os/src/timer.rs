use riscv::register::time;
use crate::{config::CLOCK_FREQ, sbi::set_timer};

const TICKS_PRE_SEC: usize = 100;
const MSEC_PRE_SEC: usize = 1000;

pub fn get_time() -> usize {
    time::read()
}

pub fn set_next_trigger() {
    set_timer( get_time() + CLOCK_FREQ / TICKS_PRE_SEC );
}

pub fn get_time_ms() -> usize {
    time::read() / ( CLOCK_FREQ / MSEC_PRE_SEC )
}