use std::{fs, path::PathBuf, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use csaf_validator::validate;

/// Benchmarks RHSA-2026-99964
fn red_hat_benchmark(c: &mut Criterion) {
    let input = PathBuf::from("benches/data/rhsa-2026_99964.json");
    let test_suite = String::from("full");

    let mut group = c.benchmark_group("Red Hat");
    group.measurement_time(Duration::from_secs(6));

    group.bench_function("Red Hat 2026-99964", |b| {
        b.iter(|| validate(&input, "2.0", std::slice::from_ref(&test_suite)))
    });

    group.finish();
}

/// Benchmarks OASIS CSAF provided file BSI-2022-0001
fn csaf_oasis_benchmark(c: &mut Criterion) {
    let inputs = fs::read_dir("../oasis-csaf/csaf_2.1/examples/csaf/")
        .expect("Directory oasis-csaf is missing")
        .filter_map(|path| {
            path.ok()
                .and_then(|path| if path.path().is_file() { Some(path.path()) } else { None })
        });
    let test_suite = String::from("full");
    let mut group = c.benchmark_group("OASIS Examples");

    for input in inputs {
        group.bench_function(input.file_name().unwrap().to_str().unwrap(), |b| {
            b.iter(|| validate(&input, "2.1", std::slice::from_ref(&test_suite)))
        });
    }

    group.finish();
}

/// Benchmarks a synthetic file that fails as many mandatory tests as possible. In addition, some
/// recommended and informative tests are failing. Failing mandatory test:
/// 6.1.1, 6.1.2, 6.1.3, 6.1.4, 6.1.5, 6.1.6, 6.1.7, 6.1.9, 6.1.10, 6.1.11, 6.1.12, 6.1.13
/// 6.1.14, 6.1.16, 6.1.17, 6.1.19, 6.1.21, 6.1.22, 6.1.23, 6.1.24, 6.1.25, 6.1.27.5, 6.1.27.6
/// 6.1.27.8, 6.1.27.12, 6.1.28, 6.1.30, 6.1.31, 6.1.32, 6.1.33, 6.1.34, 6.1.35, 6.1.36, 6.1.38
/// 6.1.41, 6.1.42, 6.1.43, 6.1.44, 6.1.47, 6.1.49, 6.1.50, 6.1.51, 6.1.52, 6.1.53, 6.1.54
/// 6.1.56, 6.1.57, 6.1.58, 6.1.59, 6.1.61, 6.1.62, 6.1.63, 6.1.64.1, 6.1.64.2, 6.1.64.3, 6.1.64.4
/// 6.1.64.5, 6.1.64.6, 6.1.64.7, 6.1.64.8, 6.1.64.9, 6.1.64.10, 6.1.64.11, 6.1.64.12, 6.1.64.13
/// 6.1.65.1, 6.1.65.2, 6.1.66
fn bad_file_benchmark(c: &mut Criterion) {
    let input = PathBuf::from("benches/data/csaf-rs_csaf-csaf_2_1-bad.json");
    let test_suite = String::from("full");

    let mut group = c.benchmark_group("Bad Files");

    group.measurement_time(Duration::from_secs(6));

    group.bench_function("Bad File", |b| {
        b.iter(|| validate(&input, "2.1", std::slice::from_ref(&test_suite)))
    });

    group.finish();
}

criterion_group!(
    validator_benches,
    red_hat_benchmark,
    csaf_oasis_benchmark,
    bad_file_benchmark
);

criterion_main!(validator_benches);
