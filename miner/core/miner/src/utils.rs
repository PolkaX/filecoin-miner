use std::thread;
use std::time::{Duration, SystemTime};

fn now_timestamp() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn wait_until(end_timestamp: u64) {
    thread::sleep(Duration::from_secs(end_timestamp - now_timestamp()))
}

pub fn sleep(sec: u64) {
    thread::sleep(Duration::from_secs(sec));
}
