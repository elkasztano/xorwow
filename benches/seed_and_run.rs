use criterion::{criterion_group, criterion_main, Criterion};
use xorwow::*;
use rand_core::{SeedableRng, RngCore};

macro_rules! create_bench {
    ($name:ident, $subj:ident, $n:expr) => {
        fn $name() {
            let mut rng = $subj::seed_from_u64(987654321);
            for _ in 0..$n {
                rng.next_u64();
            }
        }
    }
}

create_bench!(run128, Xorwow128, 10_000_000);
create_bench!(run160, Xorwow160, 10_000_000);
create_bench!(run192, Xorwow192, 10_000_000);
create_bench!(run160xor, XorwowXor160, 10_000_000);

fn seed_and_run(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic-config");
    group.sample_size(10);
    group.bench_function("run128", |b| b.iter(|| run128()));
    group.bench_function("run160", |b| b.iter(|| run160()));
    group.bench_function("run192", |b| b.iter(|| run192()));
    group.bench_function("run160xor", |b| b.iter(|| run160xor()));
    group.finish();
}

criterion_group!(benches, seed_and_run);
criterion_main!(benches);
