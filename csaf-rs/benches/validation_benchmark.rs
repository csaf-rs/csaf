use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use criterion::{Criterion, criterion_group, criterion_main};

use csaf::csaf2_0::loader::load_document_from_str as load_document_2_0;
use csaf::csaf2_0::testcases::{
    informative_tests as informative_tests_2_0, mandatory_tests as mandatory_tests_2_0,
    recommended_tests as recommended_tests_2_0,
};
use csaf::csaf2_1::loader::load_document_from_str as load_document_2_1;
use csaf::csaf2_1::testcases::{
    informative_tests as informative_tests_2_1, mandatory_tests as mandatory_tests_2_1,
    recommended_tests as recommended_tests_2_1,
};
use csaf::validation::{validate_by_preset, validate_by_test};

/// Collect all JSON test fixture files from a directory recursively.
fn collect_fixture_files(dir: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let path = PathBuf::from(dir);
    if !path.exists() {
        return files;
    }
    for entry in walkdir(&path) {
        if entry.extension().is_some_and(|ext| ext == "json") {
            let filename = entry.file_name().unwrap_or_default().to_string_lossy();
            if filename.starts_with("testcases") || filename.contains("TEMPLATE") {
                continue;
            }
            files.push(entry);
        }
    }
    files.sort();
    files
}

/// Simple recursive directory walker.
fn walkdir(path: &PathBuf) -> Vec<PathBuf> {
    let mut results = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                results.extend(walkdir(&p));
            } else {
                results.push(p);
            }
        }
    }
    results
}

/// Pre-load all fixture file contents to exclude I/O from benchmark timing.
fn load_fixture_contents(files: &[PathBuf]) -> Vec<(String, String)> {
    files
        .iter()
        .filter_map(|f| {
            let content = fs::read_to_string(f).ok()?;
            let name = f.file_name()?.to_string_lossy().to_string();
            Some((name, content))
        })
        .collect()
}

/// Benchmark each individual CSAF 2.0 test function across all fixtures.
fn bench_individual_tests_csaf_2_0(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../csaf/csaf_2.0/test/validator/data");
    let files = collect_fixture_files(fixtures_dir);
    let contents = load_fixture_contents(&files);

    if contents.is_empty() {
        eprintln!("Warning: No CSAF 2.0 test fixtures found at {fixtures_dir}");
        return;
    }

    // Pre-parse all documents once (parsing is not what we're benchmarking here)
    let documents: Vec<_> = contents
        .iter()
        .filter_map(|(_name, content)| load_document_2_0(content).ok())
        .collect();

    let all_test_ids: Vec<&str> = [mandatory_tests_2_0(), recommended_tests_2_0(), informative_tests_2_0()].concat();

    let mut group = c.benchmark_group("csaf_2_0_tests");

    for test_id in &all_test_ids {
        group.bench_function(*test_id, |b| {
            b.iter(|| {
                for doc in &documents {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        validate_by_test(doc, test_id);
                    }));
                }
            });
        });
    }

    group.finish();
}

/// Benchmark each individual CSAF 2.1 test function across all fixtures.
fn bench_individual_tests_csaf_2_1(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../csaf/csaf_2.1/test/validator/data");
    let files = collect_fixture_files(fixtures_dir);
    let contents = load_fixture_contents(&files);

    if contents.is_empty() {
        eprintln!("Warning: No CSAF 2.1 test fixtures found at {fixtures_dir}");
        return;
    }

    let documents: Vec<_> = contents
        .iter()
        .filter_map(|(_name, content)| load_document_2_1(content).ok())
        .collect();

    // CSAF 2.1 uses the same test ID scheme
    let all_test_ids: Vec<&str> = [mandatory_tests_2_1(), recommended_tests_2_1(), informative_tests_2_1()].concat();

    let mut group = c.benchmark_group("csaf_2_1_tests");

    for test_id in &all_test_ids {
        group.bench_function(*test_id, |b| {
            b.iter(|| {
                for doc in &documents {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        validate_by_test(doc, test_id);
                    }));
                }
            });
        });
    }

    group.finish();
}

/// Benchmark full preset validation (all tests combined) for overall throughput.
fn bench_full_validation(c: &mut Criterion) {
    let fixtures_dir_2_0 = concat!(env!("CARGO_MANIFEST_DIR"), "/../csaf/csaf_2.0/test/validator/data");
    let fixtures_dir_2_1 = concat!(env!("CARGO_MANIFEST_DIR"), "/../csaf/csaf_2.1/test/validator/data");

    let contents_2_0 = load_fixture_contents(&collect_fixture_files(fixtures_dir_2_0));
    let contents_2_1 = load_fixture_contents(&collect_fixture_files(fixtures_dir_2_1));

    let mut group = c.benchmark_group("full_validation");

    if !contents_2_0.is_empty() {
        let documents_2_0: Vec<_> = contents_2_0
            .iter()
            .filter_map(|(_name, content)| load_document_2_0(content).ok())
            .collect();

        group.bench_function("csaf_2_0_full_preset", |b| {
            b.iter(|| {
                for doc in &documents_2_0 {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        validate_by_preset(doc, "2.0", "full");
                    }));
                }
            });
        });
    }

    if !contents_2_1.is_empty() {
        let documents_2_1: Vec<_> = contents_2_1
            .iter()
            .filter_map(|(_name, content)| load_document_2_1(content).ok())
            .collect();

        group.bench_function("csaf_2_1_full_preset", |b| {
            b.iter(|| {
                for doc in &documents_2_1 {
                    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        validate_by_preset(doc, "2.1", "full");
                    }));
                }
            });
        });
    }

    group.finish();
}

/// Benchmark parsing only (no validation).
fn bench_parse_only(c: &mut Criterion) {
    let fixtures_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../csaf/csaf_2.0/test/validator/data");
    let contents = load_fixture_contents(&collect_fixture_files(fixtures_dir));

    if contents.is_empty() {
        return;
    }

    let mut group = c.benchmark_group("parse_only");

    group.bench_function("csaf_2_0", |b| {
        b.iter(|| {
            for (_name, content) in &contents {
                let _ = load_document_2_0(content);
            }
        });
    });

    group.finish();
}

fn configured_criterion() -> Criterion {
    Criterion::default()
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(5))
        .sample_size(50)
}

criterion_group! {
   name = benches;
   config = configured_criterion();
   targets = bench_individual_tests_csaf_2_0, bench_individual_tests_csaf_2_1, bench_full_validation, bench_parse_only
}
criterion_main!(benches);
