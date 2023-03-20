#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(dead_code)]

/// # How to use
/// 
/// ```ignore
/// stopwatch_start();
/// sleeps(1);
/// stopwatch_stop();
/// ```

use std::sync::Mutex;
use std::time::Instant;
use std::time::Duration;
use std::thread;

pub struct Stopwatch {
    start_time: Instant,
    stop_time: Instant,
}

lazy_static::lazy_static! {
    static ref STOPWATCH: Mutex<Option<Stopwatch>> = Mutex::new(None);
}

pub fn stopwatch_start() {

    println!("Stopwatch Start...");
    let stopwatch = Stopwatch {
        start_time: Instant::now(),
        stop_time: Instant::now(),
    };
    *STOPWATCH.lock().unwrap() = Some(stopwatch);
}

pub fn stopwatch_stop() {

    let mut elapsed_time: Duration = Duration::ZERO;
    let mut stopwatch_option = STOPWATCH.lock().unwrap();
    if let Some(ref mut stopwatch) = *stopwatch_option {
        stopwatch.stop_time = Instant::now();
        elapsed_time = stopwatch.stop_time - stopwatch.start_time;
    }
    *stopwatch_option = None;
    println!("Stopwatch Stop.... Elapsed time: {:?}", elapsed_time);
}

pub fn sleeps(seconds: u64) {
    std::thread::sleep(Duration::from_secs(seconds));
}

#[test]
fn test_stopwatch() {

    stopwatch_start();
    sleeps(1);
    stopwatch_stop();
}
