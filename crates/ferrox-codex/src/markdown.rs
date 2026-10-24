//! # Markdown Parser & Ferrox DOM Generator (`ferrox-codex::markdown`)
//!
//! Event-driven CommonMark/GFM translator using `pulldown-cmark`.
//! Transmutes Markdown AST nodes directly into `ferrox-front-ui` components:
//! Headings with anchor slugs, CodeBlocks with copy buttons, Admonitions,
//! styled Tables, and SPA Router Links.

use std::rc::Rc;
use pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use serde::{Deserialize, Serialize};
use ferrox_front_core::dom::*;
use ferrox_front_router::Router;
use ferrox_front_ui::{
    admonition, tab_group, mermaid_container, AdmonitionType, code_block_highlighted, TabPanelDef, TocItem,
};
use crate::highlighter::highlight_code;

/// Extracted frontmatter metadata from markdown documents.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DocMetadata {
    pub id: Option<String>,
    pub title: Option<String>,
    pub sidebar_label: Option<String>,
    pub sidebar_position: Option<f64>,
    pub description: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Parsed document representation ready for rendering.
#[derive(Clone)]
pub struct ParsedDoc {
    pub path: String,
    pub metadata: DocMetadata,
    pub title: String,
    pub raw_content: String,
    pub headings: Vec<TocItem>,
    pub search_text: String,
}

