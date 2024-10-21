// Use the `strace` command to trace the program
// strace ./target/debug/ch_03a_simple_print_strace
// look for write syscall and exit group

fn main() {
    println!("Hello, world!");
}
