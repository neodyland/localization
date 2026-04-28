#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod load;
mod t;

use std::collections::HashMap;

use load::{default_locale, get_locale};
use proc_macro2::{Ident, Literal, TokenStream, TokenTree};
use quote::{format_ident, quote};
use t::{RawTokenStream, parse_t};

fn into_literal(ts: &TokenStream) -> Literal {
    let ts = ts.clone().into_iter();
    let mut s = String::new();
    for item in ts {
        match item {
            TokenTree::Literal(l) => {
                s.push_str(&t::trim_literal(l));
            }
            TokenTree::Punct(p) => {
                s.push(p.as_char());
            }
            TokenTree::Ident(i) => {
                s.push_str(&i.to_string());
            }
            TokenTree::Group(g) => {
                s.push_str(&g.to_string());
            }
        }
    }
    Literal::string(&s)
}

fn replacement_bindings(
    r: &[(TokenStream, Option<TokenStream>)],
) -> (TokenStream, Vec<(String, Ident)>) {
    let mut bindings = TokenStream::new();
    let mut replacements = Vec::new();
    for (i, (key, value)) in r.iter().enumerate() {
        let value = value.as_ref().map_or(key, |value| value);
        let name = t::trim_literal(into_literal(key));
        let placeholder = format!("{{{{{name}}}}}");
        let ident = format_ident!("__localization_replacement_{i}");
        bindings.extend(quote! {
            let #ident = ::std::format!("{}", #value);
        });
        replacements.push((placeholder, ident));
    }
    (bindings, replacements)
}

fn template_to_tokens(template: &str, replacements: &[(String, Ident)]) -> TokenStream {
    let mut pos = 0;
    let mut literal_len = 0;
    let mut parts = Vec::new();
    let mut replacement_lens = Vec::new();

    while pos < template.len() {
        let mut next = None;
        for (placeholder, ident) in replacements {
            if let Some(relative_start) = template[pos..].find(placeholder) {
                let start = pos + relative_start;
                if next
                    .as_ref()
                    .is_none_or(|(next_start, _, _): &(usize, usize, Ident)| start < *next_start)
                {
                    next = Some((start, placeholder.len(), ident.clone()));
                }
            }
        }

        let Some((start, placeholder_len, ident)) = next else {
            let literal = &template[pos..];
            literal_len += literal.len();
            let literal = Literal::string(literal);
            parts.push(quote! {
                value.push_str(#literal);
            });
            break;
        };

        if start > pos {
            let literal = &template[pos..start];
            literal_len += literal.len();
            let literal = Literal::string(literal);
            parts.push(quote! {
                value.push_str(#literal);
            });
        }
        parts.push(quote! {
            value.push_str(#ident.as_str());
        });
        replacement_lens.push(quote! {
            #ident.len()
        });
        pos = start + placeholder_len;
    }

    quote! {
        {
            let mut value = ::std::string::String::with_capacity(#literal_len #(+ #replacement_lens)*);
            #(#parts)*
            value
        }
    }
}

fn localized_string_to_tokens(
    locale: &TokenStream,
    translations: &HashMap<String, String>,
    default_locale: &str,
    replacements: &[(String, Ident)],
) -> TokenStream {
    let mut arms = Vec::new();
    let mut default_arm = None;

    for (name, value) in translations {
        let name_literal = Literal::string(name);
        let value = template_to_tokens(value, replacements);
        if name == default_locale {
            default_arm = Some(value.clone());
        }
        arms.push(quote! {
            #name_literal => #value,
        });
    }

    let default_arm = default_arm.unwrap_or_else(|| panic!("Default locale not found"));
    quote! {
        match ::core::convert::AsRef::<str>::as_ref(&#locale) {
            #(#arms)*
            _ => #default_arm,
        }
    }
}

/// Internal proc-macro entry point for `localization::t!`.
///
/// Behavior is documented on the public `localization::t!` macro.
/// This rustdoc is not intended to be consumed directly.
/// If you need to change behavior here, also update the public docs and the
/// implementation comments around this code.
#[proc_macro]
pub fn t(item: RawTokenStream) -> RawTokenStream {
    let (locale, key, replacement) = parse_t(item);
    let map = get_locale().get(&key).unwrap_or_else(|| {
        panic!(
            "Key not found: {}. Available keys: {:?}",
            key,
            get_locale().keys()
        )
    });
    let (replacement_bindings, replacements) = replacement_bindings(&replacement);
    let localized_string =
        localized_string_to_tokens(&locale, map, default_locale(), &replacements);
    quote!(
        {
            #replacement_bindings
            #localized_string
        }
    )
    .into()
}

/// Internal proc-macro entry point for `localization::all!`.
///
/// Behavior is documented on the public `localization::all!` macro.
/// This rustdoc is not intended to be consumed directly.
/// If you need to change behavior here, also update the public docs and the
/// implementation comments around this code.
#[proc_macro]
pub fn all(_item: RawTokenStream) -> RawTokenStream {
    let all = load::get_locale();
    let mut all_coll = Vec::new();
    for (key, map) in all {
        let mut map_coll = Vec::new();
        for (key, value) in map {
            let key = Literal::string(key);
            let value = Literal::string(value);
            map_coll.push(quote! {
                map.insert(#key, #value);
            });
        }
        let key = Literal::string(key);
        all_coll.push(quote! {
            let mut map = std::collections::HashMap::<&'static str, &'static str>::new();
            #(#map_coll)*
            all.insert(#key, map);
        });
    }
    quote!(
        {
            let mut all = std::collections::HashMap::new();
            #(
                #all_coll
            )*
            all
        }
    )
    .into()
}

/// Internal proc-macro entry point for `localization::loc!`.
///
/// Behavior is documented on the public `localization::loc!` macro.
/// This rustdoc is not intended to be consumed directly.
/// If you need to change behavior here, also update the public docs and the
/// implementation comments around this code.
#[proc_macro]
pub fn loc(_item: RawTokenStream) -> RawTokenStream {
    let loc = load::get_locale_list();
    let loc = loc.iter().map(|x| Literal::string(x));
    quote!(
        [
            #(
                #loc
            ),*
        ]
    )
    .into()
}
