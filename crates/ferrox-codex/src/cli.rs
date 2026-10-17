//! # Ferrox Codex CLI Implementation (`ferrox-codex::cli`)

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Main entry point for the Ferrox Codex CLI.
pub fn run_cli() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("build");

    match command {
        "init" => {
            let target_name = args.get(2).map(|s| s.as_str()).unwrap_or("my-docs");
            execute_init(target_name);
        }
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

pub fn print_help() {
    println!(r#"
⚡ Ferrox Codex CLI • Native WebAssembly Documentation Generator
Usage:
  ferrox-codex init [<project_name>]       Initializes a new documentation starter project
  ferrox-codex build [--out-dir <dir>]     Builds release Wasm distribution & AI ssg
  ferrox-codex ssg [--out-dir <dir>]       Generates llms.txt, llms-full.txt & static SEO pages
  ferrox-codex sync [--sources <dir>]      Scans & ingests all ecosystem docs into bundle
  ferrox-codex export-gh-pages            Builds and exports directly to ferrox-docs for GitHub Pages
  ferrox-codex serve                      Starts local development server with hot-reloading
  ferrox-codex help                       Prints this help message
"#);
}

pub fn execute_init(target_name: &str) {
    let target_dir = PathBuf::from(target_name);
    println!("⚡ Initializing new Ferrox Codex documentation project in '{}'...", target_dir.display());
    let _ = fs::create_dir_all(target_dir.join("docs"));

    let overview_md = r#"---
title: Welcome to My Docs
sidebar_position: 1
---

# 🚀 Welcome to Your Documentation

Built with **Ferrox Codex** — the pure Rust & WebAssembly documentation platform.

## Features
- **Zero JavaScript Runtime Overhead**
- **Native SVG Mermaid Diagrams**
- **AI-Native (`llms.txt`) generation**
- **Blazing Fast 60 FPS Single Page Application**
"#;

    let trunk_toml = r#"[build]
target = "index.html"
release = true

[serve]
port = 8080
open = false
"#;

    let index_html = r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>Ferrox Codex Documentation</title>
  <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700;800;900&family=JetBrains+Mono:wght@400;500;700&display=swap" />
  <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-slate-950 text-slate-100 min-h-screen">
  <div id="root"></div>
</body>
</html>
"#;

    let _ = fs::write(target_dir.join("docs").join("overview.md"), overview_md);
    let _ = fs::write(target_dir.join("Trunk.toml"), trunk_toml);
    let _ = fs::write(target_dir.join("index.html"), index_html);

    println!("  -> Created docs/overview.md");
    println!("  -> Created Trunk.toml");
    println!("  -> Created index.html");
    println!("\n✅ Project '{}' created successfully!", target_name);
    println!("\nNext steps:");
    println!("  cd {}", target_name);
    println!("  ferrox-codex serve");
}

pub fn get_arg_value(args: &[String], flag: &str) -> Option<String> {
    for i in 0..args.len() {
        if args[i] == flag && i + 1 < args.len() {
            return Some(args[i + 1].clone());
        }
    }
    None
}

pub fn find_crate_dir() -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let p = PathBuf::from(manifest_dir);
        if p.join("Trunk.toml").exists() {
            return p;
        }
    }

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

    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

pub fn find_ferrox_docs_dir(crate_dir: &Path) -> PathBuf {
    let rel = crate_dir.join("../../../ferrox-docs");
    if rel.exists() {
        return rel.canonicalize().unwrap_or(rel);
    }

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

pub fn execute_build(out_dir: &str) {
    println!("================================================================");
    println!("       FERROX-CODEX: COMPILING NATIVE WASM DOCUMENTATION        ");
    println!("================================================================");

    let crate_dir = find_crate_dir();
    let crate_dir = crate_dir.canonicalize().unwrap_or(crate_dir);
    println!("Crate Directory : {}", crate_dir.display());
    println!("Output Target   : {}", out_dir);

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

    execute_ssg(&target_out.to_string_lossy());

    println!("\n================================================================");
    println!(">>> BUILD COMPLETE: Zero JS • Pure Rust WebAssembly Codex <<<");
    println!("================================================================");
}

pub fn execute_export_gh_pages() {
    println!("================================================================");
    println!("    FERROX-CODEX: EXPORTING NATIVE WASM TO GITHUB PAGES ROOT    ");
    println!("================================================================");

    let crate_dir = find_crate_dir();
    let crate_dir = crate_dir.canonicalize().unwrap_or(crate_dir);

    execute_build("dist");

    let ferrox_docs_root = find_ferrox_docs_dir(&crate_dir);
    let dist_dir = crate_dir.join("dist");

    if !ferrox_docs_root.exists() {
        eprintln!("[ERROR] ferrox-docs root directory not found at: {}", ferrox_docs_root.display());
        std::process::exit(1);
    }

    println!("\n[Export] Deploying dist/ artifacts into: {}", ferrox_docs_root.display());

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

    let cname_path = ferrox_docs_root.join("CNAME");
    let nojekyll_path = ferrox_docs_root.join(".nojekyll");

    let _ = fs::write(cname_path, "ferrox-rust.dev\n");
    let _ = fs::write(nojekyll_path, "");

    println!("  -> CNAME set to 'ferrox-rust.dev'.");
    println!("  -> .nojekyll flag touched.");

    execute_ssg(&ferrox_docs_root.to_string_lossy());

    println!("\n================================================================");
    println!(">>> EXPORT COMPLETE: ferrox-docs ready for GitHub Pages (main) <<<");
    println!("================================================================");
}

pub fn execute_serve() {
    println!("Starting Ferrox Codex local development server via Trunk...");
    let crate_dir = find_crate_dir();
    let _ = Command::new("trunk")
        .args(&["serve", "--port", "8080"])
        .current_dir(&crate_dir)
        .status();
}

pub fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
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
pub fn execute_sync(sources: &str, out_file: &str) {
    println!("================================================================");
    println!("     FERROX-CODEX: SCANNING & INGESTING ECOSYSTEM DOCS SOURCES   ");
    println!("================================================================");

    let sources_path = PathBuf::from(sources);
    println!("Scanning sources root : {}", sources_path.display());

    match crate::sync::scanner::scan_sources(&sources_path) {
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
pub fn execute_sync(_sources: &str, _out_file: &str) {}

#[cfg(not(target_arch = "wasm32"))]
pub fn execute_ssg(out_dir: &str) {
    println!("\n================================================================");
    println!("     FERROX-CODEX: GENERATING AI-NATIVE ARTIFACTS & SSG        ");
    println!("================================================================");

    let target_out = PathBuf::from(out_dir);
    let _ = fs::create_dir_all(&target_out);
    let registry = crate::DocRegistry::new();

    let llms_txt = crate::ssg::generate_llms_txt(&registry);
    let llms_path = target_out.join("llms.txt");
    if let Err(e) = fs::write(&llms_path, llms_txt) {
        eprintln!("[WARN] Failed to write llms.txt: {}", e);
    } else {
        println!("  -> Generated AI-Native index: {}", llms_path.display());
    }

    let llms_full = crate::ssg::generate_llms_full_txt(&registry);
    let llms_full_path = target_out.join("llms-full.txt");
    if let Err(e) = fs::write(&llms_full_path, llms_full) {
        eprintln!("[WARN] Failed to write llms-full.txt: {}", e);
    } else {
        println!("  -> Generated Full Context Corpus: {}", llms_full_path.display());
    }

    match crate::ssg::generate_ssg_pre_renders(&target_out, &registry) {
        Ok(count) => {
            println!("  -> Generated {} pre-rendered static HTML routes with OpenGraph tags.", count);
        }
        Err(e) => {
            eprintln!("[WARN] Failed to generate SSG pre-renders: {}", e);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub fn execute_ssg(_out_dir: &str) {}