/// Helper generating URL-safe slug anchor IDs from heading titles.
pub fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut prev_dash = false;

    for c in text.chars() {
        if c.is_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !slug.is_empty() {
            slug.push('-');
            prev_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

/// Splits frontmatter (between `---` and `---`) and the markdown body.
pub fn extract_frontmatter(content: &str) -> (DocMetadata, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---") {
        return (DocMetadata::default(), content.to_string());
    }

    let rest = &trimmed[3..];
    if let Some(end_idx) = rest.find("\n---") {
        let yaml_str = &rest[..end_idx].trim();
        let body = rest[end_idx + 4..].trim_start().to_string();

        let metadata: DocMetadata = serde_yaml::from_str(yaml_str).unwrap_or_default();
        (metadata, body)
    } else {
        (DocMetadata::default(), content.to_string())
    }
}

/// Preprocesses Docusaurus callout syntax `:::note [title] ... :::` into standard GFM callouts,
/// and normalizes `<Tabs>` / `<TabItem>` and `:::tabs` into interactive tab blocks.
pub fn preprocess_docusaurus_admonitions(markdown: &str) -> String {
    let mut result = String::with_capacity(markdown.len() + 128);
    let mut in_admonition = false;
    let mut current_type = "note";
    let mut current_title: Option<String> = None;

    for line in markdown.lines() {
        let trimmed = line.trim();

        // 1. Support for Docusaurus <Tabs> / <TabItem ...>
        if trimmed.starts_with("<Tabs") {
            result.push_str("```tabs\n");
            continue;
        }
        if trimmed == "</Tabs>" {
            result.push_str("```\n");
            continue;
        }
        if trimmed.starts_with("<TabItem") {
            let label = if let Some(idx) = trimmed.find("label=\"") {
                let after = &trimmed[idx + 7..];
                let end = after.find('"').unwrap_or(after.len());
                &after[..end]
            } else if let Some(idx) = trimmed.find("value=\"") {
                let after = &trimmed[idx + 7..];
                let end = after.find('"').unwrap_or(after.len());
                &after[..end]
            } else {
                "Tab"
            };
            result.push_str(&format!("<!-- tab-start: {} -->\n", label));
            continue;
        }
        if trimmed == "</TabItem>" {
            result.push_str("<!-- tab-end -->\n");
            continue;
        }

        // 2. Support for :::tabs and :::tab [Label] syntax
        if trimmed == ":::tabs" {
            result.push_str("```tabs\n");
            continue;
        }
        if trimmed.starts_with(":::tab") {
            let label = if let Some(start) = trimmed.find('[') {
                let end = trimmed.find(']').unwrap_or(trimmed.len());
                &trimmed[start + 1..end]
            } else {
                let rest = trimmed.trim_start_matches(':').trim_start_matches("tab").trim();
                if rest.is_empty() { "Tab" } else { rest }
            };
            result.push_str(&format!("<!-- tab-start: {} -->\n", label));
            continue;
        }

        // 3. Admonitions :::note, :::tip, :::warning, etc.
        if trimmed.starts_with(":::") && !in_admonition {
            let rest = trimmed.trim_start_matches(':').trim();
            if rest.is_empty() {
                // If closing an open tab inside a tabs block:
                result.push_str("<!-- tab-end -->\n");
                continue;
            }
            in_admonition = true;
            let parts: Vec<&str> = rest.splitn(2, ' ').collect();
            current_type = parts[0];
            current_title = parts.get(1).map(|s| s.trim_matches('[').trim_matches(']').to_string());
            continue;
        } else if trimmed == ":::" && in_admonition {
            in_admonition = false;
            current_title = None;
            continue;
        }

        if in_admonition {
            // Encode as special blockquote format
            if let Some(ref t) = current_title {
                result.push_str(&format!("> [!{}] {}\n> {}\n", current_type.to_uppercase(), t, line));
            } else {
                result.push_str(&format!("> [!{}]\n> {}\n", current_type.to_uppercase(), line));
            }
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Parses a raw Markdown string, extracting metadata, headings for TOC, and returns a `ParsedDoc`.
pub fn parse_markdown(doc_path: &str, raw_markdown: &str) -> ParsedDoc {
    let (mut metadata, body) = extract_frontmatter(raw_markdown);
    let preprocessed = preprocess_docusaurus_admonitions(&body);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&preprocessed, options);

    let mut headings = Vec::new();
    let mut doc_title = metadata.title.clone();
    let mut current_heading_text = String::new();
    let mut in_heading = false;
    let mut current_heading_level = 1;
    let mut search_tokens = String::with_capacity(body.len() / 2);

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                in_heading = true;
                current_heading_level = match level {
                    HeadingLevel::H1 => 1,
                    HeadingLevel::H2 => 2,
                    HeadingLevel::H3 => 3,
                    HeadingLevel::H4 => 4,
                    HeadingLevel::H5 => 5,
                    HeadingLevel::H6 => 6,
                };
                current_heading_text.clear();
            }
            Event::End(TagEnd::Heading(_)) => {
                in_heading = false;
                let title_clean = current_heading_text.trim().to_string();
                if doc_title.is_none() && current_heading_level == 1 {
                    doc_title = Some(title_clean.clone());
                }

                if current_heading_level >= 2 && current_heading_level <= 4 {
                    let slug = slugify(&title_clean);
                    headings.push(TocItem::new(slug, title_clean, current_heading_level as u32));
                }
            }
            Event::Text(t) => {
                if in_heading {
                    current_heading_text.push_str(&t);
                }
                search_tokens.push_str(&t);
                search_tokens.push(' ');
            }
            Event::Code(c) => {
                if in_heading {
                    current_heading_text.push_str(&c);
                }
                search_tokens.push_str(&c);
                search_tokens.push(' ');
            }
            _ => {}
        }
    }

    let final_title = doc_title
        .or_else(|| metadata.title.clone())
        .unwrap_or_else(|| {
            doc_path
                .split('/')
                .last()
                .unwrap_or("Document")
                .replace('-', " ")
                .replace('_', " ")
        });

    if metadata.title.is_none() {
        metadata.title = Some(final_title.clone());
    }

    ParsedDoc {
        path: doc_path.to_string(),
        metadata,
        title: final_title,
        raw_content: preprocessed,
        headings,
        search_text: search_tokens,
    }
}

