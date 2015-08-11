#![allow(non_snake_case)]

use super::decode;

// Given a choice of `k` values encode with `k` bits set, this computes the next
// combination. Returns `None` if the next combination cannot be represented.
fn next_combination(c: u64) -> Option<u64> {
    let u = c & (!c + 1);
    u.checked_add(c).map(|v| v + (((v ^ c) / u) >> 2))
}

#[test]
// Compare the results of two different strategies to compute combinations,
// the bit based version above and the one implemented by this library.
fn compare_with_bit_repr() {
    for i in 1 .. 64 {
        let mut c = (1 << i) - 1;

        for N in 0 .. 5000 {
            assert!(i as usize == decode(N, i).map(|v| assert!((c & (1 << v)) != 0)).count());

            match next_combination(c) {
                Some(cc) => c = cc,
                None => break,
            }
        }
    }
}

#[test]
#[should_panic(expected = "invalid argument:")]
fn panic_k_zero() {
    for N in 0 .. 20 {
        assert_eq!(decode(N, 0).count(), 0)
    }
}
