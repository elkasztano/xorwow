use criterion::{criterion_group, criterion_main, Criterion};
use xorwowgen::xorwow128::*;
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

create_bench!(run128w, LargeWrap, 1_000_000);
create_bench!(run128x, LargeXor, 1_000_000);

fn seed_and_run_128(c: &mut Criterion) {
    let mut group = c.benchmark_group("more-samples");
    group.sample_size(1000);
    group.bench_function("run128a", |b| b.iter(|| run128w()));
    group.bench_function("run128b", |b| b.iter(|| run128x()));
    group.finish();
}

criterion_group!(benches, seed_and_run_128);
criterion_main!(benches);
