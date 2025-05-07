use criterion::{black_box, criterion_group, criterion_main, Criterion};
use doppio::random;
use doppio::arith;
use doppio::schema;

fn bench_laplace_noise(c: &mut Criterion) {
    c.bench_function("laplace_noise", |b| {
        b.iter(|| random::laplace_noise(black_box(1.0)))
    });
}

fn bench_gaussian_noise(c: &mut Criterion) {
    c.bench_function("gaussian_noise", |b| {
        b.iter(|| random::gaussian_noise(black_box(1.0)))
    });
}

fn bench_histogram_noise(c: &mut Criterion) {
    c.bench_function("histogram_noise", |b| {
        b.iter(|| random::hist_noise(black_box(1.0)))
    });
}

fn bench_privacy_budget_composition(c: &mut Criterion) {
    let budget1 = arith::PrivacyBudget::new(1.0, 1e-5);
    let budget2 = arith::PrivacyBudget::new(2.0, 1e-5);
    
    c.bench_function("privacy_budget_composition", |b| {
        b.iter(|| budget1.compose(black_box(&budget2)))
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
    bench_gaussian_noise,
    bench_histogram_noise,
    bench_privacy_budget_composition,
    bench_data_point_creation
);
criterion_main!(benches); 