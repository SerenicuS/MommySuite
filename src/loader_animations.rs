use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_jitter(max: u64) -> u64 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    (nanos as u64) % max
}



pub enum AnimationType<'a> {
    Typewriter,
    Heartbeat,
    Glitch(&'a str), // Now it takes the ending word dynamically!
}
