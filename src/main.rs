use chrono::{Local, Timelike};
use nix::sys::time::TimeSpec;
use nix::time::{ClockId, ClockNanosleepFlags, clock_nanosleep};
use std::fs;

fn main() {
    loop {
        let battery_charge = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").unwrap_or("0".to_string()).trim_end().to_string();
        //let battery_status = fs::read_to_string("/sys/class/power_supply/BAT0/status").unwrap_or("Unknown".to_string()).trim_end().to_string();
        let now = Local::now();
        // Year Month Day (Week of year - Day of week) Hour Minute
        println!("{} | {}%", now.format("%Y-%m-%d (%W-%w) %H:%M"), battery_charge,/*battery_status*/);

        // now.with_second(0) fails during DST transition
        // So use this instead of now.with_second(0) + chrono::Duration::minutes(1)
        let next_minute = now + chrono::Duration::seconds(60 - i64::from(now.second()));

        // This will block the thread until the specified wall-clock time
        clock_nanosleep(
            ClockId::CLOCK_REALTIME,
            ClockNanosleepFlags::TIMER_ABSTIME,
            &TimeSpec::new(next_minute.timestamp(), 0),
        )
        .expect("Error from clock_nanosleep");
    }
}