/// Renders a parsed Markdown document into reactive DOM nodes.
pub fn render_markdown(
    doc: &ParsedDoc,
    router: &Router,
) -> DomBuilder {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&doc.raw_content, options);

    // Render stack
    let root_container = div().class("ferrox-doc-content w-full");
    let mut element_stack: Vec<DomBuilder> = vec![root_container];
    let mut current_heading_level = HeadingLevel::H1;
    let mut current_heading_text = String::new();
    let mut current_code_lang = String::new();
    let mut current_code_text = String::new();
    let mut in_code_block = false;
    let mut in_heading = false;

    let router_rc = Rc::new(router.clone());

    for event in parser {
        match event {
            Event::Start(tag) => match tag {
                Tag::Paragraph => {
                    element_stack.push(p().class("text-sm sm:text-base lg:text-[17px] leading-[1.75] text-slate-200 mb-5 font-normal break-words"));
                }
                Tag::Heading { level, .. } => {
                    in_heading = true;
                    current_heading_level = level;
                    current_heading_text.clear();
                    let h_el = match level {
                        HeadingLevel::H1 => h1().class("font-extrabold text-2xl sm:text-4xl lg:text-[40px] text-white tracking-tight leading-[1.2] pb-4 mb-6 border-b border-slate-800/80 break-words"),
                        HeadingLevel::H2 => h2().class("font-bold text-xl sm:text-2xl lg:text-[28px] text-slate-100 tracking-tight leading-snug mt-10 mb-4 pt-5 border-t border-slate-800/60 group flex items-center gap-2 break-words"),
                        HeadingLevel::H3 => h3().class("font-semibold text-lg sm:text-xl lg:text-2xl text-indigo-300 tracking-tight leading-snug mt-8 mb-3 flex items-center gap-2 break-words"),
                        HeadingLevel::H4 => h4().class("font-semibold text-base sm:text-lg lg:text-xl text-slate-200 mt-6 mb-2.5 break-words"),
                        HeadingLevel::H5 => h5().class("font-semibold text-sm sm:text-base text-slate-300 mt-5 mb-2 break-words"),
                        HeadingLevel::H6 => h6().class("font-medium text-xs sm:text-sm text-slate-400 mt-4 mb-2 uppercase tracking-wider break-words"),
                    };
                    element_stack.push(h_el);
                }
                Tag::CodeBlock(kind) => {
                    in_code_block = true;
                    current_code_text.clear();
                    current_code_lang = match kind {
                        pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                        pulldown_cmark::CodeBlockKind::Indented => "text".to_string(),
                    };
                }
                Tag::List(first_num) => {
                    if first_num.is_some() {
                        element_stack.push(ol().class("list-decimal list-outside space-y-2.5 sm:space-y-3.5 my-5 pl-5 sm:pl-6 text-slate-200 text-sm sm:text-base lg:text-[17px] leading-[1.75]"));
                    } else {
                        element_stack.push(ul().class("list-disc list-outside space-y-2.5 sm:space-y-3.5 my-5 pl-5 sm:pl-6 text-slate-200 text-sm sm:text-base lg:text-[17px] leading-[1.75]"));
                    }
                }
                Tag::Item => {
                    element_stack.push(li().class("leading-[1.75]"));
                }
                Tag::BlockQuote(_) => {
                    element_stack.push(div().class("my-5 sm:my-6 pl-4 sm:pl-5 border-l-4 border-indigo-500 italic text-slate-200 text-sm sm:text-base lg:text-[17px] leading-[1.75] bg-indigo-950/20 py-3 sm:py-4 pr-4 sm:pr-5 rounded-r-2xl"));
                }
                Tag::Table(_alignments) => {
                    let wrapper = div().class("overflow-x-auto my-6 sm:my-8 rounded-xl sm:rounded-2xl border border-slate-800 bg-slate-950/60 backdrop-blur-md shadow-2xl");
                    let tbl = table().class("w-full text-left text-xs sm:text-sm text-slate-200 border-collapse");
                    element_stack.push(wrapper);
                    element_stack.push(tbl);
                }
                Tag::TableHead => {
                    element_stack.push(thead().class("bg-slate-900/90 text-[11px] sm:text-xs uppercase font-bold tracking-wider text-slate-300 border-b border-slate-800"));
                }
                Tag::TableRow => {
                    element_stack.push(tr().class("hover:bg-slate-900/50 transition-colors border-b border-slate-800/40"));
                }
                Tag::TableCell => {
                    element_stack.push(td().class("py-2.5 sm:py-4 px-3 sm:px-5 font-normal"));
                }
                Tag::Emphasis => {
                    element_stack.push(span().class("italic text-slate-200"));
                }
                Tag::Strong => {
                    element_stack.push(span().class("font-bold text-white"));
                }
                Tag::Strikethrough => {
                    element_stack.push(span().class("line-through opacity-60"));
                }
                Tag::Link { dest_url, .. } => {
                    let url_owned = dest_url.to_string();
                    let r_clone = router_rc.clone();

                    let is_internal = url_owned.starts_with('/') || url_owned.starts_with('.') || !url_owned.starts_with("http");

                    let mut link_el = a()
                        .attr("href", &url_owned)
                        .class("text-indigo-400 hover:text-indigo-300 underline underline-offset-4 font-semibold transition-colors");

                    if is_internal {
                        let clean_route = clean_doc_route(&url_owned);
                        link_el = link_el.on("click", move |e: web_sys::Event| {
                            e.prevent_default();
                            r_clone.navigate(&clean_route);
                        });
                    } else {
                        link_el = link_el
                            .attr("target", "_blank")
                            .attr("rel", "noopener noreferrer");
                    }

                    element_stack.push(link_el);
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Paragraph | TagEnd::List(_) | TagEnd::Item | TagEnd::BlockQuote(_)
                | TagEnd::TableHead | TagEnd::TableRow | TagEnd::TableCell
                | TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough | TagEnd::Link => {
                    if element_stack.len() > 1 {
                        let finished = element_stack.pop().unwrap();
                        let parent = element_stack.last_mut().unwrap();
                        *parent = std::mem::replace(parent, div()).child(finished);
                    }
                }
                TagEnd::Table => {
                    if element_stack.len() >= 2 {
                        let tbl = element_stack.pop().unwrap();
                        let mut wrapper = element_stack.pop().unwrap();
                        wrapper = wrapper.child(tbl);
                        let parent = element_stack.last_mut().unwrap();
                        *parent = std::mem::replace(parent, div()).child(wrapper);
                    }
                }
                TagEnd::Heading(_) => {
                    in_heading = false;
                    if element_stack.len() > 1 {
                        let mut heading_el = element_stack.pop().unwrap();
                        let slug = slugify(&current_heading_text);

                        if current_heading_level >= HeadingLevel::H2 && current_heading_level <= HeadingLevel::H4 {
                            heading_el = heading_el
                                .id(&slug)
                                .child(
                                    a()
                                        .attr("href", &format!("#{}", slug))
                                        .class("opacity-0 group-hover:opacity-100 text-indigo-400/80 ml-2 font-mono text-sm transition-opacity no-underline")
                                        .text("#")
                                );
                        }

                        let parent = element_stack.last_mut().unwrap();
                        *parent = std::mem::replace(parent, div()).child(heading_el);
                    }
                }
                TagEnd::CodeBlock => {
                    in_code_block = false;
                    let lang_trimmed = current_code_lang.trim();

                    if is_mermaid_block(lang_trimmed) {
                        let diagram = mermaid_container(&current_code_text, None);
                        let parent = element_stack.last_mut().unwrap();
                        *parent = std::mem::replace(parent, div()).child(diagram);
                    } else if lang_trimmed.eq_ignore_ascii_case("tabs") {
                        let panels = parse_tabs_from_code_block(&current_code_text);
                        if !panels.is_empty() {
                            let tab_comp = tab_group(panels, 0);
                            let parent = element_stack.last_mut().unwrap();
                            *parent = std::mem::replace(parent, div()).child(tab_comp);
                        }
                    } else {
                        let highlighted = highlight_code(&current_code_text, Some(lang_trimmed));

                        let block = code_block_highlighted(
                            Some(&highlighted),
                            &current_code_text,
                            if lang_trimmed.is_empty() { None } else { Some(lang_trimmed) },
                            None,
                        );

                        let parent = element_stack.last_mut().unwrap();
                        *parent = std::mem::replace(parent, div()).child(block);
                    }
                }
                _ => {}
            },
            Event::Text(t) => {
                if in_code_block {
                    current_code_text.push_str(&t);
                } else if in_heading {
                    current_heading_text.push_str(&t);
                    if let Some(parent) = element_stack.last_mut() {
                        *parent = std::mem::replace(parent, div()).text(&t);
                    }
                } else if let Some(parent) = element_stack.last_mut() {
                    // Check for callout indicator e.g. [!NOTE] or [!TIP]
                    let trimmed = t.trim();
                    if trimmed.starts_with("[!") && trimmed.contains(']') {
                        let kind_str = trimmed[2..trimmed.find(']').unwrap_or(2)].to_string();
                        let kind = AdmonitionType::from_str_loose(&kind_str);
                        let rest = trimmed[trimmed.find(']').unwrap_or(2) + 1..].trim();
                        let callout = admonition(kind, None, span().text(rest));
                        *parent = std::mem::replace(parent, div()).child(callout);
                    } else {
                        *parent = std::mem::replace(parent, div()).text(&t);
                    }
                }
            }
            Event::Code(c) => {
                if in_code_block {
                    current_code_text.push_str(&c);
                } else if in_heading {
                    current_heading_text.push_str(&c);
                    if let Some(parent) = element_stack.last_mut() {
                        let code_el = span()
                            .class("px-2 py-0.5 rounded-md font-mono text-[13px] sm:text-sm font-semibold bg-slate-800/90 text-indigo-300 border border-slate-700/80 shadow-xs")
                            .text(&c);
                        *parent = std::mem::replace(parent, div()).child(code_el);
                    }
                } else if let Some(parent) = element_stack.last_mut() {
                    let code_el = span()
                        .class("px-2 py-0.5 rounded-md font-mono text-[13px] sm:text-sm font-semibold bg-slate-800/90 text-indigo-300 border border-slate-700/80 shadow-xs")
                        .text(&c);
                    *parent = std::mem::replace(parent, div()).child(code_el);
                }
            }
            Event::Rule => {
                if let Some(parent) = element_stack.last_mut() {
                    *parent = std::mem::replace(parent, div()).child(
                        ferrox_front_core::dom::DomBuilder::new("hr")
                            .class("my-8 border-slate-800/80")
                    );
                }
            }
            _ => {}
        }
    }

    element_stack.pop().unwrap_or_else(|| div().class("ferrox-doc-content w-full"))
}

