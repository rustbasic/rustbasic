//! # Rust Basic
//!
//! `RustBasic` is a planned development that aims to make Rust easy to learn, teach, and use.

// Errexit()
use std::fmt;
// Stopwatch
use std::time::Instant;
use std::time::Duration;
use std::thread;
use std::sync::Mutex;

pub trait Errexit<T> {

    /// This function implements `unwrap()` as `errexit()`. 
    ///
    /// `errexit()` is used in `Option` and `Result` types, and returns the internal value if the corresponding value is `Some` or `Ok`. 
    /// If it is `None` or `Err`, it panics by default. 
    ///
    /// Therefore, `errexit()` should be used only when it is certain that a value exists, allowing you to safely extract that value and proceed to the next step.
    ///   
    /// # Examples
    /// 
    /// ```ignore
    /// use rustbasic::*;
    /// 
    /// let _a: Option<usize> = Some(123);
    /// let _b: Option<usize> = None;
    /// let _c = get_ok();
    /// let _d = get_err();
    /// println!("{:?}", _a.errexit());
    /// // println!("{:?}", _b.errexit());   // It panics!
    /// println!("{:?}", _c.errexit());
    /// // println!("{:?}", _d.errexit());   // It panics!
    /// ```
    fn errexit(self) -> T;
}

impl<T> Errexit<T> for Option<T> {
    fn errexit(self) -> T {
        match self {
            Some(t) => t,
            None => self.unwrap(), // `unwrap()` panics for `None`
        }
    }
}

impl<T, E> Errexit<T> for Result<T, E> where E: fmt::Debug {
    fn errexit(self) -> T where E: fmt::Debug  {
        match self {
            Ok(t) => t,
            Err(_) => self.unwrap(), // `unwrap()` panics for `Err`
        }
    }
}

pub fn get_ok() -> Result<(), ()> {
    Ok(())
}

pub fn get_err() -> Result<(), ()> {
    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _a: Option<usize> = Some(123);
        let _b: Option<usize> = None;
        let _c = get_ok();
        let _d = get_err();
        println!("{:?}", _a.errexit());
        // println!("{:?}", _b.errexit());   // It panics!
        println!("{:?}", _c.errexit());
        // println!("{:?}", _d.errexit());   // It panics!
    }
}

/// # How to use
/// 
/// ## 1
/// ```ignore
/// stopwatch_start!();
/// sleeps(1);
/// stopwatch_stop!();
/// ```
/// ## 2
/// ```ignore
/// let mut stopwatch = Stopwatch::new();
/// stopwatch.start();
/// sleeps(1);
/// stopwatch.stop();
/// ```

pub struct Stopwatch {
    start_time: Instant,
    stop_time: Instant,
}

impl Stopwatch {

    pub fn new() -> Stopwatch {
        println!("Stopwatch Start...");
        Stopwatch {
            start_time: Instant::now(),
            stop_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn stop(&mut self) {
        self.stop_time = Instant::now();
        let elapsed_time = self.duration();
        println!("Stopwatch Stop... Elapsed time: {:?}", elapsed_time);
    }

    pub fn duration(&self) -> Duration {
        self.stop_time - self.start_time
    }
}

lazy_static::lazy_static! {
    static ref STOPWATCH: Mutex<Option<Stopwatch>> = Mutex::new(None);
}

#[macro_export]
macro_rules! stopwatch_start {
    () => {
        let stopwatch = Stopwatch::new();
        *STOPWATCH.lock().unwrap() = Some(stopwatch);
    };
}

#[macro_export]
macro_rules! stopwatch_stop {
    () => {
        let mut stopwatch_option = STOPWATCH.lock().unwrap();
        if let Some(ref mut stopwatch) = *stopwatch_option {
            stopwatch.stop();
        }
        *stopwatch_option = None;
    };
}

pub fn sleeps(second:u64) {

    let timeout = Duration::from_secs(second);

    thread::park_timeout(timeout);
//    or 
// use std::thread::sleep; sleep(timeout);   
// use async_std::task; task::sleep(Duration::from_secs(1)).await;
}


#[test]
fn test_stopwatch() {
    stopwatch_start!();
    sleeps(1);
    stopwatch_stop!();

    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    sleeps(1);
    stopwatch.stop();
}


