use chrono::{Local, Timelike};
use nix::time::{ClockId, ClockNanosleepFlags, clock_nanosleep};
use nix::sys::time::TimeSpec;

fn main() {
    loop {
        let now = Local::now();
        // Year Month Day (Week of year - Day of week) Hour Minute
        println!("{}", now.format("%Y-%m-%d (%W-%w) %H:%M"));

        // Calculate the target time: beginning of the next minute
        let start_of_minute = now.with_second(0).unwrap().with_nanosecond(0).unwrap();
        let next_minute = start_of_minute + chrono::Duration::minutes(1);

        // Convert the target time to a timespec struct for clock_nanosleep
        let ts = TimeSpec::new(
            next_minute.timestamp(),
            // TODO: Always zero? Or perhaps not because of hypothetical utc offset which is not an integral number of seconds?
            next_minute.nanosecond() as i64,
        );

        // Call clock_nanosleep with CLOCK_REALTIME and TIMER_ABSTIME
        // This will block the thread until the specified wall-clock time
        clock_nanosleep(
            ClockId::CLOCK_REALTIME,
            ClockNanosleepFlags::TIMER_ABSTIME,
            &ts,
        ).expect("Error from clock_nanosleep");
    }
}
