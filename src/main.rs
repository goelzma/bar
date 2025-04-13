use chrono::Local;
use std::thread;
use std::time;

fn main() {
    loop {
        let now = Local::now();
        println!("{}", now.format("%Y-%m-%d (%W-%w) %H:%M"));
        thread::sleep(time::Duration::from_secs(5));
    }
}
