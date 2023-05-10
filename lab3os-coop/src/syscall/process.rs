use crate:: info;

pub fn sys_exit(xstate: i32) -> ! {
    info!("[kernel] Application exited with code {}", xstate);
    loop{}
    //run_next_app()
}