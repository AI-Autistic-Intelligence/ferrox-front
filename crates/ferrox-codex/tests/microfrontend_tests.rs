use ferrox_codex::markdown::{is_mermaid_block, parse_markdown};
use ferrox_codex::registry::DocRegistry;
use ferrox_codex::sync::{extract_frontmatter, EcosystemBundle};

#[test]
fn test_mermaid_diagram_container_rendering() {
    let md = r#"# Architecture Overview

Here is the transaction pipeline:

```mermaid
sequenceDiagram
    participant Client
    participant Wasm as Ferrox Wasm
    participant Core as Rust Core Engine
    Client->>Wasm: DOM Click Event
    Wasm->>Core: Zero-Copy Pointer
    Core-->>Wasm: Linear Memory State
    Wasm-->>Client: Direct DOM Update
```

And next paragraph.
"#;

    let parsed = parse_markdown("mermaid-test", md);
    assert_eq!(parsed.title, "Architecture Overview");
    assert!(is_mermaid_block("mermaid"), "Language 'mermaid' must be identified");
    assert!(is_mermaid_block(" Mermaid  "), "Trimmed and case-insensitive check");
    assert!(!is_mermaid_block("rust"), "Other languages must not match mermaid");
    assert!(parsed.raw_content.contains("```mermaid"));
    assert!(parsed.raw_content.contains("sequenceDiagram"));
    assert!(parsed.search_text.contains("sequenceDiagram"));
    assert!(parsed.search_text.contains("Zero-Copy Pointer"));
}

#[test]
fn test_interactive_tabs_preprocessing() {
    let md = r#"# Multi-Language Example

<Tabs>
<TabItem value="rust" label="Rust (Core)">
```rust
fn main() { println!("Hello Ferrox"); }
```
</TabItem>
<TabItem value="py" label="Python (ferrox-py)">
```python
def handler(): return {"status": "ok"}
```
</TabItem>
</Tabs>
"#;

    let parsed = parse_markdown("tabs-test", md);
    // 1. Preprocessor translates <Tabs> into ```tabs and <TabItem> into <!-- tab-start: ... -->
    assert!(parsed.raw_content.contains("```tabs"), "Tabs codeblock must be generated");
    assert!(parsed.raw_content.contains("<!-- tab-start: Rust (Core) -->"), "Rust tab marker must be present");
    assert!(parsed.raw_content.contains("<!-- tab-start: Python (ferrox-py) -->"), "Python tab marker must be present");
    assert!(parsed.raw_content.contains("<!-- tab-end -->"), "Tab end markers must be present");
    assert!(parsed.raw_content.contains("fn main()"), "Rust code content must be preserved");
    assert!(parsed.raw_content.contains("def handler()"), "Python code content must be preserved");
}

#[test]
fn test_nested_submodule_route_resolution() {
    let registry = DocRegistry::new();

    // 1. Root & Core module routes
    let overview = registry.get("/overview");
    assert!(overview.is_some(), "/overview should resolve");

    let core_arch = registry.get("/core/architecture");
    assert!(core_arch.is_some(), "/core/architecture should resolve in CoreEngineModule");

    // 2. Front module routes
    let front_datagrid = registry.get("/front/datagrid");
    assert!(front_datagrid.is_some(), "/front/datagrid should resolve in FrontWasmModule");
    let front_charts = registry.get("/front/charts");
    assert!(front_charts.is_some(), "/front/charts should resolve in FrontWasmModule");

    // 3. Node & Yalc module routes
    let node_yalc = registry.get("/node/yalc");
    assert!(node_yalc.is_some(), "/node/yalc should resolve in NodeYalcModule");

    // 4. Recursive nested submodules in Python module: /py/auth/*, /py/commerce/*, /py/utils/*
    let py_auth = registry.get("/py/auth/overview");
    assert!(py_auth.is_some(), "/py/auth/overview should resolve in PythonModule -> AuthSubmodule");

    let py_commerce = registry.get("/py/commerce/gateways");
    assert!(py_commerce.is_some(), "/py/commerce/gateways should resolve in PythonModule -> CommerceSubmodule");

    let py_utils = registry.get("/py/utils/pipelines");
    assert!(py_utils.is_some(), "/py/utils/pipelines should resolve in PythonModule -> UtilsSubmodule");

    // 5. Polyglot module routes
    let poly_php = registry.get("/polyglot/php");
    assert!(poly_php.is_some(), "/polyglot/php should resolve in PolyglotModule");

    let poly_java = registry.get("/polyglot/java");
    assert!(poly_java.is_some(), "/polyglot/java should resolve in PolyglotModule");
}

