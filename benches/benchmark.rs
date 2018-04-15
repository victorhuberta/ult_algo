#[macro_use]
extern crate criterion;
extern crate ult_algo;

use criterion::Criterion;
use ult_algo::sequence;

fn benchmark(c: &mut Criterion) {
    let sequence_: Vec<i32> = (-100..100).collect();
    let pattern: Vec<i32> = (101..200).collect();
    c.bench_function("sequence::match_::bitap(&sequence, &pattern)", move |b| {
        b.iter(|| sequence::match_::bitap(&sequence_, &pattern))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
