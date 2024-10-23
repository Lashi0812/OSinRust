use std::thread;

const EACH_COUNT: usize = 1_000_000;
static mut COUNTER: usize = 0; // Unsafe static mutable variable

// static COUNTER: AtomicUsize = AtomicUsize::new(0);

// fn fcounter() {
//     for _ in 0..EACH_COUNT {
//         COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
//     }
// }

unsafe fn fcounter() {
    for _ in 0..EACH_COUNT {
        let ptr = std::ptr::addr_of_mut!(COUNTER) as *mut usize;

        // Perform a volatile read
        let value = std::ptr::read_volatile(ptr);

        // Increment the value
        let new_value = value + 1;

        // Perform a volatile write
        std::ptr::write_volatile(ptr, new_value);
    }
}

fn main() {
    let t1 = thread::spawn(|| unsafe {
        fcounter();
    });
    let t2 = thread::spawn(|| unsafe {
        fcounter();
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!(
        "Expected count to be {} but it was {}",
        2 * EACH_COUNT,
        // COUNTER.load(std::sync::atomic::Ordering::SeqCst)
        unsafe { std::ptr::read_volatile(std::ptr::addr_of!(COUNTER) as *const usize) }
    );
}
