//! # Ferrox Front Macro (`ferrox-front-macro`)
//!
//! `ferrox-front-macro` provides the procedural macro `rsx!` for compiling declarative, HTML-like component markup
//! into type-safe WebAssembly DOM node builders at Rust compile time.
//!
//! ## Key Features
//! - 🚀 **`rsx!` Macro**: Write clean HTML structures directly inside Rust component functions.
//! - 🛡️ **Compile-Time Validation**: Catch unclosed tags, invalid attributes, and type mismatches during `cargo check`.
//! - ⚡ **Zero Runtime Parsing**: HTML structure is transformed into direct Wasm DOM creation calls during compilation.

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, LitStr, Token};

// A very simplified AST for RSX
enum RsxNode {
    Element(RsxElement),
    Text(LitStr),
}

struct RsxElement {
    name: Ident,
    attributes: Vec<(Ident, LitStr)>,
    children: Vec<RsxNode>,
}

impl Parse for RsxNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![<]) {
            input.parse::<Token![<]>()?;
            let name: Ident = input.parse()?;
            
            let mut attributes = Vec::new();
            while !input.peek(Token![>]) && !input.peek(Token![/]) {
                let attr_name: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let attr_value: LitStr = input.parse()?;
                attributes.push((attr_name, attr_value));
            }

            if input.peek(Token![/]) {
                // Self closing: <div />
                input.parse::<Token![/]>()?;
                input.parse::<Token![>]>()?;
                return Ok(RsxNode::Element(RsxElement { name, attributes, children: vec![] }));
            }
            
            input.parse::<Token![>]>()?;
            
            let mut children = Vec::new();
            while !input.peek(Token![<]) || !input.peek2(Token![/]) {
                children.push(input.parse()?);
            }
            
            // Closing tag: </div>
            input.parse::<Token![<]>()?;
            input.parse::<Token![/]>()?;
            let close_name: Ident = input.parse()?;
            if close_name != name {
                return Err(syn::Error::new(close_name.span(), "Mismatched closing tag"));
            }
            input.parse::<Token![>]>()?;
            
            Ok(RsxNode::Element(RsxElement { name, attributes, children }))
        } else {
            // Text node (must be string literal for this simplified version)
            let text: LitStr = input.parse()?;
            Ok(RsxNode::Text(text))
        }
    }
}

fn expand_node(node: &RsxNode) -> proc_macro2::TokenStream {
    match node {
        RsxNode::Text(lit) => {
            quote! {
                ferrox_front_core::dom::DomBuilder::text_node(#lit)
            }
        },
        RsxNode::Element(el) => {
            let name_str = el.name.to_string();
            let mut attrs = quote! {};
            for (k, v) in &el.attributes {
                let k_str = k.to_string();
                attrs = quote! { #attrs.attr(#k_str, #v) };
            }
            let mut children = quote! {};
            for child in &el.children {
                let child_expanded = expand_node(child);
                children = quote! { #children.child(#child_expanded) };
            }
            quote! {
                ferrox_front_core::dom::DomBuilder::new(#name_str)
                #attrs
                #children
            }
        }
    }
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let root = parse_macro_input!(input as RsxNode);
    let expanded = expand_node(&root);
    TokenStream::from(expanded)
}