use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(dur) => {
            let secs = dur.as_secs();
            let nanos = dur.subsec_nanos();
            println!("Current Time is: {}.{}s since UNIX_EPOCH", secs, nanos);
        }
        Err(_) => {
            println!("Current Time is: time before UNIX_EPOCH");
        }
    }
}