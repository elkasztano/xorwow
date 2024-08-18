use criterion::{criterion_group, criterion_main, Criterion};
use xorwowgen::*;
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

create_bench!(run96, Xorwow96, 0x20000);
create_bench!(run128, Xorwow128, 0x20000);
create_bench!(run160, Xorwow160, 0x20000);
create_bench!(run96xor, XorwowXor96, 0x20000);
create_bench!(run128xor, XorwowXor128, 0x20000);
create_bench!(run160xor, XorwowXor160, 0x20000);

fn seed_and_run(c: &mut Criterion) {
    let mut group = c.benchmark_group("basic-config");
    group.sample_size(100);
    group.bench_function("run96", |b| b.iter(|| run96()));
    group.bench_function("run128", |b| b.iter(|| run128()));
    group.bench_function("run160", |b| b.iter(|| run160()));
    group.bench_function("run96xor", |b| b.iter(|| run96xor()));
    group.bench_function("run128xor", |b| b.iter(|| run128xor()));
    group.bench_function("run160xor", |b| b.iter(|| run160xor()));
    group.finish();
}

criterion_group!(benches, seed_and_run);
criterion_main!(benches);
