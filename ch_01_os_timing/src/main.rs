use libc::{getrusage, rusage, RUSAGE_SELF};
use std::{
    alloc::{alloc, Layout},
    thread::sleep,
    time::{Duration, Instant},
};

const NUM_MULS: usize = 100_000_000;
const NUM_MALLOCS: usize = 100_000;
const MALLOC_SIZE: usize = 1000; // Size in bytes
const SLEEP_SEC: u64 = 3;

struct ProfileTimes {
    user_usec: i64,
    kernel_usec: i64,
    real_usec: Instant,
}

impl ProfileTimes {
    fn start() -> ProfileTimes {
        let mut usage: rusage = unsafe { std::mem::zeroed() };
        unsafe {
            getrusage(RUSAGE_SELF, &mut usage);
        }
        Self {
            user_usec: usage.ru_utime.tv_sec * 1000000 + usage.ru_utime.tv_usec,
            kernel_usec: usage.ru_stime.tv_sec * 1000000 + usage.ru_stime.tv_usec,
            real_usec: std::time::Instant::now(),
        }
    }

    fn end(&self) -> ProfileTimes {
        let mut usage: rusage = unsafe { std::mem::zeroed() };
        unsafe {
            getrusage(RUSAGE_SELF, &mut usage);
        }
        Self {
            user_usec: usage.ru_utime.tv_sec * 1000000 + usage.ru_utime.tv_usec - self.user_usec,
            kernel_usec: usage.ru_stime.tv_sec * 1000000 + usage.ru_stime.tv_usec
                - self.kernel_usec,
            real_usec: self.real_usec,
        }
    }

    fn end_and_logs(&self) {
        self.end();
        let pid = std::process::id();
        println!(
            "pid : {:?} ,real : {:.2?} , user : {:?} , kernel : {:?} sec",
            pid,
            self.real_usec.elapsed().as_micros() as f64 / 1_000_000f64,
            self.user_usec as f64 / 1_000_000f64,
            self.kernel_usec as f64 / 1_000_000f64
        );
    }
}

fn main() {
    let profile = ProfileTimes::start();
    let mut _x = 1.0;
    for _ in 0..NUM_MULS {
        _x *= 1.1;
    }
    profile.end_and_logs();

    let profile = ProfileTimes::start();
    let layout = Layout::from_size_align(MALLOC_SIZE, 1).unwrap();
    for _ in 0..NUM_MALLOCS {
        unsafe {
            let _p = alloc(layout); // Allocate memory of size MALLOC_SIZE
        }
    }
    profile.end_and_logs();

    let profile = ProfileTimes::start();
    sleep(Duration::from_secs(SLEEP_SEC)); // Sleep for SLEEP_SEC seconds
    profile.end_and_logs();
}
