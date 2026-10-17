use ferrox_front_ui::{AdmonitionType, DocNavItem, TocItem};

#[test]
fn test_admonition_type_variants_and_tokens() {
    // 1. Note variant
    let note = AdmonitionType::Note;
    assert_eq!(note.title(), "NOTE");
    assert_eq!(note.icon_name(), "info");
    assert_eq!(note.border_color(), "#6366f1");

    // 2. Tip variant
    let tip = AdmonitionType::Tip;
    assert_eq!(tip.title(), "TIP");
    assert_eq!(tip.icon_name(), "zap");
    assert_eq!(tip.border_color(), "#10b981");

    // 3. Warning variant
    let warning = AdmonitionType::Warning;
    assert_eq!(warning.title(), "WARNING");
    assert_eq!(warning.icon_name(), "alert-triangle");
    assert_eq!(warning.border_color(), "#f59e0b");

    // 4. Danger variant
    let danger = AdmonitionType::Danger;
    assert_eq!(danger.title(), "DANGER");
    assert_eq!(danger.icon_name(), "alert-circle");
    assert_eq!(danger.border_color(), "#ef4444");

    // 5. Caution variant
    let caution = AdmonitionType::Caution;
    assert_eq!(caution.title(), "CAUTION");
    assert_eq!(caution.icon_name(), "shield");
    assert_eq!(caution.border_color(), "#f97316");

    // 6. Loose string parsing
    assert_eq!(AdmonitionType::from_str_loose("tip"), AdmonitionType::Tip);
    assert_eq!(AdmonitionType::from_str_loose("WARNING"), AdmonitionType::Warning);
    assert_eq!(AdmonitionType::from_str_loose("danger"), AdmonitionType::Danger);
    assert_eq!(AdmonitionType::from_str_loose("other"), AdmonitionType::Note);
}

#[test]
fn test_toc_item_creation() {
    let toc_h2 = TocItem::new("architecture", "Architecture Overview", 2);
    assert_eq!(toc_h2.id, "architecture");
    assert_eq!(toc_h2.title, "Architecture Overview");
    assert_eq!(toc_h2.level, 2);

    let toc_h3 = TocItem::new("signals", "Zero-Cost Signals", 3);
    assert_eq!(toc_h3.id, "signals");
    assert_eq!(toc_h3.level, 3);
}

#[test]
fn test_doc_nav_item_tree_and_expansion_state() {
    // 1. Creation with builder pattern
    let item1 = DocNavItem::new("item-intro", "Introduction")
        .with_href("/overview")
        .with_icon("book-open");

    assert_eq!(item1.id, "item-intro");
    assert_eq!(item1.label, "Introduction");
    assert_eq!(item1.href.as_deref(), Some("/overview"));
    assert_eq!(item1.icon.as_deref(), Some("book-open"));
    assert!(!item1.is_expanded.get(), "Item should start collapsed by default");

    // 2. Child nodes inside a category branch
    let sub1 = DocNavItem::new("sub-1", "Component Catalog")
        .with_href("/ferrox-front/components");
    let sub2 = DocNavItem::new("sub-2", "DataGrid")
        .with_href("/ferrox-front/datagrid")
        .with_badge("1M-Rows");

    let branch = DocNavItem::new("cat-front", "Ferrox Front Engine")
        .with_icon("layers")
        .with_children(vec![sub1, sub2]);

    assert_eq!(branch.children.len(), 2);
    assert_eq!(branch.children[1].badge.as_deref(), Some("1M-Rows"));

    // 3. Test branch starts collapsed by default
    assert!(!branch.is_expanded.get(), "Branch starts collapsed by default");

    // Test has_active_child
    assert!(branch.has_active_child("/ferrox-front/datagrid"));
    assert!(branch.has_active_child("ferrox-front/components"));
    assert!(!branch.has_active_child("/ferrox/unknown"));
    
    // Expand branch
    branch.is_expanded.set(true);
    assert!(branch.is_expanded.get(), "Branch should now be expanded");

    // Re-collapse branch
    branch.is_expanded.set(false);
    assert!(!branch.is_expanded.get(), "Branch should be collapsed again");
}
