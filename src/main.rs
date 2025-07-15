use chrono::{Local, Timelike};
use std::thread;

fn main() {
    loop {
        let now = Local::now();
        // Year Month Day (Week of year - Day of week) Hour Minute
        println!("{}", now.format("%Y-%m-%d (%W-%w) %H:%M"));

        let sleep_duration = chrono::Duration::seconds(60)
            - chrono::Duration::seconds(now.second() as i64)
            - chrono::Duration::nanoseconds(now.nanosecond() as i64);

        thread::sleep(sleep_duration.to_std().unwrap());
    }
}
