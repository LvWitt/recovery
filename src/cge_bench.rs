pub mod algorithms;
pub mod readers;
use crate::{
    algorithms::cgne,
    readers::{create_matrix_from_csv, create_vector_from_csv},
};
use criterion::{criterion_group, criterion_main, Criterion};

fn cgne_benchmark(c: &mut Criterion) {
    let vector = create_vector_from_csv("../Data/G-1.csv").unwrap();
    let matrix = create_matrix_from_csv("../Data/H-1.csv",50816,3600).unwrap();
    let mut group = c.benchmark_group("benches");

   /*  group.sample_size(10).bench_function("cge1", |f| {
        f.iter(|| cgne(matrix.clone(), vector.clone(), 0.0004))
    });*/
    group.sample_size(10).bench_function("cgne", |f| {
        f.iter(|| cgne(matrix.clone(), vector.clone(), 0.0004))
    });
}


criterion_group!(benches, cgne_benchmark);
criterion_main!(benches);
