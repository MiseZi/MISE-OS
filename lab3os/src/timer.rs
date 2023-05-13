use riscv::regieter::time;
use crate::config::CLOCK_FREQ;

const TICKS_PRE_SEC: usize = 100;
const MICRO_PRE_SEC: usize = 1_000_000;

pub fn get_time() -> usize {
    time::read();
}

pub fn set_next_trigger() {
    set_timer(ger_time() + CLOCK_FREQ / TICKS_PRE_SEC);
}

pub fn get_time_us() -> usize {
    time::read() / ( CLOCK_FREQ / TICKS_PRE_SEC );
}