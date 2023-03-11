//! # Rust Basic
//!
//! `RustBasic` is a planned development that aims to make Rust easy to learn, teach, and use.
use std::fmt;

pub trait Errpass<T> {

    /// This function implements `unwrap()` as `errpass()`. 
    ///
    /// `errpass()` is used in `Option` and `Result` types, and returns the internal value if the corresponding value is `Some` or `Ok`. 
    /// If it is `None` or `Err`, it panics by default. 
    ///
    /// Therefore, `errpass()` should be used only when it is certain that a value exists, allowing you to safely extract that value and proceed to the next step.
    ///   
    /// # Examples
    /// 
    /// ```
    /// use rustbasic::*;
    /// 
    /// let _a: Option<usize> = Some(123);
    /// let _b: Option<usize> = None;
    /// let _c = get_ok();
    /// let _d = get_err();
    /// println!("{:?}", _a.errpass());
    /// // println!("{:?}", _b.errpass());   // It panics!
    /// println!("{:?}", _c.errpass());
    /// // println!("{:?}", _d.errpass());   // It panics!
    /// ```
    fn errpass(self) -> T;
}

impl<T> Errpass<T> for Option<T> {
    fn errpass(self) -> T {
        match self {
            Some(t) => t,
            None => self.unwrap(), // `unwrap()` panics for `None`
        }
    }
}

impl<T, E> Errpass<T> for Result<T, E> where E: fmt::Debug {
    fn errpass(self) -> T where E: fmt::Debug  {
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
        println!("{:?}", _a.errpass());
        // println!("{:?}", _b.errpass());   // It panics!
        println!("{:?}", _c.errpass());
        // println!("{:?}", _d.errpass());   // It panics!
    }
}
