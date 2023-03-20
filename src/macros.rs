#![allow(unused_doc_comments)]
#![allow(unused_imports)]
#![allow(dead_code)]

/*
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
*/
