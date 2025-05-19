# Miden Prover Benchmarking

This document describes how to run and analyze benchmarks for the Miden prover.

## Running Benchmarks

You can run the benchmarks in two ways:

### Option 1: Using Make (from miden-base directory)

```bash
make bench-prover
```

### Option 2: Running Directly (from bench-prover directory)

```bash
# Run the benchmarks
cargo bench

# Process the results
cargo run
```

## How It Works

1. `cargo bench` uses [Criterion.rs](https://github.com/bheisler/criterion.rs) to run performance benchmarks
2. By default, Criterion stores raw benchmark results in `target/criterion/`
3. `cargo run` parses these results and creates a consolidated summary in `consolidated_benchmarks.json`

## Viewing Results

### HTML Reports

Criterion automatically generates HTML reports with its built-in reporting feature. After running the benchmarks, you can find these reports in the Criterion directory by default under `target/criterion/{BENCHMARK_GROUP}/index.html`


### Consolidated JSON Summary

The `consolidated_benchmarks.json` file contains a summary of all proving benchmarks in a structured format:

Example `consolidated_benchmarks.json` structure:
```json
{
  "timestamp": "2023-10-15T14:30:22Z",
  "benchmarks": [
    {
      "name": "fibonacci_10",
      "proving_time_ms": 124.5,
      "memory_usage_mb": 45.2,
      "peak_memory_mb": 67.8
    },
    {
      "name": "merkle_path_verification",
      "proving_time_ms": 378.9,
      "memory_usage_mb": 103.5,
      "peak_memory_mb": 128.2
    }
    // ... more benchmarks
  ]
}
```
