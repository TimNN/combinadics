use rust_test::{Bencher, black_box};
use super::decode;

#[bench]
fn decode_k_2(b: &mut Bencher) {
    let mut i = 1_000_000;

    b.iter(|| {i += 1; decode(i, 2).map(black_box).count()});
}

#[bench]
fn decode_k_3(b: &mut Bencher) {
    let mut i = 1_000_000;

    b.iter(|| {i += 1; decode(i, 3).map(black_box).count()});
}

#[bench]
fn decode_k_4(b: &mut Bencher) {
    let mut i = 1_000_000;

    b.iter(|| {i += 1; decode(i, 4).map(black_box).count()});
}

#[bench]
fn decode_k_10(b: &mut Bencher) {
    let mut i = 1_000_000;

    b.iter(|| {i += 1; decode(i, 10).map(black_box).count()})
}
