//! # Ecosystem Multi-Repo Documentation Ingestion (`ferrox-codex::sync`)
//!
//! Automated ingestion engine that recursively scans all documentation sources
//! across the Ferrox ecosystem repositories, extracts YAML frontmatter,
//! and builds a consolidated in-memory bundle and search catalog.

use serde::{Deserialize, Serialize};
use crate::markdown::DocMetadata;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestedDoc {
    pub module_id: String,
    pub relative_path: String,
    pub full_route: String,
    pub title: String,
    pub metadata: DocMetadata,
    pub raw_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemBundle {
    pub generated_at: String,
    pub total_docs: usize,
    pub docs: Vec<IngestedDoc>,
}

impl EcosystemBundle {
    pub fn new() -> Self {
        Self {
            generated_at: String::new(),
            total_docs: 0,
            docs: Vec::new(),
        }
    }

    pub fn add_doc(&mut self, doc: IngestedDoc) {
        self.docs.push(doc);
        self.total_docs = self.docs.len();
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

pub use crate::markdown::extract_frontmatter;

#[cfg(not(target_arch = "wasm32"))]
pub mod scanner {
    use super::*;
    use std::fs;
    use std::path::{Path, PathBuf};

    /// Scans an ecosystem sources directory or repository root for documentation files.
    pub fn scan_sources(root_dir: &Path) -> std::io::Result<EcosystemBundle> {
        let mut bundle = EcosystemBundle::new();
        bundle.generated_at = format!("{:?}", std::time::SystemTime::now());

        // Potential documentation locations to probe
        let probe_targets = vec![
            ("core", root_dir.join("ferrox/docs")),
            ("front", root_dir.join("ferrox-front/docs")),
            ("node", root_dir.join("ferrox-node/docs")),
            ("node", root_dir.join("node-yalc/docs")),
            ("node", root_dir.join("nestjs-yalc/docs")),
            ("python", root_dir.join("ferrox-py/docs")),
            ("toolkit", root_dir.join("ferrox-agent-toolkit/docs")),
            ("docs", root_dir.join("ferrox-docs/docs")),
            ("docs", root_dir.to_path_buf()),
        ];

        let mut scanned_paths = std::collections::HashSet::new();

        for (module_id, target_dir) in probe_targets {
            if target_dir.exists() && target_dir.is_dir() {
                scan_dir_recursive(&target_dir, &target_dir, module_id, &mut bundle, &mut scanned_paths)?;
            }
        }

        Ok(bundle)
    }

    fn scan_dir_recursive(
        base_dir: &Path,
        current_dir: &Path,
        module_id: &str,
        bundle: &mut EcosystemBundle,
        scanned: &mut std::collections::HashSet<PathBuf>,
    ) -> std::io::Result<()> {
        if let Ok(entries) = fs::read_dir(current_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    // Skip hidden dirs, target, node_modules
                    if !dir_name.starts_with('.') && dir_name != "target" && dir_name != "node_modules" && dir_name != "dist" {
                        scan_dir_recursive(base_dir, &path, module_id, bundle, scanned)?;
                    }
                } else if path.is_file() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    if ext == "md" || ext == "mdx" {
                        let canonical = path.canonicalize().unwrap_or_else(|_| path.clone());
                        if scanned.insert(canonical) {
                            if let Ok(content) = fs::read_to_string(&path) {
                                let rel = path.strip_prefix(base_dir).unwrap_or(&path);
                                let rel_str = rel.to_string_lossy().replace('\\', "/");
                                let route = rel_str.trim_end_matches(".md").trim_end_matches(".mdx");

                                let (metadata, body) = extract_frontmatter(&content);
                                let title = metadata.title.clone()
                                    .or_else(|| {
                                        // Try first h1 heading
                                        for line in body.lines() {
                                            let l = line.trim();
                                            if l.starts_with("# ") {
                                                return Some(l.trim_start_matches("# ").trim().to_string());
                                            }
                                        }
                                        None
                                    })
                                    .unwrap_or_else(|| route.to_string());

                                let full_route = format!("/{}/{}", module_id, route.trim_start_matches('/'));

                                bundle.add_doc(IngestedDoc {
                                    module_id: module_id.to_string(),
                                    relative_path: rel_str,
                                    full_route,
                                    title,
                                    metadata,
                                    raw_content: content,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
