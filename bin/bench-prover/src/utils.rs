use std::{
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};
use serde_json::to_writer_pretty;

#[derive(Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub function_name: String,
    pub time_secs: f64,
    pub sample_size: usize,
}

pub fn ensure_output_dir() -> std::io::Result<()> {
    let path = Path::new("benchmark_results");
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn save_results_to_json(results: Vec<BenchmarkResult>, filename: &str) -> std::io::Result<()> {
    ensure_output_dir()?;
    let path = Path::new("benchmark_results").join(filename);
    let file = File::create(path)?;
    to_writer_pretty(file, &results)?;
    Ok(())
}
