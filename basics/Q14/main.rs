use chrono::{Local, SecondsFormat};
use std::time::Instant;

fn main() {
    let system_time = Local::now();
    let instant = Instant::now();

    let formatted_time = system_time.to_rfc3339_opts(SecondsFormat::Nanos, true);
    println!("now: {} m=+{:.9}", formatted_time, instant.elapsed().as_secs_f64());
}