# 🎨 Ferrox Front UI (`ferrox-front-ui`)

`ferrox-front-ui` provides an enterprise-grade **Glassmorphism Design System** for WebAssembly applications. It features a 12-column responsive layout grid, design tokens, virtualized DataGrid (1,000,000+ rows), and a suite of pre-styled Rust Wasm UI components.

---

## 🎨 Glassmorphism Design System Components

1. **Buttons (`button`)**: Supports variants `primary`, `secondary`, `outline`, `glass`, `danger`, `ghost`.
2. **Status Badges (`badge`)**: Pill badges with variants `primary`, `success`, `warning`, `danger`, `info`.
3. **Cards & Panels (`card`)**: `card()`, `card_header(title)`, `card_body()`, `card_footer()`.
4. **Notification Alerts (`alert`)**: Banner alerts for feedback (`success`, `danger`, `warning`, `info`).
5. **Form Controls (`input`, `form_group`)**: Styled input fields with cyan label groups.
6. **Progress Bars (`progress_bar`)**: Smooth percentage progress tracks (0% to 100%).
7. **User Pill & Avatar (`avatar`, `user_pill`)**: Circular avatar with initials and user info pill.
8. **Loading Spinner (`spinner`)**: Animated CSS glassmorphism spinner.
9. **Virtualized DataGrid (`virtual_data_grid`)**: High-performance table virtualization rendering 1,000,000+ rows smoothly.

---

## 🚀 Usage Examples

### 1. Buttons & Badges
```rust
use ferrox_front_ui::components::{button, badge};

let btn = button("Save Changes", "primary");
let status = badge("Active", "success");
```

### 2. Form Groups
```rust
use ferrox_front_ui::components::{form_group, input};

let email_field = form_group("Email Address", input("email", "alex@enterprise.com"));
```

### 3. User Pill & Progress Bar
```rust
use ferrox_front_ui::components::{user_pill, progress_bar};

let user = user_pill("Alexander Wright", "Admin");
let progress = progress_bar(78.5);
```

### 4. 12-Column Grid Layout
```rust
use ferrox_front_ui::layout::{container, row, col};

let layout = container()
    .child(
        row()
            .child(col("12 md-6 lg-4").child(/* Col 1 */))
            .child(col("12 md-6 lg-8").child(/* Col 2 */))
    );
```
