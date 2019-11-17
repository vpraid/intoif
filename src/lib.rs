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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn none_on_zero() {
        assert_eq!(None, 0.none_if(|x| *x == 0))
    }

    #[test]
    fn some_on_zero() {
        assert_eq!(Some(0), 0.some_if(|x| *x == 0))
    }

}
