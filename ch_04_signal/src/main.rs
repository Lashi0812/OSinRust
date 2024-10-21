use std::sync::atomic::{AtomicBool, Ordering};

use nix::{
    libc::{self, ioctl, TIOCGWINSZ},
    sys::signal::{sigaction, SaFlags, SigAction, SigHandler, SigSet, Signal},
};

static RESIZED: AtomicBool = AtomicBool::new(false); // Atomic flag to indicate resizing

extern "C" fn signal_handler(_: libc::c_int) {
    RESIZED.store(true, Ordering::SeqCst); // Set the resized flag to true
}

fn main() {
    let mut winsize = libc::winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    let sig_action = SigAction::new(
        SigHandler::Handler(signal_handler),
        SaFlags::empty(),
        SigSet::empty(),
    );
    unsafe {
        sigaction(Signal::SIGWINCH, &sig_action).unwrap();
    }

    loop {
        if RESIZED.load(Ordering::SeqCst) {
            unsafe { ioctl(0, TIOCGWINSZ as _, &mut winsize) };

            println!(
                "Terminal size: {} rows, {} cols",
                winsize.ws_row, winsize.ws_col
            );
            RESIZED.store(false, Ordering::SeqCst); // Reset the flag
        }
    }
}
