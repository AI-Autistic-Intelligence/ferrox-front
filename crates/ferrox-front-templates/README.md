# Ferrox Front Templates (`ferrox-front-templates`)

`ferrox-front-templates` provides ready-to-use production layouts for Ferrox Front.

## 1. What It Is & Architectural Purpose
The `ferrox-front-templates` crate is a library of pre-built, production-ready SaaS page layouts, admin dashboards, and common UI shells. Its architectural purpose is to act as a higher-level scaffolding tool, allowing developers to bootstrap complex applications instantly without having to manually compose the primitive `Container`, `Row`, and `Col` elements from scratch for standard layouts like "Sidebar + Topbar + Main Content".

## 2. Architectural Layering
This crate sits near the top of the frontend stack, directly below the user's application logic. It composes elements from `ferrox-front-ui`, `ferrox-front-router`, and `ferrox-front-security` into cohesive page structures. It provides specialized macros and wrapper components that accept children blocks, essentially serving as the "layout frame" into which developers inject their specific feature components.

## 3. How It Works Under the Hood
Templates are built using standard `rsx!` component composition. For example, the `AdminShell` component internally manages the state of the sidebar (expanded vs collapsed) using a `Signal<bool>`. It uses CSS Grid to layout the main viewport sections and provides `<Slot>` injection points for the user to pass in their navigation links, header profile widget, and main router view.

## 4. Why It Was Designed This Way
Building responsive app shells (sidebars that collapse on mobile, sticky headers, bottom navigation) is repetitive and often buggy across different viewports. By providing a curated set of templates, Ferrox Front ensures that enterprise users have immediate access to rock-solid, mobile-responsive layouts that have been rigorously tested against edge cases, saving countless hours of CSS debugging.

## 5. Practical Usage Guide & Extended Code Examples
Using the `AdminShell` template to bootstrap a dashboard application.

```rust,no_run
use ferrox_front_core::prelude::*;
use ferrox_front_templates::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        AdminShell {
            // Sidebar Navigation
            sidebar: rsx! {
                NavMenu {
                    NavItem { href: "/", icon: "home", "Dashboard" }
                    NavItem { href: "/users", icon: "users", "Users" }
                    NavItem { href: "/settings", icon: "settings", "Settings" }
                }
            },
            // Top Header
            header: rsx! {
                div { class: "flex justify-end w-full",
                    UserProfileWidget { name: "Admin" }
                }
            },
            // Main Content Area
            main: rsx! {
                AppRouter {} // Your ferrox-front-router here
            }
        }
    }
}
```

## 6. Anti-Patterns: How NOT to Use It
> [!CAUTION]
> **Anti-Pattern: Hardcoding Business Logic inside Templates**
> Do not fork or modify the source code of the templates to tightly couple them with your specific business logic or API calls. Templates should remain "dumb" UI shells. Always pass dynamic data, callbacks, and specialized logic down into the template via properties or `rsx!` child slots.

## 7. Pro-Tips & Best Practices
> [!TIP]
> **Pro-Tip: Responsive Overrides**
> The templates are designed to be mobile-first and responsive automatically. However, if you need to override the default breakpoint behavior (e.g., hiding the sidebar entirely on tablets), use the exposed `ferrox-theme` CSS variables in your global stylesheet rather than trying to inject inline styles into the template components.
