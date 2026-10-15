---
id: components
title: Catalogo Componenti Enterprise
sidebar_position: 3
---

# Catalogo Componenti Enterprise (`ferrox-front-ui`)

La crate `ferrox-front-ui` implementa un Design System moderno, fortemente tipizzato e conforme agli standard WCAG 2.1 AA. Tutti i componenti supportano varianti visive avanzate, micro-animazioni, stati accessibili `:focus-visible` e glassmorphism.

---

## 🎛️ Bottoni Avanzati (`button_advanced`)

I bottoni offrono un'interfaccia ricca di varianti dimensionali, stilistiche e stati di caricamento nativi:

```rust
use ferrox_front_ui::components::button_advanced;

// Bottone Primario con icona
let btn_submit = button_advanced("Invia Ordine", "primary", "md", false, Some("zap"))
    .on_click(|| println!("Ordine inviato!"));

// Bottone Glassmorphism con stato di caricamento
let btn_loading = button_advanced("Elaborazione in corso...", "glass", "sm", true, None);

// Bottone Outline Distruttivo
let btn_delete = button_advanced("Cancella", "outline", "xs", false, Some("trash"));
```

### Parametri Supportati

| Parametro | Tipo | Valori Supportati | Descrizione |
| :--- | :--- | :--- | :--- |
| `text` | `&str` | Qualsiasi stringa | Testo visualizzato nel pulsante |
| `variant` | `&str` | `"primary"`, `"outline"`, `"glass"`, `"ghost"`, `"danger"`, `"success"` | Stile visuale del pulsante |
| `size` | `&str` | `"xs"`, `"sm"`, `"md"`, `"lg"` | Padding e dimensione carattere |
| `is_loading` | `bool` | `true`, `false` | Mostra uno spinner SVG nativo disabilitando il click |
| `icon` | `Option<&str>` | Nome icona (es. `"zap"`, `"search"`, `"download"`) | Icona SVG renderizzata a sinistra del testo |

---

## 📝 Form Controls & Validazione Reattiva

### 1. Advanced Input (`input_advanced`)

L'input enterprise supporta icone prefisse/suffisse, messaggi di validità reattivi e pulsanti di clear rapido:

```rust
use ferrox_front_ui::components::input_advanced;

let price_input = input_advanced(
    "Prezzo Limite (USD)",
    "price-input",
    "number",
    "97050.00",
    "Inserisci il prezzo limite in dollari...",
    Some("dollar-sign"),
    Some("Prezzo conforme al tick size minimo di $0.50"),
    false, // is_valid
    false, // is_invalid
);
```

### 2. Validated FormField & RuleBuilder (`form_engine`)

Il motore `form_engine` permette di definire regole di validazione dichiarative che si aggiornano reattivamente ad ogni digitazione:

```rust
use ferrox_front_ui::form_engine::{FormField, RuleBuilder};

let email_field = FormField::new(
    "user-email",
    "email",
    "Indirizzo Email Aziendale",
    "".to_string(),
    RuleBuilder::new()
        .required("L'email è obbligatoria")
        .email("Inserisci un formato email valido (es. trader@ferrox.exchange)")
        .build(),
).with_placeholder("nome@azienda.com");
```

---

## 🔄 Toggle Switch e Componenti di Stato

```rust
use ferrox_front_ui::components::switch_toggle;

// Toggle per abilitazione stop-loss o modalità notturna
let stop_loss_switch = switch_toggle(
    "trailing-sl",
    "Trailing Stop-Loss Attivo",
    true, // checked
    |is_checked| {
        println!("Stato Trailing SL modificato in: {}", is_checked);
    },
);
```

---

## 🪟 Modali Dialog & Drawer Laterali

### 1. Modal Dialog

I modali di Ferrox Front sono costruiti direttamente nel DOM con overlay di backdrop blur e focus trap accessibile:

```rust
use ferrox_front_ui::components::modal_dialog;
use ferrox_front_core::dom::{p, span};

let confirmation_modal = modal_dialog(
    "Conferma Esecuzione Ordine a Mercato",
    p().text("Sei sicuro di voler acquistare 0.5 BTC a mercato al prezzo medio stimato di $97,050.00 USD?"),
    "Esegui Subito",
    "Annulla",
    || println!("Ordine confermato!"),
    || println!("Operazione annullata"),
);
```

### 2. Drawer Laterale

I drawer scivolano dal bordo destro o sinistro dello schermo con animazioni CSS hardware-accelerated, ideali per impostazioni avanzate o filtri dell'order book:

```rust
use ferrox_front_ui::components::drawer_panel;

let settings_drawer = drawer_panel(
    "Impostazioni Trading Desk",
    p().text("Configura la sensibilità del motore di matching e i filtri di volume minimo."),
    true, // is_open
    || println!("Drawer chiuso"),
);
```

---

## 🎨 Catalogo di 60+ Icone Vettoriali Pure Rust (`Icon::render`)

Ferrox Front include un catalogo nativo di icone SVG compilate direttamente all'interno del binario Rust, con zero richieste HTTP e zero file font esterni:

```rust
use ferrox_front_ui::icons::Icon;

// Rendering di icone con scala e colore personalizzato
let icon_zap = Icon::render("zap", 16, "#818cf8");
let icon_trend_up = Icon::render("trend-up", 20, "#10b981");
let icon_layers = Icon::render("layers", 18, "#38bdf8");
let icon_chart = Icon::render("chart-candlestick", 24, "#f59e0b");
```

### Categorie Principali di Icone Disponibili
- **Finanza & Trading**: `trend-up`, `trend-down`, `dollar-sign`, `credit-card`, `activity`, `chart-candlestick`, `chart-bar`, `layers`.
- **Navigazione & Azioni**: `search`, `settings`, `trash`, `edit`, `check`, `x`, `chevron-right`, `chevron-left`, `menu`, `external-link`.
- **Dati & File**: `folder`, `file-text`, `database`, `download`, `upload`, `filter`, `sort-asc`, `sort-desc`.
- **Sistema & Stato**: `cpu`, `zap`, `lock`, `shield`, `info`, `alert-circle`, `palette`, `moon`, `sun`.
