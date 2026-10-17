//! # Ferrox Codex CLI Tool (`ferrox-codex`)
//!
//! Native command-line build and export tool for the Ferrox documentation engine.
//! Completely replaces Node.js / Docusaurus build pipeline with native Rust and Trunk.
//!
//! ## Usage:
//! ```bash
//! cargo run -p ferrox-codex -- build [--out-dir <dir>]
//! cargo run -p ferrox-codex -- export-gh-pages
//! cargo run -p ferrox-codex -- serve
//! ```

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("build");

    match command {
        "build" => {
            let out_dir = get_arg_value(&args, "--out-dir")
                .unwrap_or_else(|| "dist".to_string());
            execute_build(&out_dir);
        }
        "ssg" => {
            let out_dir = get_arg_value(&args, "--out-dir")
                .unwrap_or_else(|| "dist".to_string());
            execute_ssg(&out_dir);
        }
        "sync" => {
            let sources = get_arg_value(&args, "--sources")
                .unwrap_or_else(|| ".".to_string());
            let out_file = get_arg_value(&args, "--out")
                .unwrap_or_else(|| "docs_bundle.json".to_string());
            execute_sync(&sources, &out_file);
        }
        "export-gh-pages" => {
            execute_export_gh_pages();
        }
        "serve" => {
            execute_serve();
        }
        "help" | "--help" | "-h" => {
            print_help();
        }
        unknown => {
            eprintln!("Unknown command: '{}'", unknown);
            print_help();
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!(r#"
⚡ Ferrox Codex CLI • Native WebAssembly Documentation Generator
Usage:
  ferrox-codex build [--out-dir <dir>]     Builds release Wasm distribution & AI ssg
  ferrox-codex ssg [--out-dir <dir>]       Generates llms.txt, llms-full.txt & static SEO pages
  ferrox-codex sync [--sources <dir>]      Scans & ingests all ecosystem docs into bundle
  ferrox-codex export-gh-pages            Builds and exports directly to ferrox-docs for GitHub Pages
  ferrox-codex serve                      Starts local development server with hot-reloading
  ferrox-codex help                       Prints this help message
"#);
}

fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

fn find_crate_dir() -> PathBuf {
    // 1. If CARGO_MANIFEST_DIR is set and has Trunk.toml
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let p = PathBuf::from(manifest_dir);
        if p.join("Trunk.toml").exists() {
            return p;
        }
    }

    // 2. Check current_dir and common relative paths
    if let Ok(curr) = env::current_dir() {
        if curr.join("Trunk.toml").exists() {
            return curr;
        }
        if curr.join("crates/ferrox-codex/Trunk.toml").exists() {
            return curr.join("crates/ferrox-codex");
        }
        if curr.join("ferrox-front/crates/ferrox-codex/Trunk.toml").exists() {
            return curr.join("ferrox-front/crates/ferrox-codex");
        }
        if curr.join("../ferrox-front/crates/ferrox-codex/Trunk.toml").exists() {
            return curr.join("../ferrox-front/crates/ferrox-codex");
        }
        if curr.join("../../ferrox-front/crates/ferrox-codex/Trunk.toml").exists() {
            return curr.join("../../ferrox-front/crates/ferrox-codex");
        }
    }

    // Fallback to current directory
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn find_ferrox_docs_dir(crate_dir: &Path) -> PathBuf {
    // Check relative to crate_dir
    let rel = crate_dir.join("../../../ferrox-docs");
    if rel.exists() {
        return rel.canonicalize().unwrap_or(rel);
    }

    // Check relative to current working directory
    if let Ok(curr) = env::current_dir() {
        if curr.join("ferrox-docs").exists() {
            return curr.join("ferrox-docs");
        }
        if curr.file_name().and_then(|s| s.to_str()) == Some("ferrox-docs") {
            return curr;
        }
        if curr.join("../ferrox-docs").exists() {
            return curr.join("../ferrox-docs");
        }
    }

    rel
}

fn execute_build(out_dir: &str) {
    println!("================================================================");
    println!("       FERROX-CODEX: COMPILING NATIVE WASM DOCUMENTATION        ");
    println!("================================================================");

    let crate_dir = find_crate_dir();
    let crate_dir = crate_dir.canonicalize().unwrap_or(crate_dir);
    println!("Crate Directory : {}", crate_dir.display());
    println!("Output Target   : {}", out_dir);

    // Run Trunk release build
    println!("\n[1/2] Invoking Trunk WebAssembly compiler (trunk build --release)...");
    let status = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir(&crate_dir)
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("  -> Wasm binary compilation & optimization succeeded!");
        }
        Ok(s) => {
            eprintln!("[ERROR] Trunk build failed with exit code: {:?}", s.code());
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to execute trunk command: {}", e);
            eprintln!("        Please ensure trunk is installed via: cargo install trunk");
            std::process::exit(1);
        }
    }

    // Copy to custom out_dir if different from dist
    let dist_dir = crate_dir.join("dist");
    let target_out = if Path::new(out_dir).is_absolute() {
        PathBuf::from(out_dir)
    } else {
        crate_dir.join(out_dir)
    };

    if dist_dir != target_out && target_out != crate_dir.join("dist") {
        println!("\n[2/2] Copying release artifacts into: {}", target_out.display());
        if let Err(e) = copy_dir_all(&dist_dir, &target_out) {
            eprintln!("[ERROR] Failed to copy artifacts: {}", e);
            std::process::exit(1);
        }
        println!("  -> Distribution copied successfully.");
    } else {
        println!("\n[2/2] Release artifacts ready in {}", dist_dir.display());
    }

    // Generate AI-Native artifacts (llms.txt, llms-full.txt) and SSG pre-renders
    execute_ssg(&target_out.to_string_lossy());

    println!("\n================================================================");
    println!(">>> BUILD COMPLETE: Zero JS • Pure Rust WebAssembly Codex <<<");
    println!("================================================================");
}

