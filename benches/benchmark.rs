#[macro_use] extern crate criterion;
extern crate rand;
#[macro_use] extern crate ult_algo;

use criterion::Criterion;
use rand::Rng;
use ult_algo::sequence;
include_sequence_search!();

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

    c.bench_function("sequence::search::ternary(search_target, |x| x.powf(x), 50.0, 1000.0, 0.0001)", |b| {
        b.iter(|| {
            sequence::search::ternary(sequence::search::SearchTarget::Maximum,
                |x| x.powf(x), 50.0, 1000.0, 0.0001)
        })
    });

    c.bench_function("sequence::search::ternary_min!(|x| x.powf(x), 50.0, 1000.0, 0.0001)", |b| {
        b.iter(|| ternary_min!(|x| x.powf(x), 50.0, 1000.0, 0.0001))
    });

    c.bench_function("sequence::search::ternary_max!(|x| x.powf(x), 50.0, 1000.0, 0.0001)", |b| {
        b.iter(|| ternary_max!(|x| x.powf(x), 50.0, 1000.0, 0.0001))
    });

    let sequence: Vec<i32> = (-100..100).collect();
    c.bench_function("sequence::search::binary(&sequence, &868)", move |b| {
        b.iter(|| sequence::search::binary(&sequence, &868))
    });

    let sequence: Vec<i32> = (-100..100).collect();
    c.bench_function("sequence::search::exponential(&sequence, &868)", move |b| {
        b.iter(|| sequence::search::exponential(&sequence, &868))
    });

    let sequence: Vec<i32> = (-100..100).collect();
    c.bench_function("sequence::search::interpolation(&sequence, &99)", move |b| {
        b.iter(|| sequence::search::interpolation(&sequence, &99))
    });
}

criterion_group!(benches, sequence_benchmark);
criterion_main!(benches);
