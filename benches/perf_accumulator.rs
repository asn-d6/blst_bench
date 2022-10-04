 #![allow(non_snake_case)]

use rand;
use rand::Rng;
use std::hash::{Hash, Hasher};
use hashbrown::HashMap;
use blstrs::{G1Projective, Scalar};
use criterion::*;
use group::ff::Field;
use group::Group;

/// A G1Projective that hashes
#[derive(Clone, Copy, Eq, PartialEq )]
pub struct MyProjective(G1Projective);

impl Hash for MyProjective {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.x().to_bytes_le().hash(state);
        self.0.y().to_bytes_le().hash(state);
        self.0.z().to_bytes_le().hash(state);
    }
}

/// An MSM accumulator object
#[derive(Clone)]
pub struct MsmAccumulator {
    base_scalar_map: HashMap<MyProjective, Scalar>,
}


impl MsmAccumulator {
    pub(crate) fn new() -> Self {
        Self {
            base_scalar_map: HashMap::new(),
        }
    }

    /// Accumulate the inner product <vec_b, vec_P>
    pub fn accumulate(
        &mut self,
        vec_b: &[Scalar],
        vec_P: &[MyProjective],
    ) {
        // let's ignore the random lincomb random factor for now
//        let random_factor = Scalar::random(rng);
        for (scalar, base) in vec_b.iter().zip(vec_P.iter()) {
            let entry_scalar = self.base_scalar_map.entry(*base).or_insert_with(Scalar::zero);
            *entry_scalar += scalar
        }
    }

    /// Compute all the MSMs accumulated
    pub fn compute(&self) -> G1Projective {
        let mut bases = vec![];
        let mut scalars = vec![];
        for (base, scalar) in &self.base_scalar_map {
            bases.push(base.0);
            scalars.push(*scalar);
        }

        return G1Projective::multi_exp(&bases, &scalars);
    }
}


fn benchmark_accumulator(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let size = 32_000;

    let vec_B: Vec<_> = (0..size).map(|_| G1Projective::random(&mut rng)).collect();
    let mut all_uint4_vecs = vec![];
    let mut all_uint1_vecs = vec![];
    let mut all_scalar_vecs = vec![];
    for _ in 0..16 { // 16 collectors
        let vec_uint4: Vec<_> = (0..size).map(|_| Scalar::from(rng.gen_range(0..=15))).collect();
        all_uint4_vecs.push(vec_uint4);
        let vec_uint1s: Vec<_> = (0..size).map(|_| Scalar::from(rng.gen_range(0..=1))).collect();
        all_uint1_vecs.push(vec_uint1s);
        let vec_scalar: Vec<_> = (0..size).map(|_| Scalar::random(&mut rng)).collect();
        all_scalar_vecs.push(vec_scalar);
    }


    c.bench_function("naive batch-verify 16 32k msms w/ full scalar", |b| {
        b.iter(|| {
           for vec_scalar in &all_scalar_vecs {
                G1Projective::multi_exp(&vec_B, &vec_scalar);
            }
        });
    });
    c.bench_function("naive batch-verify 16 32k msms w/ uint4", |b| {
        b.iter(|| {
           for vec_uint4 in &all_uint4_vecs {
                G1Projective::multi_exp(&vec_B, &vec_uint4);
            }
        });
    });
    c.bench_function("naive batch-verify 16 32k msms w/ bitfield", |b| {
        b.iter(|| {
           for vec_uint1 in &all_uint1_vecs {
                G1Projective::multi_exp(&vec_B, &vec_uint1);
            }
        });
    });

    let vec_B: Vec<_> = (0..size).map(|_| MyProjective(G1Projective::random(&mut rng))).collect();
    c.bench_function("accumulate 16 32k msms w/ full scalar", |b| {
        b.iter(|| {
            let mut accumulator = MsmAccumulator::new();
            for vec_scalar in &all_scalar_vecs {
                accumulator.accumulate(&vec_scalar, &vec_B);
            }

            accumulator.compute();
        });
    });

    c.bench_function("accumulate 16 32k msms w/ uint4", |b| {
        b.iter(|| {
            let mut accumulator = MsmAccumulator::new();
            for vec_uint4 in &all_uint4_vecs {
                accumulator.accumulate(&vec_uint4, &vec_B);
            }

            accumulator.compute();
        });
    });

    c.bench_function("accumulate 16 32k msms w/ bitfield", |b| {
        b.iter(|| {
            let mut accumulator = MsmAccumulator::new();
            for vec_uint1 in &all_uint1_vecs {
                accumulator.accumulate(&vec_uint1, &vec_B);
            }

            accumulator.compute();
        });
    });
}


criterion_group! {name = accumulator;
                  config = Criterion::default().sample_size(10);
                  targets = benchmark_accumulator
}

criterion_main!(accumulator);

