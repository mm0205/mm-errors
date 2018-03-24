//! Provides `Oks` struct that implements `Iterator` trait.


use Result;

/// Oks.
#[derive(Debug)]
pub struct Oks<T> {
    source: T,
}

impl<T> Iterator for Oks<T>
    where T: Iterator {
    type Item = Result<T::Item>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(x) => Some(Ok(x)),
            None => None
        }
    }
}

impl<T> Clone for Oks<T>
    where T: Iterator + Clone {
    fn clone(&self) -> Self {
        Oks {
            source: self.source.clone(),
        }
    }
}

pub trait OksExtension {
    type Iterator;
    fn oks(self) -> Oks<Self::Iterator>;
}

impl<T> OksExtension for T
    where T: Iterator {
    type Iterator = T;

    fn oks(self) -> Oks<Self::Iterator> {
        Oks {
            source: self,
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oks() {
        let test_data = vec![0, 1, 2, 3];
        for (index, ok) in test_data.iter().oks().enumerate() {
            match ok {
                Ok(&x) => assert_eq!(index as i32, x),
                Err(e) => panic!("{:?}", e),
            }
        }
    }
}
