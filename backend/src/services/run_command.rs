// use std::process;
use execute::shell;
// use std::{ffi::CStr, process};
// use cstr::cstr;
// use nix::unistd::{fork, ForkResult, execv};
// use crate::errors::ServerError;
// // use tokio::process::Command;

pub async fn run_command_detached(_cmd: String){
    // match unsafe {fork()} {
    //     Ok(ForkResult::Child) => {
    //         std::process::Command::new(&cmd).spawn().expect("Failed to spawn process");
    //         // let a = cstr!(cmd);
    //         // let args: &[&CStr] = &[];
    //         // let output = execv(&a, args);
    //         unsafe {libc::_exit(0)};
    //     }
    //     Ok(ForkResult::Parent{child, ..}) => {
    //         println!("Child pid: {}", child);
    //         nix::sys::wait::waitpid(child, None).unwrap();
    //     }
    //     Err(e) => {
    //         panic!("Failed to fork {}", e);
    //     }
    // }
    todo!("not implemented");
}

pub fn run_command(cmd: String){
    let mut exec = shell(cmd);
    exec.spawn().expect("Failed to spawn process");
}
