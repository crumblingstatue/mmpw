use criterion::{criterion_group, criterion_main, Criterion};
use mmpw_validate::{binstring, validate};

fn bench_validate(c: &mut Criterion) {
    let key = binstring::hash_name(b"Dew");
    c.bench_function("validate", |b| {
        b.iter(|| validate(b"MYTHICDREAMYDEFECT", &key))
    });
}

criterion_group!(benches, bench_validate);
criterion_main!(benches);
