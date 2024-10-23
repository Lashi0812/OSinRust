use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use rand::Rng;

struct SharedData {
    num: Mutex<i32>,     // Shared value
    flag_a: Mutex<bool>, // Shared boolean flag
    flag_b: Mutex<bool>, // Another shared boolean flag
}

fn main() {
    let shared_pair = Arc::new(SharedData {
        num: Mutex::new(0),
        flag_a: Mutex::new(false),
        flag_b: Mutex::new(false),
    });

    // Clone Arc and spawn the "Fizz" thread
    let fizz_clone = Arc::clone(&shared_pair);
    thread::spawn(move || loop {
        let num = fizz_clone.num.lock().unwrap();
        let mut flag_a = fizz_clone.flag_a.lock().unwrap();
        if (*num % 3) == 0 && !*flag_a {
            *flag_a = true;
            print!("\tfizz");
        }
        drop(num);
        drop(flag_a); // Explicitly release the lock
    });

    // Clone Arc and spawn the "Buzz" thread
    let buzz_clone = Arc::clone(&shared_pair);
    thread::spawn(move || loop {
        let num = buzz_clone.num.lock().unwrap();
        let mut flag_b = buzz_clone.flag_b.lock().unwrap();
        if (*num % 5) == 0 && !*flag_b {
            *flag_b = true;
            print!("\tbuzz");
        }
        drop(num); // Explicitly release the lock
        drop(flag_b);
    });

    // Main thread to update the number randomly
    let mut rng = rand::thread_rng();
    loop {
        let mut num = shared_pair.num.lock().unwrap();
        let mut flag_a = shared_pair.flag_a.lock().unwrap();
        let mut flag_b = shared_pair.flag_b.lock().unwrap();

        *num = rng.gen_range(0..=16);
        *flag_a = false;
        *flag_b = false;

        print!("\n{} :", *num);
        sleep(Duration::from_micros(100000)); // Wait for 1 second
        drop(num); // Explicitly release the lock
        drop(flag_a);
        drop(flag_b);
    }
}