/// Normalizes internal documentation links (e.g. `/docs/ferrox-front/overview` or `overview.md` to SPA route `/overview`).
pub fn clean_doc_route(url: &str) -> String {
    let mut clean = url.trim_end_matches(".md");
    if let Some(stripped) = clean.strip_prefix("/docs/") {
        clean = stripped;
    } else if let Some(stripped) = clean.strip_prefix("docs/") {
        clean = stripped;
    } else if let Some(stripped) = clean.strip_prefix("./") {
        clean = stripped;
    }

    if !clean.starts_with('/') {
        format!("/{}", clean)
    } else {
        clean.to_string()
    }
}

/// Checks if a code block language tag indicates a Mermaid diagram.
pub fn is_mermaid_block(lang: &str) -> bool {
    lang.trim().eq_ignore_ascii_case("mermaid")
}

/// Parses multi-tab code blocks formatted with `<!-- tab-start: Label -->` into TabPanelDef items.
pub fn parse_tabs_from_code_block(text: &str) -> Vec<TabPanelDef> {
    let mut panels = Vec::new();
    let mut cur_label: Option<String> = None;
    let mut cur_body = String::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<!-- tab-start:") {
            if let Some(lbl) = cur_label.take() {
                panels.push(create_tab_panel(lbl, &cur_body));
                cur_body.clear();
            }
            let label = trimmed.trim_start_matches("<!-- tab-start:").trim_end_matches("-->").trim();
            cur_label = Some(label.to_string());
        } else if trimmed == "<!-- tab-end -->" {
            if let Some(lbl) = cur_label.take() {
                panels.push(create_tab_panel(lbl, &cur_body));
                cur_body.clear();
            }
        } else if cur_label.is_some() {
            cur_body.push_str(line);
            cur_body.push('\n');
        }
    }

    if let Some(lbl) = cur_label {
        panels.push(create_tab_panel(lbl, &cur_body));
    }

    panels
}

fn create_tab_panel(label: String, body: &str) -> TabPanelDef {
    let slug = slugify(&label);
    let trimmed_body = body.trim();

    let content = if trimmed_body.starts_with("```") {
        // Parse inner code block
        let lines: Vec<&str> = trimmed_body.lines().collect();
        let lang = lines[0].trim_start_matches('`').trim();
        let code_lines = if lines.len() > 2 {
            lines[1..lines.len() - 1].join("\n")
        } else if lines.len() == 2 {
            lines[1].trim_end_matches('`').to_string()
        } else {
            String::new()
        };
        let highlighted = highlight_code(&code_lines, if lang.is_empty() { None } else { Some(lang) });
        code_block_highlighted(Some(&highlighted), &code_lines, if lang.is_empty() { None } else { Some(lang) }, None)
    } else {
        div()
            .class("p-5 rounded-2xl bg-slate-900/60 border border-slate-800/80 text-slate-200 text-sm leading-relaxed whitespace-pre-wrap")
            .text(trimmed_body)
    };

    TabPanelDef::new(slug, label, content)
}
