//! Ported from io.github.moulberry.repo.util.StreamIt (neurepoparser 1.12.0).

/// java: public class StreamIt<T> implements Iterable<T> - lets a `Stream` be used in an enhanced `for`. A Rust
/// iterator already is one; the wrapper is kept so the class exists.
pub struct StreamIt<I: Iterator> {
    /// java: Stream<T> s;
    s: I,
}

impl<I: Iterator> StreamIt<I> {
    /// java: public StreamIt(Stream<T> t)
    pub fn new(t: I) -> Self {
        StreamIt { s: t }
    }
}

impl<I: Iterator> IntoIterator for StreamIt<I> {
    type Item = I::Item;
    type IntoIter = I;

    /// java: public Iterator<T> iterator() { return s.iterator(); }
    fn into_iter(self) -> I {
        self.s
    }
}
