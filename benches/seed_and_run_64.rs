use criterion::{criterion_group, criterion_main, Criterion};
use xorwowgen::xorwow64::*;
use rand_core::SeedableRng;

macro_rules! create_bench {
    ($name:ident, $subj:ident, $n:expr) => {
        fn $name() {
            let mut rng = $subj::seed_from_u64(9876512349876);
            for _ in 0..$n {
                rng.return_u32();
            }
        }
    }
}

create_bench!(run64wa, WrapA, 5_000_000_000usize);
create_bench!(run64wb, WrapB, 5_000_000_000usize);
create_bench!(run64xa, XorA, 5_000_000_000usize);
create_bench!(run64xb, XorB, 5_000_000_000usize);

fn seed_and_run_64(c: &mut Criterion) {
    let mut group = c.benchmark_group("more-samples");
    group.sample_size(1000);
    group.bench_function("run64wa", |b| b.iter(|| run64wa()));
    group.bench_function("run64wb", |b| b.iter(|| run64wb()));
    group.bench_function("run64xa", |b| b.iter(|| run64xa()));
    group.bench_function("run64xb", |b| b.iter(|| run64xb()));
    group.finish();
}

criterion_group!(benches, seed_and_run_64);
criterion_main!(benches);
