//! Build automation for CSAF validator web demo
//!
//! Usage:
//!   cargo xtask build-web    - Build WASM and frontend
//!   cargo xtask build-wasm   - Build only WASM
//!   cargo xtask build-frontend - Build only frontend
//!   cargo xtask install      - Install frontend dependencies
//!   cargo xtask clean        - Clean build artifacts

use anyhow::{bail, Context, Result};
use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn main() -> Result<()> {
    let task = env::args().nth(1);

    let root = project_root();
    env::set_current_dir(&root)?;

    match task.as_deref() {
        Some("build-web") => build_web()?,
        Some("build-wasm") => build_wasm()?,
        Some("build-frontend") => build_frontend()?,
        Some("gen-ts") => gen_ts()?,
        Some("install") => install_deps()?,
        Some("clean") => clean()?,
        Some("dev") => dev_server()?,
        _ => print_help(),
    }

    Ok(())
}

fn print_help() {
    eprintln!(
        "Usage: cargo xtask <TASK>

TASKS:
    build-web       Build WASM module and frontend (complete build)
    build-wasm      Build only the WASM module
    build-frontend  Build only the frontend
    gen-ts          Generate TypeScript defs from Rust types (ts-rs)
    install         Install frontend dependencies
    clean           Clean build artifacts
    dev             Start frontend dev server (requires build-wasm first)
"
    );
}

fn build_web() -> Result<()> {
    println!("🏗️  Building CSAF Validator Web Demo\n");

    // Check prerequisites
    check_wasm_pack()?;
    check_deno()?;

    // Build WASM
    build_wasm()?;

    // Build frontend
    build_frontend()?;

    println!("\n✅ Build complete!");
    println!("📁 Static files are in: csaf-validator/src/web/static/");
    println!("\n🚀 Run the web server with:");
    println!("   cargo run --bin csaf-validator --release -- --web");

    Ok(())
}

fn build_wasm() -> Result<()> {
    println!("🦀 Building WASM module...");

    check_wasm_pack()?;

    let status = Command::new("wasm-pack")
        .args(["build", "--target", "web", "--scope", "csaf-rs", "csaf-rs"])
        .status()
        .context("Failed to run wasm-pack")?;

    if !status.success() {
        bail!("wasm-pack build failed");
    }

    println!("✓ WASM module built successfully");
    Ok(())
}

fn build_frontend() -> Result<()> {
    println!("🎨 Building frontend...");

    check_deno()?;

    let web_ui_dir = project_root().join("csaf-validator/web-ui");

    let status = Command::new("deno")
        .args(["task", "build"])
        .current_dir(&web_ui_dir)
        .status()
        .context("Failed to run deno task build")?;

    if !status.success() {
        bail!("Frontend build failed");
    }

    println!("✓ Frontend built successfully");
    Ok(())
}

fn gen_ts() -> Result<()> {
    println!("🧾 Generating TypeScript definitions from Rust types (ts-rs)...");

    // Run the helper binary included in the csaf-rs crate that emits the .d.ts file
    let status = Command::new("cargo")
        .args(["run", "-p", "csaf-rs", "--bin", "tsgen"])
        .status()
        .context("Failed to run tsgen binary")?;

    if !status.success() {
        bail!("tsgen failed");
    }

    println!("✓ TypeScript definitions generated");
    Ok(())
}

fn install_deps() -> Result<()> {
    println!("📦 Installing frontend dependencies...");

    check_deno()?;

    let web_ui_dir = project_root().join("csaf-validator/web-ui");

    // Deno doesn't require explicit install, but we can cache dependencies
    let status = Command::new("deno")
        .args(["cache", "--reload", "npm:vite", "npm:react", "npm:react-dom"])
        .current_dir(&web_ui_dir)
        .status()
        .context("Failed to cache deno dependencies")?;

    if !status.success() {
        bail!("Deno cache failed");
    }

    println!("✓ Dependencies cached");
    Ok(())
}

fn dev_server() -> Result<()> {
    println!("🚀 Starting development server...");
    println!("💡 Make sure you've run 'cargo xtask build-wasm' first!\n");

    check_deno()?;

    let web_ui_dir = project_root().join("csaf-validator/web-ui");

    let status = Command::new("deno")
        .args(["task", "dev"])
        .current_dir(&web_ui_dir)
        .status()
        .context("Failed to start dev server")?;

    if !status.success() {
        bail!("Dev server failed");
    }

    Ok(())
}

fn clean() -> Result<()> {
    println!("🧹 Cleaning build artifacts...");

    let paths = [
        "csaf-validator/web-ui/node_modules",
        "csaf-validator/web-ui/public/assets",
        "csaf-validator/src/web/static",
        "target",
        ".deno",
    ];

    for path in &paths {
        let full_path = project_root().join(path);
        if full_path.exists() {
            println!("  Removing {}", path);
            if full_path.is_dir() {
                std::fs::remove_dir_all(&full_path).with_context(|| format!("Failed to remove {}", path))?;
            } else {
                std::fs::remove_file(&full_path).with_context(|| format!("Failed to remove {}", path))?;
            }
        }
    }

    println!("✓ Clean complete");
    Ok(())
}

// Helper functions

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

fn check_wasm_pack() -> Result<()> {
    if Command::new("wasm-pack")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        bail!("wasm-pack not found. Install it with:\n  cargo install wasm-pack");
    }
    Ok(())
}

fn check_deno() -> Result<()> {
    if Command::new("deno")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_err()
    {
        bail!(
            "Deno not found. Install it from https://deno.com/ or run:\n  curl -fsSL https://deno.land/install.sh | sh"
        );
    }
    Ok(())
}
