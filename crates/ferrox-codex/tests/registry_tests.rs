use ferrox_codex::registry::DocRegistry;

#[test]
fn test_route_resolution() {
    let registry = DocRegistry::new();

    // 1. /overview
    let doc_overview = registry.get("/overview");
    assert!(doc_overview.is_some(), "Path /overview should resolve to a document");
    assert_eq!(doc_overview.unwrap().path, "overview");

    // 2. /overview/example-projects
    let doc_examples = registry.get("/overview/example-projects");
    assert!(doc_examples.is_some(), "Path /overview/example-projects should resolve");
    assert_eq!(doc_examples.unwrap().path, "overview/example-projects");

    // 3. /showcase alias
    let doc_showcase = registry.get("/showcase");
    assert!(doc_showcase.is_some(), "Path /showcase should resolve via alias");

    // 4. /benchmarks prefix
    let doc_bench = registry.get("/benchmarks");
    assert!(doc_bench.is_some(), "Path /benchmarks should resolve via section prefix");
    assert!(doc_bench.unwrap().path.starts_with("benchmarks"));
}

#[test]
fn test_route_fallback() {
    let registry = DocRegistry::new();

    // Unknown route should return None with get()
    let unknown = registry.get("/this/route/does/not/exist");
    assert!(unknown.is_none(), "Unknown route should return None");

    // get_or_fallback should fall back cleanly to overview
    let fallback_doc = registry.get_or_fallback("/this/route/does/not/exist");
    assert_eq!(fallback_doc.path, "overview", "Fallback should default to overview");
}

#[test]
fn test_pagination_sequence() {
    let registry = DocRegistry::new();

    // First document in ordered routes should have no previous, but a next
    let (first_prev, first_next) = registry.prev_next("overview");
    assert!(first_prev.is_none(), "First document should have no previous page");
    assert!(first_next.is_some(), "First document should have a next page");

    // Middle document should have both previous and next
    let (mid_prev, mid_next) = registry.prev_next("ferrox-front/components");
    assert!(mid_prev.is_some(), "Components doc should have a previous page");
    assert!(mid_next.is_some(), "Components doc should have a next page");

    // Unknown document should return (None, None)
    let (none_prev, none_next) = registry.prev_next("non-existent-page");
    assert!(none_prev.is_none());
    assert!(none_next.is_none());
}

#[test]
fn test_in_memory_search_engine() {
    let registry = DocRegistry::new();

    // 1. Search for "DataGrid"
    let hits_datagrid = registry.search("DataGrid");
    assert!(!hits_datagrid.is_empty(), "Search for 'DataGrid' should return results");
    assert!(hits_datagrid[0].score > 0, "Top hit score should be positive");
    assert!(!hits_datagrid[0].snippet.is_empty(), "Top hit should contain a snippet");

    // 2. Search for "Zero-JS" (or "zero")
    let hits_zero = registry.search("zero");
    assert!(!hits_zero.is_empty(), "Search for 'zero' should return results");
    assert!(hits_zero[0].score > 0);

    // 3. Search for "1,000,000" or "RPS"
    let hits_bench = registry.search("1,000,000");
    assert!(!hits_bench.is_empty(), "Search for '1,000,000' should match benchmark document");
    assert!(hits_bench[0].score > 0);
    assert!(!hits_bench[0].snippet.is_empty());
}
