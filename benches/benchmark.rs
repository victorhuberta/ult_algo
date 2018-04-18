#[macro_use]
extern crate criterion;
extern crate rand;
extern crate ult_algo;

use criterion::Criterion;
use rand::Rng;
use ult_algo::sequence;

fn sequence_benchmark(c: &mut Criterion) {
    let sequence_: Vec<i32> = (-100..100).collect();
    let pattern: Vec<i32> = (101..200).collect();
    c.bench_function("sequence::match_::bitap(&sequence, &pattern)", move |b| {
        b.iter(|| sequence::match_::bitap(&sequence_, &pattern))
    });

    let source: Vec<char> = "sitting in my kitchen like a boss knitting its scarf".chars().collect();
    let target: Vec<char> = "kittens love sitting on the knit of my scarf in my kitchen".chars().collect();
    c.bench_function("sequence::match_::levenshtein_distance(&source, &target)", move |b| {
        b.iter(|| sequence::match_::levenshtein_distance(&source, &target))
    });

    c.bench_function("sequence::selection::quick_smallest(&mut sequence, k)", |b| {
        let mut sequence_: Vec<i32> = (-100..100).collect();
        rand::thread_rng().shuffle(&mut sequence_);
        b.iter(|| *sequence::selection::quick_smallest(&mut sequence_, 100))
    });
}

criterion_group!(benches, sequence_benchmark);
criterion_main!(benches);
