use std::{collections::HashMap, env, fmt::Write as FmtWrite, fs, path::Path};

use csv::ReaderBuilder;

pub fn generate_cwe_phf() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_dir = Path::new(&manifest_dir);

    let cwe_dir = manifest_dir.join("assets/cwe");
    let output_file = out_dir.join("cwe_generated.rs");

    println!("cargo::rerun-if-changed={}", cwe_dir.display());

    let cwe_files = fs::read_dir(&cwe_dir)
        .expect("Can't read CWE data directory")
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();

    let mut mapping: HashMap<String, CweRelease> = HashMap::new();
    for file in cwe_files {
        println!("cargo::rerun-if-changed={}", file.display());

        let filename = file.file_stem().unwrap().to_str().unwrap();

        let parts = filename.split("_").collect::<Vec<&str>>();
        let version = parts.get(1).unwrap().to_string();
        let release_date = parts
            .get(2)
            .and_then(|date_part| {
                if date_part.is_empty() {
                    return None;
                }
                let mut date_split = date_part.splitn(3, "-").map(|pos| pos.parse::<u64>().unwrap());
                Some((
                    date_split.next().unwrap() as i32,
                    date_split.next().unwrap() as u32,
                    date_split.next().unwrap() as u32,
                ))
            })
            .unwrap_or((1970_i32, 1_u32, 1_u32));
        let entries = parse_entries(&file);
        let release = CweRelease { release_date, entries };
        mapping.insert(version, release);
    }
    generate_cwe_code(mapping, &output_file);
}

struct CweRelease {
    release_date: (i32, u32, u32),
    entries: HashMap<String, CweEntry>,
}

struct CweEntry {
    status: String,
    name: String,
    usage: String,
}

fn parse_entries(path: &Path) -> HashMap<String, CweEntry> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b'\t')
        .has_headers(false)
        .from_path(path)
        .unwrap();

    let mut entries = HashMap::new();
    for result in reader.records() {
        let entry = result.expect("Invalid CSV record");

        let id = format!("CWE-{}", entry.get(0).expect("No CWE ID"));
        let status = to_cwe_status(entry.get(1).expect("No CWE status"));
        let name = entry.get(2).expect("No CWE name").trim().to_string();
        let usage = if entry.get(3).expect("No usage indicator") == "0" {
            String::from("CweUsage::NotProvided")
        } else {
            to_cwe_usage(entry.get(4).expect("No CWE usage"))
        };
        entries.insert(id, CweEntry { status, name, usage });
    }
    entries
}

fn to_cwe_status(status: &str) -> String {
    match status {
        "Draft" => String::from("CweStatus::Draft"),
        "Incomplete" => String::from("CweStatus::Incomplete"),
        "Stable" => String::from("CweStatus::Stable"),
        "Usable" => String::from("CweStatus::Usable"),
        "Deprecated" => String::from("CweStatus::Deprecated"),
        s => panic!("Unexpected CWE status: {s}"),
    }
}

fn to_cwe_usage(usage: &str) -> String {
    match usage {
        "Allowed" => String::from("CweUsage::Allowed"),
        "Allowed-with-Review" => String::from("CweUsage::AllowedWithReview"),
        "Discouraged" => String::from("CweUsage::Discouraged"),
        "Prohibited" => String::from("CweUsage::Prohibited"),
        u => panic!("Unknown usage: {u}"),
    }
}

fn generate_cwe_code(mapping: HashMap<String, CweRelease>, output_file: &Path) {
    let mut output = String::new();

    let mut outer = phf_codegen::Map::new();

    for (version, release) in mapping {
        let static_name = format!("CWE_ENTRIES_{}", version.replace('.', "_"));

        let mut inner = phf_codegen::Map::new();

        for (id, entry) in &release.entries {
            let value = format!(
                "CweEntry {{ \
                    status: {}, \
                    name: r##\"{}\"##,
                    usage: {}, \
                }}",
                entry.status, entry.name, entry.usage
            );
            inner.entry(id, value);
        }

        writeln!(
            output,
            "/// Maps CWE versions (e.g. \"4.20\") to its release date and its entries\nstatic {static_name}: phf::Map<&'static str, CweEntry> = {};",
            inner.build()
        )
        .unwrap();

        outer.entry(
            version,
            format!(
                "CweRelease {{ \
                release_date: ({}, {}, {}), \
                entries: &{static_name}, \
            }}",
                release.release_date.0, release.release_date.1, release.release_date.2,
            ),
        );
    }
    writeln!(
        output,
        "pub static CWE_RELEASES: phf::Map<&'static str, CweRelease> = {};",
        outer.build()
    )
    .unwrap();

    fs::write(output_file, output).expect("Can't write generated CWE sources.");
}
