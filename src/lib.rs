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

impl<T> IntoOption for T {}