#[test]
fn test_cross_module_unified_search() {
    let registry = DocRegistry::new();

    // Search across core engine
    let hits_core = registry.search("BTreeMap");
    assert!(!hits_core.is_empty(), "Should find BTreeMap in CoreEngine docs");
    assert!(hits_core[0].score > 0);

    // Search across front module
    let hits_front = registry.search("DataGrid");
    assert!(!hits_front.is_empty(), "Should find DataGrid in FrontWasm docs");

    // Search across python recursive submodules
    let hits_py = registry.search("Commerce");
    assert!(!hits_py.is_empty(), "Should find Commerce in Python submodule docs");

    // Search across polyglot module
    let hits_poly = registry.search("Hexagonal");
    assert!(!hits_poly.is_empty(), "Should find Hexagonal in Polyglot Java docs");
}

#[test]
fn test_frontmatter_extraction_and_bundle() {
    let md = r#"---
title: Custom Module Page
sidebar_position: 2
description: Comprehensive overview of modular microfrontends.
tags:
  - microfrontend
  - wasm
---
# Custom Module Page
Body content here.
"#;

    let (meta, body) = extract_frontmatter(md);
    assert_eq!(meta.title.as_deref(), Some("Custom Module Page"));
    assert_eq!(meta.description.as_deref(), Some("Comprehensive overview of modular microfrontends."));
    assert_eq!(meta.tags.len(), 2);
    assert!(body.starts_with("# Custom Module Page"));

    let mut bundle = EcosystemBundle::new();
    bundle.add_doc(ferrox_codex::sync::IngestedDoc {
        module_id: "test".to_string(),
        relative_path: "test/page.md".to_string(),
        full_route: "/test/page".to_string(),
        title: meta.title.clone().unwrap(),
        metadata: meta,
        raw_content: md.to_string(),
    });

    assert_eq!(bundle.total_docs, 1);
    let json = bundle.to_json().expect("Bundle serialization to JSON must succeed");
    assert!(json.contains("Custom Module Page"));

    let deserialized = EcosystemBundle::from_json(&json).expect("Bundle deserialization must succeed");
    assert_eq!(deserialized.total_docs, 1);
    assert_eq!(deserialized.docs[0].full_route, "/test/page");
}

#[test]
fn test_llms_txt_and_full_generation() {
    let registry = DocRegistry::new();

    // 1. llms.txt standard format
    let llms_txt = ferrox_codex::ssg::generate_llms_txt(&registry);
    assert!(llms_txt.contains("# Ferrox Ecosystem Documentation"));
    assert!(llms_txt.contains("https://llmstxt.org") || llms_txt.contains("Core Architecture"));
    assert!(llms_txt.contains("Ferrox Core"));
    assert!(llms_txt.contains("Ferrox Front"));
    assert!(llms_txt.contains("https://ferrox-rust.dev/overview"));

    // 2. llms-full.txt complete corpus
    let llms_full = ferrox_codex::ssg::generate_llms_full_txt(&registry);
    assert!(llms_full.contains("# Ferrox Ecosystem • Complete LLM Context Corpus"));
    assert!(llms_full.contains("ROUTE: /overview"));
    assert!(llms_full.contains("ROUTE: /front"));
}

#[test]
fn test_i18n_route_and_locales() {
    use ferrox_codex::i18n::{parse_route_locale, SupportedLocale};

    let (loc1, p1) = parse_route_locale("/it/overview");
    assert_eq!(loc1, SupportedLocale::It);
    assert_eq!(p1, "/overview");

    let (loc2, p2) = parse_route_locale("/es/front/datagrid");
    assert_eq!(loc2, SupportedLocale::Es);
    assert_eq!(p2, "/front/datagrid");

    let (loc3, p3) = parse_route_locale("/overview");
    assert_eq!(loc3, SupportedLocale::En);
    assert_eq!(p3, "/overview");

    assert_eq!(SupportedLocale::It.search_placeholder(), "Cerca nella documentazione... (Ctrl+K)");
    assert_eq!(SupportedLocale::En.search_placeholder(), "Search documentation... (Ctrl+K)");
}

#[test]
fn test_interactive_views_registry() {
    let registry = DocRegistry::new();

    let datagrid_view = registry.get_interactive_view("/front/datagrid/playground");
    assert!(datagrid_view.is_some(), "/front/datagrid/playground interactive view should be registered");

    let charts_view = registry.get_interactive_view("/front/charts/live");
    assert!(charts_view.is_some(), "/front/charts/live interactive view should be registered");

    let non_existent = registry.get_interactive_view("/non/existent/view");
    assert!(non_existent.is_none());
}

