#![allow(non_snake_case)]

use blstrs::{G1Projective, G2Projective, Scalar};
use criterion::*;
use group::ff::Field;
use group::Group;

fn benchmark_bitfield(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let size = 32_000;
    let vec_B: Vec<_> = (0..size).map(|_| G1Projective::random(&mut rng)).collect();

    let vec_1: Vec<_> = (0..size).map(|_| Scalar::from(1)).collect();
    c.bench_function("bitfield", |b| {
        b.iter(|| G1Projective::multi_exp(&vec_B, &vec_1));
    });

    let vec_uint4: Vec<_> = (0..size).map(|_| Scalar::from(4)).collect();
    c.bench_function("uint4", |b| {
        b.iter(|| G1Projective::multi_exp(&vec_B, &vec_uint4));
    });

    let vec_a: Vec<_> = (0..size).map(|_| Scalar::random(&mut rng)).collect();
    c.bench_function("scalars", |b| {
        b.iter(|| G1Projective::multi_exp(&vec_B, &vec_a));
    });

}


criterion_group! {name = bitfield;
                  config = Criterion::default();
                  targets = benchmark_bitfield
}

criterion_main!(bitfield);
