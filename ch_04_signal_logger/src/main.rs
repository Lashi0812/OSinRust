use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};

use nix::{
    libc,
    sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal},
};

static CHANGED: AtomicBool = AtomicBool::new(false);
static SIGNAL_NUMBER: AtomicI32 = AtomicI32::new(0);

extern "C" fn signal_handler(sig: libc::c_int) {
    CHANGED.store(true, Ordering::SeqCst);
    SIGNAL_NUMBER.store(sig, Ordering::SeqCst);
}
fn main() {
    let sig_action = SigAction::new(
        SigHandler::Handler(signal_handler),
        SaFlags::empty(),
        SigSet::empty(),
    );
    for signal in Signal::iterator() {
        if signal != Signal::SIGKILL && signal != Signal::SIGSTOP {
            unsafe {
                sigaction(signal, &sig_action).unwrap();
            }
        }
    }

    loop {
        if CHANGED.load(Ordering::SeqCst) {
            println!("Signal received {}", SIGNAL_NUMBER.load(Ordering::SeqCst));
            CHANGED.store(false, Ordering::SeqCst);
        }
    }
}
