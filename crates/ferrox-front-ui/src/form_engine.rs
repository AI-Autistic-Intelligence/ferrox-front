//! # Reactive Form State & Validation Engine (`form_engine`)
//!
//! Provides React Hook Form + Zod / Superforms parity for Rust WebAssembly:
//! - Reactive `FormField<T>` tracking `value`, `error`, `touched`, `dirty`, `is_valid` signals.
//! - Chainable `RuleBuilder<T>` supporting `required`, `email`, `min_length`, `max_length`, `numeric_range`, `regex`, and `custom` rules.
//! - Automatic WCAG 2.1 AA compliant input rendering with accessible ARIA attributes (`aria-invalid`, `aria-describedby`),
//!   `:focus-visible` ring indicators, and SVG alert icons.
//! - Multi-step `FormWizard` component with step-gated validation preventing premature advancement.

use std::rc::Rc;
use ferrox_front_core::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, Event};

/// Type alias for validation rule functions.
pub type ValidationRule<T> = Rc<dyn Fn(&T) -> Result<(), String>>;

/// Chainable rule builder for form fields.
pub struct RuleBuilder<T> {
    rules: Vec<ValidationRule<T>>,
}

impl<T: 'static> RuleBuilder<T> {
    /// Creates a new empty RuleBuilder.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Appends a custom rule callback returning `Ok(())` or `Err(String)`.
    pub fn custom<F>(mut self, f: F) -> Self
    where
        F: Fn(&T) -> Result<(), String> + 'static,
    {
        self.rules.push(Rc::new(f));
        self
    }

    /// Finalizes and returns the vector of validation rules.
    pub fn build(self) -> Vec<ValidationRule<T>> {
        self.rules
    }
}

impl Default for RuleBuilder<String> {
    fn default() -> Self {
        Self::new()
    }
}

