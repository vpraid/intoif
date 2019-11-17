#![no_std]

pub trait IntoOption where Self: Sized {
    fn some_if<P>(self, predicate: P) -> Option<Self>
    where P: Fn(&Self) -> bool {
        if predicate(&self) {
            Some(self)
        } else {
            None
        }
    }
    fn none_if<P>(self, predicate: P) -> Option<Self>
    where P: Fn(&Self) -> bool {
        if predicate(&self) {
            None
        } else {
            Some(self)
        }
    }
}

pub trait IntoResult where Self: Sized {
    fn ok_if<P, E>(self, predicate: P, error: E) -> Result<Self, E>
    where P: Fn(&Self) -> bool {
        if predicate(&self) {
            Ok(self)
        } else {
            Err(error)
        }
    }
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
