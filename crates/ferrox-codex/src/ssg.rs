//! # Static Site Generation & AI-Native Layer (`ferrox-codex::ssg`)
//!
//! Generates:
//! 1. `llms.txt`: Standardized machine-readable ecosystem overview and sitemap for AI agents.
//! 2. `llms-full.txt`: Consolidated plaintext documentation corpus for LLM fine-tuning and context retrieval.
//! 3. Pre-rendered static HTML with OpenGraph social preview tags and SEO crawlable content.

#[cfg(not(target_arch = "wasm32"))]
use std::fs;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;
use crate::registry::DocRegistry;

/// Builds `llms.txt` following the https://llmstxt.org specification.
pub fn generate_llms_txt(registry: &DocRegistry) -> String {
    let mut out = String::new();
    out.push_str("# Ferrox Ecosystem Documentation\n\n");
    out.push_str("> High-performance, zero-overhead polyglot software ecosystem with 1,000,000 Req/Sec core engine and native Rust WebAssembly UI.\n\n");

    out.push_str("## Core Architecture & Runtimes\n");
    out.push_str("- [Ferrox Core](https://ferrox-rust.dev/core): High-performance Rust HTTP/WebSocket backend engine (1M RPS, BTreeMap routing, zero-copy linear memory).\n");
    out.push_str("- [Ferrox Front](https://ferrox-rust.dev/front): Native WebAssembly reactive UI library with Virtualized DataGrid (1M rows), SVG technical charts, and zero-JS reactivity.\n");
    out.push_str("- [Ferrox Node & Yalc](https://ferrox-rust.dev/node): Node.js & NestJS enterprise microservices layer with CQRS, event-driven sagas, and transport adapters.\n");
    out.push_str("- [Ferrox Python](https://ferrox-rust.dev/py): Asynchronous Python 3.12+ engine with authentication, commerce gateways, and data processing pipelines.\n");
    out.push_str("- [Ferrox Polyglot](https://ferrox-rust.dev/polyglot): Multi-runtime integrations for PHP 8.3+ (Saga/CQRS) and Java 21+ (Hexagonal DDD).\n\n");

    out.push_str("## Documentation Pages Index\n");
    for (route, doc) in registry.all_docs() {
        let desc = doc.metadata.description.as_deref()
            .unwrap_or_else(|| "Ecosystem technical documentation and architecture reference.");
        let clean_desc = desc.replace('\n', " ");
        let truncated: String = clean_desc.chars().take(120).collect();
        out.push_str(&format!("- [{}]({}): {}\n", doc.title, format!("https://ferrox-rust.dev{}", route), truncated));
    }

    out.push_str("\n## Machine-Readable Resources\n");
    out.push_str("- [Full LLM Context Corpus](https://ferrox-rust.dev/llms-full.txt): Complete, consolidated plain-text documentation corpus for deep retrieval.\n");
    out.push_str("- [Documentation Schema Bundle](https://ferrox-rust.dev/docs_bundle.json): Structured JSON export of all documentation nodes.\n");

    out
}

/// Builds `llms-full.txt` containing the entire corpus in markdown format.
pub fn generate_llms_full_txt(registry: &DocRegistry) -> String {
    let mut out = String::new();
    out.push_str("# Ferrox Ecosystem • Complete LLM Context Corpus\n\n");
    out.push_str("This document contains the complete documentation corpus for the Ferrox ecosystem,\n");
    out.push_str("formatted for direct ingestion by Large Language Models (LLMs).\n\n");

    for (route, doc) in registry.all_docs() {
        out.push_str("===============================================================================\n");
        out.push_str(&format!("DOCUMENT: {}\n", doc.title));
        out.push_str(&format!("ROUTE: {}\n", route));
        if let Some(ref d) = doc.metadata.description {
            out.push_str(&format!("DESCRIPTION: {}\n", d));
        }
        if !doc.metadata.tags.is_empty() {
            out.push_str(&format!("TAGS: {}\n", doc.metadata.tags.join(", ")));
        }
        out.push_str("===============================================================================\n\n");
        out.push_str(&doc.raw_content);
        out.push_str("\n\n");
    }

    out
}

/// Generates pre-rendered static HTML files for all routes for SEO and Social OpenGraph previews.
#[cfg(not(target_arch = "wasm32"))]
pub fn generate_ssg_pre_renders(out_dir: &Path, registry: &DocRegistry) -> std::io::Result<usize> {
    let mut count = 0;

    // Load template index.html if available
    let template_path = out_dir.join("index.html");
    let template_html = if template_path.exists() {
        fs::read_to_string(&template_path).unwrap_or_default()
    } else {
        String::new()
    };

    for (route, doc) in registry.all_docs() {
        let clean = route.trim_matches('/');
        if clean.is_empty() {
            continue;
        }

        let target_dir = out_dir.join(clean);
        fs::create_dir_all(&target_dir)?;

        let title = format!("{} • Ferrox Codex", doc.title);
        let desc = doc.metadata.description.as_deref()
            .unwrap_or("Next-gen WebAssembly documentation platform for the Ferrox ecosystem: zero Node.js, vector Mermaid SVG, and interactive Wasm sandboxes.");
        let page_url = format!("https://ferrox-rust.dev/{}", clean);

        let rendered = if !template_html.is_empty() {
            let mut page = template_html.clone();
            // Replace title
            if let Some(start) = page.find("<title>") {
                if let Some(end) = page.find("</title>") {
                    page.replace_range(start + 7..end, &title);
                }
            }
            // Replace description
            page = page.replace(
                "Native WebAssembly documentation engine & static generator for the Ferrox ecosystem. Replaces Docusaurus with pure Rust WebAssembly dogfooding.",
                desc
            );
            // Replace og:title
            page = page.replace(
                "content=\"⚡ Ferrox Codex • Native WebAssembly Documentation Engine\"",
                &format!("content=\"{}\"", title)
            );
            // Replace og:url
            page = page.replace(
                "content=\"https://ferrox-rust.dev\"",
                &format!("content=\"{}\"", page_url)
            );
            // Replace og:description
            page = page.replace(
                "content=\"Next-gen WebAssembly documentation platform for the Ferrox ecosystem: zero Node.js, vector Mermaid SVG, responsive microfrontends, and 1M row live Wasm DataGrid.\"",
                &format!("content=\"{}\"", desc)
            );
            page
        } else {
            format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{}</title>
    <meta name="description" content="{}">
    <meta property="og:title" content="{}">
    <meta property="og:description" content="{}">
    <meta property="og:url" content="{}">
    <meta property="og:type" content="article">
    <meta name="twitter:card" content="summary_large_image">
</head>
<body>
    <h1>{}</h1>
    <p>{}</p>
    <a href="/{}">Read in Interactive WebAssembly Reader</a>
</body>
</html>"#,
                title, desc, title, desc, page_url, doc.title, desc, clean
            )
        };

        fs::write(target_dir.join("index.html"), rendered)?;
        count += 1;
    }

    Ok(count)
}
