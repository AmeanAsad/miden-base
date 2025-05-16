use std::time::Duration;

use bench_prover::bench_functions::{
    prove_consume_multiple_notes, prove_consume_note_with_new_account,
};
use criterion::{Criterion, SamplingMode, black_box, criterion_group, criterion_main};

fn core_benchmarks(c: &mut Criterion) {
    // Create a single benchmark group for all functions
    let mut group = c.benchmark_group("miden_proving");

    // Configure the group
    group
        .sampling_mode(SamplingMode::Flat)
        .sample_size(10)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(2));

    group.bench_function("prove_consume_note_with_new_account", |b| {
        b.iter(|| black_box(prove_consume_note_with_new_account()))
    });

    group.bench_function("prove_consume_multiple_notes", |b| {
        b.iter(|| black_box(prove_consume_multiple_notes()))
    });

    // Finish the group - this ensures proper closing of files
    group.finish();
}

// Use a single criterion group
criterion_group!(benches, core_benchmarks);
criterion_main!(benches);
