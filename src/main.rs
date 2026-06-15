use chrono::{Local, Timelike};
use nix::sys::time::TimeSpec;
use nix::time::{ClockId, ClockNanosleepFlags, clock_nanosleep};
use std::fs;

fn main() {
    loop {
        let battery_charge = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or("0".to_string()).trim_end().to_string();
        let battery_status = fs::read_to_string("/sys/class/power_supply/BAT0/status").unwrap_or("Unknown".to_string()).trim_end().to_string();
        let now = Local::now();
        // Year Month Day (Week of year - Day of week) Hour Minute
        println!("{} | {}% | {}", now.format("%Y-%m-%d (%W-%w) %H:%M"), battery_charge, battery_status);

        let start_of_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap();
        let next_minute = start_of_minute + chrono::Duration::minutes(1);

        // This will block the thread until the specified wall-clock time
        clock_nanosleep(
            ClockId::CLOCK_REALTIME,
            ClockNanosleepFlags::TIMER_ABSTIME,
            &TimeSpec::new(
                next_minute.timestamp(),
                // TODO: Always zero? Or perhaps not because of hypothetical sub-second timezone fuckery?
                next_minute.nanosecond() as i64,
            ),
        )
        .expect("Error from clock_nanosleep");
    }
}
