use ferrox_codex::markdown::{parse_markdown, preprocess_docusaurus_admonitions, slugify};
use ferrox_codex::highlighter::highlight_code;

#[test]
fn test_heading_slugification() {
    assert_eq!(slugify("Why Zero-JS?"), "why-zero-js");
    assert_eq!(slugify("1,000,000 Req/Sec Performance"), "1-000-000-req-sec-performance");
    assert_eq!(slugify("Getting Started with Ferrox"), "getting-started-with-ferrox");
    assert_eq!(slugify("Special #$% Characters & Symbols"), "special-characters-symbols");
}

#[test]
fn test_heading_extraction_in_toc() {
    let md = r#"---
title: Sample Architecture
---
# Main Architecture

## Core Engine
Description of core engine.

### Reactive Signals
Signals provide direct updates.

## Performance Benchmark
Numbers and stats.
"#;

    let parsed = parse_markdown("architecture", md);
    assert_eq!(parsed.title, "Sample Architecture");
    // H1 is document title; H2-H4 are TOC anchor items
    assert_eq!(parsed.headings.len(), 3);

    assert_eq!(parsed.headings[0].title, "Core Engine");
    assert_eq!(parsed.headings[0].level, 2);
    assert_eq!(parsed.headings[0].id, "core-engine");

    assert_eq!(parsed.headings[1].title, "Reactive Signals");
    assert_eq!(parsed.headings[1].level, 3);
    assert_eq!(parsed.headings[1].id, "reactive-signals");

    assert_eq!(parsed.headings[2].title, "Performance Benchmark");
    assert_eq!(parsed.headings[2].level, 2);
    assert_eq!(parsed.headings[2].id, "performance-benchmark");
}

#[test]
fn test_admonitions_parsing_and_preprocessing() {
    // 1. Docusaurus :::tip syntax
    let docusaurus_callout = r#":::tip Performance Tip
Always favor signals over virtual DOM diffing.
:::"#;
    let preprocessed = preprocess_docusaurus_admonitions(docusaurus_callout);
    assert!(preprocessed.contains("> [!TIP] Performance Tip"));
    assert!(preprocessed.contains("Always favor signals"));

    // 2. GFM > [!WARNING] syntax in document
    let gfm_doc = r#"# Safety
> [!WARNING] Production Security
> Ensure SealedToken is stored in WebAssembly linear memory only.
"#;
    let parsed = parse_markdown("security", gfm_doc);
    assert!(parsed.search_text.contains("Production Security") || parsed.search_text.contains("SealedToken"));
}

#[test]
fn test_code_block_highlighting() {
    // 1. Rust code highlighting
    let rust_code = r#"pub fn calculate() -> u32 { 42 }"#;
    let highlighted_rust = highlight_code(rust_code, Some("rust"));
    assert!(highlighted_rust.contains("text-purple-400"), "Rust keywords should have purple styling");
    assert!(highlighted_rust.contains("pub") && highlighted_rust.contains("fn"));

    // 2. TypeScript/JS code highlighting
    let ts_code = r#"const banner: string = "Ferrox";"#;
    let highlighted_ts = highlight_code(ts_code, Some("typescript"));
    assert!(highlighted_ts.contains("text-purple-400"));

    // 3. Bash code highlighting
    let bash_code = r#"cargo build --release"#;
    let highlighted_bash = highlight_code(bash_code, Some("bash"));
    assert!(!highlighted_bash.is_empty());
}
