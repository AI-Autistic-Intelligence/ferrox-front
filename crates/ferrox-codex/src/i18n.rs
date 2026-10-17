//! # Multi-Language (i18n) Support (`ferrox-codex::i18n`)
//!
//! Provides localization tokens, locale detection, language switching signals,
//! and routing resolution for: `en`, `it`, `es`, `fr`, `de`, `zh`, `ja`, `ko`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedLocale {
    En,
    It,
    Es,
    Fr,
    De,
    Zh,
    Ja,
    Ko,
}

impl SupportedLocale {
    pub const ALL: &'static [SupportedLocale] = &[
        Self::En,
        Self::It,
        Self::Es,
        Self::Fr,
        Self::De,
        Self::Zh,
        Self::Ja,
        Self::Ko,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            Self::En => "en",
            Self::It => "it",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::De => "de",
            Self::Zh => "zh",
            Self::Ja => "ja",
            Self::Ko => "ko",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::En => "English 🇺🇸",
            Self::It => "Italiano 🇮🇹",
            Self::Es => "Español 🇪🇸",
            Self::Fr => "Français 🇫🇷",
            Self::De => "Deutsch 🇩🇪",
            Self::Zh => "中文 🇨🇳",
            Self::Ja => "日本語 🇯🇵",
            Self::Ko => "한국어 🇰🇷",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.trim().to_lowercase().as_str() {
            "it" => Self::It,
            "es" => Self::Es,
            "fr" => Self::Fr,
            "de" => Self::De,
            "zh" => Self::Zh,
            "ja" => Self::Ja,
            "ko" => Self::Ko,
            _ => Self::En,
        }
    }

    /// UI strings localization
    pub fn search_placeholder(&self) -> &'static str {
        match self {
            Self::En => "Search documentation... (Ctrl+K)",
            Self::It => "Cerca nella documentazione... (Ctrl+K)",
            Self::Es => "Buscar en la documentación... (Ctrl+K)",
            Self::Fr => "Rechercher dans la documentation... (Ctrl+K)",
            Self::De => "Dokumentation durchsuchen... (Ctrl+K)",
            Self::Zh => "搜索文档... (Ctrl+K)",
            Self::Ja => "ドキュメントを検索... (Ctrl+K)",
            Self::Ko => "문서 검색... (Ctrl+K)",
        }
    }

    pub fn on_this_page(&self) -> &'static str {
        match self {
            Self::En => "ON THIS PAGE",
            Self::It => "IN QUESTA PAGINA",
            Self::Es => "EN ESTA PÁGINA",
            Self::Fr => "SUR CETTE PAGE",
            Self::De => "AUF DIESER SEITE",
            Self::Zh => "本页导读",
            Self::Ja => "このページの内容",
            Self::Ko => "이 페이지의 내용",
        }
    }

    pub fn prev_label(&self) -> &'static str {
        match self {
            Self::En => "Previous",
            Self::It => "Precedente",
            Self::Es => "Anterior",
            Self::Fr => "Précédent",
            Self::De => "Vorherige",
            Self::Zh => "上一页",
            Self::Ja => "前へ",
            Self::Ko => "이전",
        }
    }

    pub fn next_label(&self) -> &'static str {
        match self {
            Self::En => "Next",
            Self::It => "Successivo",
            Self::Es => "Siguiente",
            Self::Fr => "Suivant",
            Self::De => "Nächste",
            Self::Zh => "下一页",
            Self::Ja => "次へ",
            Self::Ko => "다음",
        }
    }
}

/// Splits route into locale and canonical doc path.
/// E.g. `/it/overview` -> `(SupportedLocale::It, "/overview")`
/// E.g. `/overview` -> `(SupportedLocale::En, "/overview")`
pub fn parse_route_locale(route: &str) -> (SupportedLocale, String) {
    let clean = route.trim_start_matches('/');
    if let Some((first, rest)) = clean.split_once('/') {
        let loc = SupportedLocale::from_code(first);
        if loc != SupportedLocale::En || first == "en" {
            return (loc, format!("/{}", rest));
        }
    } else {
        let loc = SupportedLocale::from_code(clean);
        if loc != SupportedLocale::En || clean == "en" {
            return (loc, "/overview".to_string());
        }
    }

    (SupportedLocale::En, if route.starts_with('/') { route.to_string() } else { format!("/{}", route) })
}
