//! This crate provides methods to de- and encode numbers in the [combinatorial
//! number system](https://en.wikipedia.org/wiki/Combinatorial_number_system),
//! sometime referred to as "combinadics".

#![cfg_attr(feature = "nightly", feature(test))]
#[cfg(feature = "nightly")] extern crate test as rust_test;

extern crate binomial_iter;

use binomial_iter::{BinomialIter, DecNIter};

/// Iterates through all choices decoded from the given `N` and `k` in
/// descending order.
#[allow(non_snake_case)]
pub struct DecodeIter {
    N: u32,
    // The `n` holds the state for the 'remaining' iteration, see `next`.
    n: u32,
    bin: DecNIter,
}

/// Returns an iterator over the choices decoded from the given `N` and `k`.
/// The choices will be returned by the iterator in descending order.
///
/// # Panics
/// If `k == 0`
#[allow(non_snake_case)]
pub fn decode(N: u32, k: u32) -> DecodeIter {
    assert!(k > 0, "invalid argument: `k == 0`");

    let mut bin = BinomialIter::new(k, k);

    // find the largest possible n
    while bin.binom() < N {
        bin.inc_n().expect("N too large to decode");
    }

    DecodeIter {
        N: N,
        n: bin.n(), // Initialize n to it's largest possible bin.n
        bin: bin.iter_dec_n(),
    }
}

impl Iterator for DecodeIter {
    type Item = u32;

    #[inline]
    fn next(&mut self) -> Option<u32> {
        // find the next choice
        while let Some((n, binom)) = self.bin.next() {
            if binom <= self.N {
                self.bin.dec_k();
                self.N -= binom;
                return Some(n);
            }
        }

        // `BinomialIter` does not support `n < k` so there may still be some
        // choices left, to be precise, all `x < bin.n` still need to be printed.
        // The first time this path is reached, initialize `self.n`.
        if self.n > self.bin.n() {
            self.n = self.bin.n();
        }

        if self.n > 0 {
            self.n -= 1;
            Some(self.n)
        } else {
            None
        }
    }
}

#[cfg(all(test, feature = "nightly"))]
mod bench;

#[cfg(test)]
mod test;
