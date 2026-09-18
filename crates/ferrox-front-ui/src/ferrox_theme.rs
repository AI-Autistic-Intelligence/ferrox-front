use ferrox_front_core::dom::{div, DomBuilder};

pub enum ThemeMode {
    Cyber,
    DarkSlate,
    Corporate,
}

impl ThemeMode {
    pub fn css_vars(&self) -> &'static str {
        match self {
            ThemeMode::Cyber => "--bg-glass: rgba(10, 10, 20, 0.7); --border-glass: 1px solid rgba(0, 255, 255, 0.2); --text-primary: #e0f7fa;",
            ThemeMode::DarkSlate => "--bg-glass: rgba(30, 41, 59, 0.7); --border-glass: 1px solid rgba(148, 163, 184, 0.2); --text-primary: #f8fafc;",
            ThemeMode::Corporate => "--bg-glass: rgba(255, 255, 255, 0.8); --border-glass: 1px solid rgba(0, 0, 0, 0.1); --text-primary: #1e293b;",
        }
    }
}

pub fn ferrox_theme_provider(theme: ThemeMode, children: Vec<DomBuilder>) -> DomBuilder {
    let mut container = div()
        .attr("class", "ferrox-theme-root")
        .attr("style", &format!("min-height: 100vh; font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; {}", theme.css_vars()));
    
    for child in children {
        container = container.child(child);
    }
    
    container
}
