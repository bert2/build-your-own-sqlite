use std::{
    iter::{Chain, Flatten},
    option::IntoIter,
};

pub struct IterEither<L, R, T>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    pub left: Option<L>,
    pub right: Option<R>,
    inner: Option<Chain<Flatten<IntoIter<L>>, Flatten<IntoIter<R>>>>,
}

impl<L, R, T> IterEither<L, R, T>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    pub fn left(iter: L) -> IterEither<L, R, T> {
        Self {
            left: Some(iter),
            right: None,
            inner: None,
        }
    }

    pub fn right(iter: R) -> IterEither<L, R, T> {
        Self {
            left: None,
            right: Some(iter),
            inner: None,
        }
    }
}

impl<L, R, T> Iterator for IterEither<L, R, T>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if matches!(self.inner, None) {
            self.inner = Some(
                self.left
                    .take()
                    .into_iter()
                    .flatten()
                    .chain(self.right.take().into_iter().flatten()),
            );
        }

        self.inner.as_mut().unwrap().next()
    }
}