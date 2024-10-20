fn f(depth: i32, bottom: usize) {
    if depth % 1000 == 0 {
        // Get the current address of depth as a usize and compute the difference
        let current_address = &depth as *const i32 as usize;
        println!(
            "depth: {} address difference: {} current address: {:p}",
            depth,
            bottom - current_address,
            &depth
        );
    }
    f(depth + 1, bottom)
}

fn start() {
    let depth = 0;
    let bottom = &depth as *const i32 as usize; // Store the initial address
    f(depth, bottom)
}

fn main() {
    start();
}
