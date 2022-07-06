//! App management syscalls
use crate::batch::run_next_app;
use crate::batch::get_curr_app;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    run_next_app()
}

pub fn get_taskinfo() -> isize {
    get_curr_app();
    return 0 as isize;
}