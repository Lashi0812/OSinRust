use std::{ffi::CString, process::exit};

use nix::{
    sys::wait::waitpid,
    unistd::{execvp, fork, ForkResult},
};

fn main() {
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("I'm the parent, my child is {}", child);
            waitpid(child, None).unwrap();
        }
        Ok(ForkResult::Child) => {
            execvp(&CString::new("ls").unwrap(), &[CString::new("ls").unwrap()])
                .expect("execlp failed");
            exit(1);
        }
        Err(err) => {
            // Handle fork failure
            eprintln!("Fork failed: {}", err);
            exit(1);
        }
    }
}