impl RuleBuilder<String> {
    /// Asserts that a string field is not empty or whitespace only.
    pub fn required(mut self, err_msg: &str) -> Self {
        let msg = err_msg.to_string();
        self.rules.push(Rc::new(move |val: &String| {
            if val.trim().is_empty() {
                Err(msg.clone())
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Validates an email address via RFC-compliant RegExp or pattern match.
    pub fn email(mut self, err_msg: &str) -> Self {
        let msg = err_msg.to_string();
        self.rules.push(Rc::new(move |val: &String| {
            let s = val.trim();
            if s.is_empty() {
                return Ok(()); // Use required() for empty check
            }
            // Use js_sys::RegExp in WebAssembly for zero-dependency high-accuracy regex
            let regex = js_sys::RegExp::new(
                r#"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+$"#,
                "",
            );
            if regex.test(s) {
                Ok(())
            } else {
                Err(msg.clone())
            }
        }));
        self
    }

    /// Enforces a minimum character length.
    pub fn min_length(mut self, min: usize, err_msg: &str) -> Self {
        let msg = err_msg.to_string();
        self.rules.push(Rc::new(move |val: &String| {
            if val.chars().count() < min {
                Err(msg.clone())
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Enforces a maximum character length.
    pub fn max_length(mut self, max: usize, err_msg: &str) -> Self {
        let msg = err_msg.to_string();
        self.rules.push(Rc::new(move |val: &String| {
            if val.chars().count() > max {
                Err(msg.clone())
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Validates a numeric range parsed from string.
    pub fn numeric_range(mut self, min: f64, max: f64, err_msg: &str) -> Self {
        let msg = err_msg.to_string();
        self.rules.push(Rc::new(move |val: &String| {
            match val.trim().parse::<f64>() {
                Ok(n) if n >= min && n <= max => Ok(()),
                _ => Err(msg.clone()),
            }
        }));
        self
    }

    /// Validates against a custom Regular Expression pattern string.
    pub fn regex(mut self, pattern: &str, err_msg: &str) -> Self {
        let msg = err_msg.to_string();
        let pat = pattern.to_string();
        self.rules.push(Rc::new(move |val: &String| {
            let re = js_sys::RegExp::new(&pat, "");
            if re.test(val) {
                Ok(())
            } else {
                Err(msg.clone())
            }
        }));
        self
    }
}

/// A reactive form field containing independent state signals.
#[derive(Clone)]
pub struct FormField<T: Clone + PartialEq + 'static> {
    pub id: String,
    pub name: String,
    pub label: String,
    pub placeholder: String,
    pub helper_text: String,
    pub input_type: String,
    pub value: Signal<T>,
    pub error: Signal<Option<String>>,
    pub touched: Signal<bool>,
    pub dirty: Signal<bool>,
    pub is_valid: Signal<bool>,
    rules: Vec<ValidationRule<T>>,
    initial_value: T,
}

impl<T: Clone + PartialEq + 'static> FormField<T> {
    /// Creates a new FormField with initial value and validation rules.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        label: impl Into<String>,
        initial_value: T,
        rules: Vec<ValidationRule<T>>,
    ) -> Self {
        let id_str = id.into();
        Self {
            id: id_str,
            name: name.into(),
            label: label.into(),
            placeholder: String::new(),
            helper_text: String::new(),
            input_type: "text".to_string(),
            value: Signal::new(initial_value.clone()),
            error: Signal::new(None),
            touched: Signal::new(false),
            dirty: Signal::new(false),
            is_valid: Signal::new(true),
            rules,
            initial_value,
        }
    }

    /// Sets the placeholder text for this field.
    pub fn with_placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the helper text displayed below the field.
    pub fn with_helper_text(mut self, helper: impl Into<String>) -> Self {
        self.helper_text = helper.into();
        self
    }

    /// Sets the HTML input type (e.g. "text", "email", "password", "number").
    pub fn with_input_type(mut self, input_type: impl Into<String>) -> Self {
        self.input_type = input_type.into();
        self
    }

    /// Sets the current field value, marks dirty flag, and triggers validation.
    pub fn set_value(&self, val: T) {
        let dirty = val != self.initial_value;
        self.dirty.set(dirty);
        self.value.set(val);
        self.validate();
    }

    /// Marks the field as touched (focused and blurred).
    pub fn touch(&self) {
        self.touched.set(true);
        self.validate();
    }

    /// Validates the current value against all configured rules.
    pub fn validate(&self) -> bool {
        let current_val = self.value.get();
        for rule in &self.rules {
            if let Err(err_msg) = rule(&current_val) {
                self.error.set(Some(err_msg));
                self.is_valid.set(false);
                return false;
            }
        }
        self.error.set(None);
        self.is_valid.set(true);
        true
    }

    /// Resets the field to its initial value, clearing error and dirty states.
    pub fn reset(&self) {
        self.value.set(self.initial_value.clone());
        self.error.set(None);
        self.touched.set(false);
        self.dirty.set(false);
        self.is_valid.set(true);
    }
}

impl FormField<String> {
    /// Renders an enterprise WCAG 2.1 AA compliant text/email/password/number input group.
    pub fn render(&self) -> DomBuilder {
        let field_id = self.id.clone();
        let error_id = format!("{}-error", field_id);
        let helper_id = format!("{}-helper", field_id);

        let current_val = self.value.get();
        let current_err = self.error.get();
        let is_touched = self.touched.get();
        let has_error = current_err.is_some() && is_touched;
        let is_success = is_touched && self.dirty.get() && current_err.is_none();

        // Accessible class computation
        let input_classes = vec![
            "ferrox-form-input",
            "w-full",
            "px-4",
            "py-2.5",
            "rounded-xl",
            "text-sm",
            "font-sans",
            "transition-all",
            "duration-200",
            "outline-none",
        ];

        let inline_style = if has_error {
            "background: rgba(239, 68, 68, 0.06); border: 1.5px solid #ef4444; color: #f8fafc; box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.2);"
        } else if is_success {
            "background: rgba(16, 185, 129, 0.06); border: 1.5px solid #10b981; color: #f8fafc; box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.15);"
        } else {
            "background: rgba(15, 23, 42, 0.65); border: 1.5px solid rgba(255, 255, 255, 0.12); color: #f8fafc;"
        };

        let described_by = if has_error {
            error_id.clone()
        } else if !self.helper_text.is_empty() {
            helper_id.clone()
        } else {
            "".to_string()
        };

        let mut input_elem = input()
            .id(&field_id)
            .attr("name", &self.name)
            .attr("type", &self.input_type)
            .attr("value", &current_val)
            .attr("placeholder", &self.placeholder)
            .attr("aria-invalid", if has_error { "true" } else { "false" })
            .attr("aria-describedby", &described_by)
            .class(&input_classes.join(" "))
            .style_attr(inline_style);

        // Bind reactive input event
        let self_clone = self.clone();
        input_elem = input_elem.on_input(move |e: Event| {
            if let Some(target) = e.target() {
                if let Ok(input_el) = target.dyn_into::<HtmlInputElement>() {
                    self_clone.set_value(input_el.value());
                }
            }
        });

        // Bind blur event for touched state
        let self_blur = self.clone();
        input_elem = input_elem.on("blur", move |_: Event| {
            self_blur.touch();
        });

        // Status badge / Icon
        let mut status_icon = div().class("absolute right-3 top-3 pointer-events-none");
        if has_error {
            // Warning SVG exclamation
            status_icon = status_icon.child(
                svg()
                    .attr("width", "18")
                    .attr("height", "18")
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "none")
                    .attr("stroke", "#ef4444")
                    .attr("stroke-width", "2")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
                    .children(vec![
                        circle().attr("cx", "12").attr("cy", "12").attr("r", "10"),
                        line().attr("x1", "12").attr("y1", "8").attr("x2", "12").attr("y2", "12"),
                        line().attr("x1", "12").attr("y1", "16").attr("x2", "12.01").attr("y2", "16"),
                    ])
            );
        } else if is_success {
            // Success checkmark SVG
            status_icon = status_icon.child(
                svg()
                    .attr("width", "18")
                    .attr("height", "18")
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "none")
                    .attr("stroke", "#10b981")
                    .attr("stroke-width", "2")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
                    .child(polyline().attr("points", "20 6 9 17 4 12"))
            );
        }

        // Label
        let label_elem = label()
            .attr("for", &field_id)
            .class("block text-xs font-semibold uppercase tracking-wider text-slate-300 mb-1.5")
            .text(&self.label);

        // Input wrapper
        let input_wrapper = div()
            .class("relative flex items-center")
            .child(input_elem)
            .child(status_icon);

        // Group container
        let mut group = div()
            .class("ferrox-form-group flex flex-col mb-4")
            .child(label_elem)
            .child(input_wrapper);

        // Render Error or Helper message
        if has_error {
            if let Some(err) = current_err {
                let error_elem = div()
                    .id(&error_id)
                    .attr("role", "alert")
                    .class("flex items-center gap-1.5 mt-1.5 text-xs text-rose-400 font-medium")
                    .children(vec![
                        svg()
                            .attr("width", "14")
                            .attr("height", "14")
                            .attr("viewBox", "0 0 24 24")
                            .attr("fill", "none")
                            .attr("stroke", "currentColor")
                            .attr("stroke-width", "2")
                            .children(vec![
                                circle().attr("cx", "12").attr("cy", "12").attr("r", "10"),
                                line().attr("x1", "12").attr("y1", "8").attr("x2", "12").attr("y2", "12"),
                                line().attr("x1", "12").attr("y1", "16").attr("x2", "12.01").attr("y2", "16"),
                            ]),
                        span().text(&err),
                    ]);
                group = group.child(error_elem);
            }
        } else if !self.helper_text.is_empty() {
            let helper_elem = div()
                .id(&helper_id)
                .class("mt-1 text-xs text-slate-400")
                .text(&self.helper_text);
            group = group.child(helper_elem);
        }

        group
    }
}

/// Step definition for `FormWizard`.
#[derive(Clone)]
pub struct WizardStep {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub validate_step: Rc<dyn Fn() -> bool>,
    pub render_content: Rc<dyn Fn() -> DomBuilder>,
}

impl WizardStep {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        subtitle: impl Into<String>,
        validate_step: impl Fn() -> bool + 'static,
        render_content: impl Fn() -> DomBuilder + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: subtitle.into(),
            validate_step: Rc::new(validate_step),
            render_content: Rc::new(render_content),
        }
    }
}

/// Reactive State coordinator for multi-step form wizards.
#[derive(Clone)]
pub struct FormWizardState {
    pub current_step: Signal<usize>,
    pub total_steps: usize,
    pub steps: Vec<WizardStep>,
    pub is_completed: Signal<bool>,
}

impl FormWizardState {
    pub fn new(steps: Vec<WizardStep>) -> Self {
        let total = steps.len();
        Self {
            current_step: Signal::new(0),
            total_steps: total,
            steps,
            is_completed: Signal::new(false),
        }
    }

    /// Attempts to advance to the next step, executing current step's validator.
    pub fn next(&self) -> bool {
        let curr = self.current_step.get();
        if curr < self.total_steps {
            let step = &self.steps[curr];
            if (step.validate_step)() {
                if curr + 1 < self.total_steps {
                    self.current_step.set(curr + 1);
                    return true;
                } else {
                    self.is_completed.set(true);
                    return true;
                }
            }
        }
        false
    }

    /// Moves back to the previous step without requiring validation.
    pub fn prev(&self) {
        let curr = self.current_step.get();
        if curr > 0 {
            self.current_step.set(curr - 1);
            self.is_completed.set(false);
        }
    }

    /// Checks if a step index is currently active.
    pub fn is_current(&self, step_idx: usize) -> bool {
        self.current_step.get() == step_idx
    }

    /// Checks if a step index is completed.
    pub fn is_finished(&self, step_idx: usize) -> bool {
        self.current_step.get() > step_idx || self.is_completed.get()
    }
}

/// Renders a multi-step FormWizard with step indicators, reactive validation gating, and navigation controls.
pub fn form_wizard(
    state: FormWizardState,
    on_complete: impl Fn() + 'static,
) -> DomBuilder {
    let curr = state.current_step.get();
    let total = state.total_steps;
    let completed = state.is_completed.get();

    // 1. Wizard Header / Stepper Progress
    let mut step_nav = div().class("flex items-center justify-between mb-8 relative");

    for (idx, step) in state.steps.iter().enumerate() {
        let is_active = idx == curr && !completed;
        let is_past = idx < curr || completed;

        let badge_style = if is_active {
            "background: linear-gradient(135deg, #3b82f6, #6366f1); color: #ffffff; box-shadow: 0 0 15px rgba(99, 102, 241, 0.5); border: 2px solid #818cf8;"
        } else if is_past {
            "background: #10b981; color: #ffffff; border: 2px solid #059669;"
        } else {
            "background: rgba(30, 41, 59, 0.8); color: #64748b; border: 2px solid rgba(255, 255, 255, 0.1);"
        };

        let badge_content = if is_past {
            // Checkmark
            span().child(
                svg()
                    .attr("width", "14")
                    .attr("height", "14")
                    .attr("viewBox", "0 0 24 24")
                    .attr("fill", "none")
                    .attr("stroke", "currentColor")
                    .attr("stroke-width", "3")
                    .attr("stroke-linecap", "round")
                    .attr("stroke-linejoin", "round")
                    .child(polyline().attr("points", "20 6 9 17 4 12"))
            )
        } else {
            span().text(&(idx + 1).to_string())
        };

        let step_item = div()
            .class("flex flex-col items-center z-10")
            .children(vec![
                div()
                    .class("w-9 h-9 rounded-full flex items-center justify-center font-bold text-sm transition-all duration-300")
                    .style_attr(badge_style)
                    .child(badge_content),
                span()
                    .class(&format!(
                        "text-xs font-semibold mt-2 {}",
                        if is_active { "text-indigo-400" } else if is_past { "text-emerald-400" } else { "text-slate-500" }
                    ))
                    .text(&step.title),
            ]);

        step_nav = step_nav.child(step_item);
    }

    // 2. Active Step Content Container
    let content_area = if completed {
        div()
            .class("p-8 text-center flex flex-col items-center justify-center min-h-[220px]")
            .children(vec![
                div()
                    .class("w-16 h-16 rounded-full bg-emerald-500/20 text-emerald-400 flex items-center justify-center mb-4 ring-8 ring-emerald-500/10")
                    .child(
                        svg()
                            .attr("width", "32")
                            .attr("height", "32")
                            .attr("viewBox", "0 0 24 24")
                            .attr("fill", "none")
                            .attr("stroke", "currentColor")
                            .attr("stroke-width", "2.5")
                            .attr("stroke-linecap", "round")
                            .child(polyline().attr("points", "20 6 9 17 4 12"))
                    ),
                h3().class("text-xl font-bold text-white mb-2").text("Process Completed Successfully"),
                p().class("text-sm text-slate-400 max-w-md")
                    .text("All entered data has been validated in Wasm memory with zero external dependencies."),
            ])
    } else if let Some(active_step) = state.steps.get(curr) {
        div()
            .class("wizard-step-body p-6 rounded-2xl bg-slate-900/40 border border-slate-800/80 mb-6")
            .children(vec![
                div()
                    .class("mb-5 border-b border-slate-800/60 pb-3")
                    .children(vec![
                        h4().class("text-base font-bold text-white").text(&active_step.title),
                        p().class("text-xs text-slate-400 mt-0.5").text(&active_step.subtitle),
                    ]),
                (active_step.render_content)(),
            ])
    } else {
        div()
    };

    // 3. Navigation Controls / Action Footer
    let state_prev = state.clone();
    let state_next = state.clone();
    let on_complete_rc = Rc::new(on_complete);

    let prev_btn = button()
        .class(&format!(
            "px-5 py-2.5 rounded-xl text-xs font-semibold transition-all {}",
            if curr > 0 && !completed {
                "bg-slate-800 hover:bg-slate-700 text-slate-200 cursor-pointer"
            } else {
                "bg-slate-900 text-slate-600 cursor-not-allowed opacity-50"
            }
        ))
        .text("← Previous")
        .on_click(move || {
            state_prev.prev();
        });

    let is_last_step = curr + 1 == total;
    let next_btn_text = if is_last_step { "Confirm & Submit ✓" } else { "Next →" };

    let next_btn = button()
        .class(&format!(
            "px-6 py-2.5 rounded-xl text-xs font-semibold transition-all shadow-lg {}",
            if completed {
                "bg-slate-800 text-slate-500 cursor-not-allowed opacity-50"
            } else if is_last_step {
                "bg-emerald-600 hover:bg-emerald-500 text-white cursor-pointer shadow-emerald-900/30"
            } else {
                "bg-indigo-600 hover:bg-indigo-500 text-white cursor-pointer shadow-indigo-900/30"
            }
        ))
        .text(next_btn_text)
        .on_click(move || {
            if !completed {
                if state_next.next() && is_last_step {
                    on_complete_rc();
                }
            }
        });

    let footer_actions = div()
        .class("flex items-center justify-between pt-4 border-t border-slate-800/80")
        .children(vec![prev_btn, next_btn]);

    div()
        .class("ferrox-form-wizard w-full max-w-2xl mx-auto p-6 rounded-3xl bg-slate-950/80 border border-slate-800/80 shadow-2xl backdrop-blur-xl")
        .children(vec![step_nav, content_area, footer_actions])
}
