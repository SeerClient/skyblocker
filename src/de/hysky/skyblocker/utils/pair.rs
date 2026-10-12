//! Ported from de.hysky.skyblocker.utils.Pair by fabric2seer.

#[allow(unused_imports)]
use crate::prelude::*;

/// java: public record Pair<L, R>(L left, R right)
#[derive(Clone, Debug)]
pub struct Pair<L, R> {
    /// java: L left
    pub left: L,
    /// java: R right
    pub right: R,
}

impl<L: Clone + 'static, R: Clone + 'static> Pair<L, R> {
    /// java: the canonical record constructor
    pub fn new(mut left: L, mut right: R) -> Self {
        Self { left, right }
    }

    /// java: the record accessor `left()`
    pub fn left(&self) -> L {
        self.left.clone()
    }

    /// java: the record accessor `right()`
    pub fn right(&self) -> R {
        self.right.clone()
    }

    /// java: public static <L, R> Pair<L, R> of(L left, R right)
    pub fn of<L: Clone + 'static, R: Clone + 'static>(left: &L, right: &R) -> (L, R) {
        return Pair::new(left.clone(), right.clone());
    }

}
