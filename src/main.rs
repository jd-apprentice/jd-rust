use std::thread::sleep;

#[path = "1_module/tests/add_sub.rs"]
mod add_sub;
#[path = "2_module/tests/mul_div.rs"]
mod mul_div;

fn main() {
    const SLEEP_MESSAGE: &str = "Sleep forever...";
    println!("{}", SLEEP_MESSAGE);
    sleep(std::time::Duration::from_millis(f64::INFINITY as u64));
}
