// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 1..num + 1 {
        println!("Ring! Call number {}", i);
    }
}