fn execute_export_gh_pages() {
    println!("================================================================");
    println!("    FERROX-CODEX: EXPORTING NATIVE WASM TO GITHUB PAGES ROOT    ");
    println!("================================================================");

    let crate_dir = find_crate_dir();
    let crate_dir = crate_dir.canonicalize().unwrap_or(crate_dir);

    // 1. Build release
    execute_build("dist");

    // 2. Identify ferrox-docs directory
    let ferrox_docs_root = find_ferrox_docs_dir(&crate_dir);
    let dist_dir = crate_dir.join("dist");

    if !ferrox_docs_root.exists() {
        eprintln!("[ERROR] ferrox-docs root directory not found at: {}", ferrox_docs_root.display());
        std::process::exit(1);
    }

    println!("\n[Export] Deploying dist/ artifacts into: {}", ferrox_docs_root.display());

    // Copy dist/ contents into ferrox-docs/
    if let Ok(entries) = fs::read_dir(&dist_dir) {
        let mut count = 0;
        for entry in entries.flatten() {
            let src = entry.path();
            let file_name = entry.file_name();
            let dest = ferrox_docs_root.join(&file_name);

            if src.is_dir() {
                let _ = copy_dir_all(&src, &dest);
            } else {
                let _ = fs::copy(&src, &dest);
            }
            count += 1;
        }
        println!("  -> Successfully exported {} items to ferrox-docs root.", count);
    }

    // Ensure CNAME and .nojekyll for GitHub Pages
    let cname_path = ferrox_docs_root.join("CNAME");
    let nojekyll_path = ferrox_docs_root.join(".nojekyll");

    let _ = fs::write(cname_path, "ferrox-rust.dev\n");
    let _ = fs::write(nojekyll_path, "");

    println!("  -> CNAME set to 'ferrox-rust.dev'.");
    println!("  -> .nojekyll flag touched.");

    // Generate AI-Native and SSG artifacts directly into ferrox-docs root
    execute_ssg(&ferrox_docs_root.to_string_lossy());

    println!("\n================================================================");
    println!(">>> EXPORT COMPLETE: ferrox-docs ready for GitHub Pages (main) <<<");
    println!("================================================================");
}

fn execute_serve() {
    println!("Starting Ferrox Codex local development server via Trunk...");
    let crate_dir = env::current_dir().unwrap();
    let _ = Command::new("trunk")
        .args(&["serve", "--port", "8080"])
        .current_dir(&crate_dir)
        .status();
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &dst.join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn execute_sync(sources: &str, out_file: &str) {
    println!("================================================================");
    println!("     FERROX-CODEX: SCANNING & INGESTING ECOSYSTEM DOCS SOURCES   ");
    println!("================================================================");

    let sources_path = PathBuf::from(sources);
    println!("Scanning sources root : {}", sources_path.display());

    match ferrox_codex::sync::scanner::scan_sources(&sources_path) {
        Ok(bundle) => {
            println!("  -> Successfully ingested {} markdown documents across ecosystem repositories!", bundle.total_docs);
            for doc in &bundle.docs {
                println!("     • [{}] {} -> {}", doc.module_id, doc.title, doc.full_route);
            }

            if let Ok(json) = bundle.to_json() {
                let out_path = PathBuf::from(out_file);
                if let Err(e) = fs::write(&out_path, json) {
                    eprintln!("[ERROR] Failed to write bundle to {}: {}", out_path.display(), e);
                } else {
                    println!("  -> Saved ecosystem documentation bundle to: {}", out_path.display());
                }
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Ingestion failed: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn execute_sync(_sources: &str, _out_file: &str) {}

#[cfg(not(target_arch = "wasm32"))]
fn execute_ssg(out_dir: &str) {
    println!("\n================================================================");
    println!("     FERROX-CODEX: GENERATING AI-NATIVE ARTIFACTS & SSG        ");
    println!("================================================================");

    let target_out = PathBuf::from(out_dir);
    let _ = fs::create_dir_all(&target_out);
    let registry = ferrox_codex::DocRegistry::new();

    // 1. Generate llms.txt
    let llms_txt = ferrox_codex::ssg::generate_llms_txt(&registry);
    let llms_path = target_out.join("llms.txt");
    if let Err(e) = fs::write(&llms_path, llms_txt) {
        eprintln!("[WARN] Failed to write llms.txt: {}", e);
    } else {
        println!("  -> Generated AI-Native index: {}", llms_path.display());
    }

    // 2. Generate llms-full.txt
    let llms_full = ferrox_codex::ssg::generate_llms_full_txt(&registry);
    let llms_full_path = target_out.join("llms-full.txt");
    if let Err(e) = fs::write(&llms_full_path, llms_full) {
        eprintln!("[WARN] Failed to write llms-full.txt: {}", e);
    } else {
        println!("  -> Generated Full Context Corpus: {}", llms_full_path.display());
    }

    // 3. Generate SSG pre-renders
    match ferrox_codex::ssg::generate_ssg_pre_renders(&target_out, &registry) {
        Ok(count) => {
            println!("  -> Generated {} pre-rendered static HTML routes with OpenGraph tags.", count);
        }
        Err(e) => {
            eprintln!("[WARN] Failed to generate SSG pre-renders: {}", e);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn execute_ssg(_out_dir: &str) {}



