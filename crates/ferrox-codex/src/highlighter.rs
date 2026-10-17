//! # Syntax Highlighter Engine (`ferrox-codex::highlighter`)
//!
//! Ultra-fast, zero-dependency, pure Rust WebAssembly syntax tokenizer for code blocks.
//! Supports: Rust, Python, Bash/Shell, TypeScript, JavaScript, JSON, YAML, HTML, CSS, SQL, PHP, Java.

/// Token classifications for code highlighting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Keyword,
    TypeName,
    StringLit,
    Comment,
    NumberLit,
    Function,
    Operator,
    Punctuation,
    Plain,
}

impl TokenType {
    pub fn css_class(&self) -> &'static str {
        match self {
            Self::Keyword => "text-purple-400 font-semibold",
            Self::TypeName => "text-sky-300 font-medium",
            Self::StringLit => "text-lime-300",
            Self::Comment => "text-slate-500 italic",
            Self::NumberLit => "text-amber-400",
            Self::Function => "text-blue-400",
            Self::Operator => "text-pink-400",
            Self::Punctuation => "text-slate-400",
            Self::Plain => "text-slate-200",
        }
    }
}

pub struct Token<'a> {
    pub text: &'a str,
    pub token_type: TokenType,
}

/// Tokenizes and highlights code into HTML spans with Ferrox Design System CSS classes.
pub fn highlight_code(code: &str, lang: Option<&str>) -> String {
    let language = lang.unwrap_or("text").trim().to_lowercase();
    let mut html = String::with_capacity(code.len() * 3);

    for line in code.lines() {
        highlight_line(line, &language, &mut html);
        html.push('\n');
    }

    // Trim trailing newline if original didn't end with one
    if !code.ends_with('\n') && html.ends_with('\n') {
        html.pop();
    }

    html
}

