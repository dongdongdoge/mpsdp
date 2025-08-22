use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mpsdp::random;
use mpsdp::arith;
use mpsdp::schema;

fn bench_laplace_noise(c: &mut Criterion) {
    c.bench_function("laplace_noise", |b| {
        b.iter(|| random::laplace_noise(black_box(1.0)))
    });
}

fn bench_data_point_creation(c: &mut Criterion) {
    c.bench_function("data_point_creation", |b| {
        b.iter(|| schema::DataPoint::new(black_box(vec![1.0, 2.0, 3.0])))
    });
}

criterion_group!(
    benches,
    bench_laplace_noise,
    bench_data_point_creation
);
criterion_main!(benches); 