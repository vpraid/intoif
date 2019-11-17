//! This library provides two convenient traits that allow you to convert values into option or result
//! based on the provided predicate. It is somewhat similar to the boolinator crate, except you don't
//! need to create a boolean - the predicate will do it for you. This can be useful e.g. when writing a
//! long chain that endsd with a fold which you want to convert into Option or Result.
//!
//! ```
//! use intoif::IntoOption;
//! fn fizz_buzz(n: u32) -> String {
//!     [(3, "Fizz"), (5, "Buzz")]
//!         .iter()
//!         .filter_map(|(x, s)| if n % x == 0 { Some(*s) } else { None })
//!         .collect::<String>()
//!         .none_if(String::is_empty)
//!         .unwrap_or_else(|| n.to_string())
//! }
//! ```

#![no_std]

/// Allow construction of Option from any type using predicate to choose between Some and None.
pub trait IntoOption where Self: Sized {
    /// Returns Some(self) if predicate returns true on self, None otherwise.
    fn some_if<P>(self, predicate: P) -> Option<Self>
    where P: Fn(&Self) -> bool {
        if predicate(&self) {
            Some(self)
        } else {
            None
        }
    }
    /// Returns None if predicate returns true on self, Some(self) otherwise.
    fn none_if<P>(self, predicate: P) -> Option<Self>
    where P: Fn(&Self) -> bool {
        if predicate(&self) {
            None
        } else {
            Some(self)
        }
    }
}

/// Allow construction of Result from any type using predicate to choose between Ok and Err.
pub trait IntoResult where Self: Sized {
    /// Returns Ok(self) if predicate returns true on self, Err(error) otherwise.
    fn ok_if<P, E>(self, predicate: P, error: E) -> Result<Self, E>
    where P: Fn(&Self) -> bool {
        if predicate(&self) {
            Ok(self)
        } else {
            Err(error)
        }
    }
    /// Returns Err(error) if predicate returns true on self, Ok(self) otherwise.
    fn err_if<P, E>(self, predicate: P, error: E) -> Result<Self, E>
    where P: Fn(&Self) -> bool {
        if predicate(&self) {
            Err(error)
        } else {
            Ok(self)
        }
    }
}

impl<T> IntoOption for T {}
impl<T> IntoResult for T {}