fn highlight_line(line: &str, lang: &str, out: &mut String) {
    let trimmed = line.trim_start();
    
    // Check line comment
    if (lang == "rust" || lang == "ts" || lang == "js" || lang == "java" || lang == "c" || lang == "cpp") && trimmed.starts_with("//") {
        let leading_spaces = &line[..line.len() - trimmed.len()];
        escape_html(leading_spaces, out);
        out.push_str(r#"<span class="text-slate-500 italic">"#);
        escape_html(trimmed, out);
        out.push_str("</span>");
        return;
    }

    if (lang == "py" || lang == "python" || lang == "bash" || lang == "sh" || lang == "yaml" || lang == "yml") && trimmed.starts_with('#') {
        let leading_spaces = &line[..line.len() - trimmed.len()];
        escape_html(leading_spaces, out);
        out.push_str(r#"<span class="text-slate-500 italic">"#);
        escape_html(trimmed, out);
        out.push_str("</span>");
        return;
    }

    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        // 1. Whitespace
        if c.is_whitespace() {
            out.push(c);
            i += 1;
            continue;
        }

        // 2. Line Comment mid-line
        if (lang == "rust" || lang == "ts" || lang == "js" || lang == "java") && c == '/' && i + 1 < len && chars[i + 1] == '/' {
            let rest: String = chars[i..].iter().collect();
            out.push_str(r#"<span class="text-slate-500 italic">"#);
            escape_html(&rest, out);
            out.push_str("</span>");
            break;
        }

        if (lang == "py" || lang == "python" || lang == "bash" || lang == "sh" || lang == "yaml" || lang == "yml") && c == '#' {
            let rest: String = chars[i..].iter().collect();
            out.push_str(r#"<span class="text-slate-500 italic">"#);
            escape_html(&rest, out);
            out.push_str("</span>");
            break;
        }

        // 3. String Literals ("...", '...', `...`)
        if c == '"' || c == '\'' || c == '`' {
            let quote = c;
            let start = i;
            i += 1;
            while i < len {
                if chars[i] == '\\' && i + 1 < len {
                    i += 2;
                } else if chars[i] == quote {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            let str_val: String = chars[start..i].iter().collect();
            out.push_str(r#"<span class="text-lime-300">"#);
            escape_html(&str_val, out);
            out.push_str("</span>");
            continue;
        }

        // 4. Numbers
        if c.is_ascii_digit() && (i == 0 || !chars[i - 1].is_alphanumeric() && chars[i - 1] != '_') {
            let start = i;
            while i < len && (chars[i].is_ascii_alphanumeric() || chars[i] == '.' || chars[i] == '_') {
                i += 1;
            }
            let num_val: String = chars[start..i].iter().collect();
            out.push_str(r#"<span class="text-amber-400 font-mono">"#);
            escape_html(&num_val, out);
            out.push_str("</span>");
            continue;
        }

        // 5. Identifiers, Keywords & Types
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let is_next_paren = i < len && chars[i] == '(';

            if is_keyword(&word, lang) {
                out.push_str(r#"<span class="text-purple-400 font-semibold">"#);
                escape_html(&word, out);
                out.push_str("</span>");
            } else if is_type_name(&word) {
                out.push_str(r#"<span class="text-sky-300 font-medium">"#);
                escape_html(&word, out);
                out.push_str("</span>");
            } else if is_next_paren {
                out.push_str(r#"<span class="text-blue-400">"#);
                escape_html(&word, out);
                out.push_str("</span>");
            } else {
                escape_html(&word, out);
            }
            continue;
        }

        // 6. Operators & Punctuation
        if "->=!<>&|+-*/%^~?".contains(c) {
            out.push_str(r#"<span class="text-pink-400 font-medium">"#);
            escape_char(c, out);
            out.push_str("</span>");
            i += 1;
            continue;
        }

        // 7. Fallback char
        escape_char(c, out);
        i += 1;
    }
}

fn is_keyword(word: &str, lang: &str) -> bool {
    match lang {
        "rust" => matches!(
            word,
            "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern"
                | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match"
                | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self"
                | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe"
                | "use" | "where" | "while" | "async" | "await" | "dyn"
        ),
        "py" | "python" => matches!(
            word,
            "and" | "as" | "assert" | "async" | "await" | "break" | "class" | "continue"
                | "def" | "del" | "elif" | "else" | "except" | "False" | "finally" | "for"
                | "from" | "global" | "if" | "import" | "in" | "is" | "lambda" | "None"
                | "nonlocal" | "not" | "or" | "pass" | "raise" | "return" | "True" | "try"
                | "while" | "with" | "yield"
        ),
        "ts" | "typescript" | "js" | "javascript" => matches!(
            word,
            "break" | "case" | "catch" | "class" | "const" | "continue" | "debugger"
                | "default" | "delete" | "do" | "else" | "export" | "extends" | "finally"
                | "for" | "function" | "if" | "import" | "in" | "instanceof" | "new"
                | "return" | "super" | "switch" | "this" | "throw" | "try" | "typeof"
                | "var" | "void" | "while" | "with" | "yield" | "let" | "static" | "async"
                | "await" | "type" | "interface" | "enum" | "from" | "of" | "true" | "false"
                | "null" | "undefined"
        ),
        "bash" | "sh" | "shell" => matches!(
            word,
            "if" | "then" | "else" | "elif" | "fi" | "case" | "esac" | "for" | "while"
                | "until" | "do" | "done" | "in" | "function" | "select" | "time" | "source"
                | "export" | "echo" | "set" | "cd" | "cargo" | "npm" | "pip" | "trunk"
        ),
        "sql" => matches!(
            word,
            "SELECT" | "select" | "FROM" | "from" | "WHERE" | "where" | "INSERT" | "insert"
                | "UPDATE" | "update" | "DELETE" | "delete" | "JOIN" | "join" | "INNER" | "inner"
                | "LEFT" | "left" | "RIGHT" | "right" | "ORDER" | "order" | "BY" | "by"
                | "GROUP" | "group" | "HAVING" | "having" | "LIMIT" | "limit" | "AND" | "and"
                | "OR" | "or" | "NOT" | "not" | "AS" | "as" | "CREATE" | "create" | "TABLE" | "table"
        ),
        _ => matches!(
            word,
            "fn" | "let" | "pub" | "struct" | "enum" | "impl" | "use" | "return" | "if"
                | "else" | "match" | "for" | "while" | "async" | "await" | "def" | "class"
                | "import" | "function" | "const" | "export"
        ),
    }
}

fn is_type_name(word: &str) -> bool {
    // Uppercase first letter and not all caps
    let mut chars = word.chars();
    if let Some(first) = chars.next() {
        if first.is_uppercase() && !word.chars().all(|c| c.is_uppercase()) {
            return true;
        }
    }

    matches!(
        word,
        "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
            | "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
            | "f32" | "f64" | "bool" | "char" | "str" | "String"
            | "Option" | "Result" | "Vec" | "HashMap" | "HashSet"
            | "Box" | "Rc" | "Arc" | "RefCell" | "Mutex"
            | "int" | "float" | "boolean" | "string" | "number" | "any" | "void"
    )
}

fn escape_html(s: &str, out: &mut String) {
    for c in s.chars() {
        escape_char(c, out);
    }
}

fn escape_char(c: char, out: &mut String) {
    match c {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        '"' => out.push_str("&quot;"),
        '\'' => out.push_str("&#39;"),
        _ => out.push(c),
    }
}
