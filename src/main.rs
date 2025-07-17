use chrono::{Timelike, Local};
use std::time::Duration;
use libc::{clock_nanosleep, CLOCK_REALTIME, TIMER_ABSTIME};

fn main() {
    loop {
        let now = Local::now();
        // Year Month Day (Week of year - Day of week) Hour Minute
        println!("{}", now.format("%Y-%m-%d (%W-%w) %H:%M"));

        // Calculate the target time: beginning of the next minute
        let mut next_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap();
        if next_minute <= now {
            next_minute = next_minute + chrono::Duration::minutes(1);
        }

        // Convert the target time to a timespec struct for clock_nanosleep
        let target_timestamp_secs = next_minute.timestamp();
        let target_timestamp_nanos = next_minute.nanosecond();

        let ts = libc::timespec {
            tv_sec: target_timestamp_secs,
            tv_nsec: target_timestamp_nanos as libc::c_long,
        };

        // Call clock_nanosleep with CLOCK_REALTIME and TIMER_ABSTIME
        // This will block the thread until the specified wall-clock time
        let res = unsafe {
            clock_nanosleep(
                CLOCK_REALTIME,
                TIMER_ABSTIME,
                &ts,
                std::ptr::null_mut(), // No remainder
            )
        };

        if res != 0 {
            let err = std::io::Error::from_raw_os_error(res);
            panic!("clock_nanosleep failed: {}", err);
        }
    }
}
