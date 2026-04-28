#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod load;
mod t;

use std::collections::HashMap;

use load::{default_locale, get_locale};
use proc_macro_crate::{FoundCrate, crate_name};
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

fn string_literal(ts: &TokenStream) -> Option<String> {
    let mut ts = ts.clone().into_iter();
    let Some(TokenTree::Literal(literal)) = ts.next() else {
        return None;
    };
    if ts.next().is_some() {
        return None;
    }
    let literal = literal.to_string();
    if literal.starts_with('"') {
        Some(literal[1..literal.len() - 1].to_string())
    } else {
        None
    }
}

fn runtime_crate() -> TokenStream {
    match crate_name("localization") {
        Ok(FoundCrate::Itself) => quote!(crate),
        Ok(FoundCrate::Name(name)) => {
            let ident = format_ident!("{name}");
            quote!(::#ident)
        }
        Err(_) => quote!(::localization),
    }
}

fn replacement_bindings(
    runtime_crate: &TokenStream,
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
            let #ident = #runtime_crate::__private::format!("{}", #value);
        });
        replacements.push((placeholder, ident));
    }
    (bindings, replacements)
}

fn template_to_tokens(
    runtime_crate: &TokenStream,
    template: &str,
    replacements: &[(String, Ident)],
) -> TokenStream {
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
            let mut value = #runtime_crate::__private::String::with_capacity(#literal_len #(+ #replacement_lens)*);
            #(#parts)*
            value
        }
    }
}

fn localized_string_to_tokens(
    runtime_crate: &TokenStream,
    locale: &TokenStream,
    literal_locale: Option<&str>,
    translations: &HashMap<String, String>,
    default_locale: &str,
    replacements: &[(String, Ident)],
) -> TokenStream {
    let default_value = translations
        .get(default_locale)
        .unwrap_or_else(|| panic!("Default locale not found"));

    if let Some(literal_locale) = literal_locale {
        let value = translations.get(literal_locale).unwrap_or(default_value);
        return template_to_tokens(runtime_crate, value, replacements);
    }

    let mut arms = Vec::new();
    let default_arm = template_to_tokens(runtime_crate, default_value, replacements);

    for (name, value) in translations {
        let name_literal = Literal::string(name);
        if name == default_locale {
            arms.push(quote! {
                #name_literal => __localization_default(),
            });
            continue;
        }
        let value = template_to_tokens(runtime_crate, value, replacements);
        arms.push(quote! {
            #name_literal => #value,
        });
    }

    quote! {
        let __localization_default = || #default_arm;
        match ::core::convert::AsRef::<str>::as_ref(&#locale) {
            #(#arms)*
            _ => __localization_default(),
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
    let runtime_crate = runtime_crate();
    let (locale, key, replacement) = parse_t(item);
    let map = get_locale().get(&key).unwrap_or_else(|| {
        panic!(
            "Key not found: {}. Available keys: {:?}",
            key,
            get_locale().keys()
        )
    });
    let (replacement_bindings, replacements) = replacement_bindings(&runtime_crate, &replacement);
    let literal_locale = string_literal(&locale);
    let localized_string = localized_string_to_tokens(
        &runtime_crate,
        &locale,
        literal_locale.as_deref(),
        map,
        default_locale(),
        &replacements,
    );
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
    let runtime_crate = runtime_crate();
    let all = load::get_locale();
    let mut all_coll = Vec::new();
    for (key, map) in all {
        let mut map_coll = Vec::new();
        let map_len = map.len();
        for (key, value) in map {
            let key = Literal::string(key);
            let value = Literal::string(value);
            map_coll.push(quote! {
                map.insert(#key, #value);
            });
        }
        let key = Literal::string(key);
        all_coll.push(quote! {
            let mut map = #runtime_crate::__private::HashMap::<&'static str, &'static str>::with_capacity(#map_len);
            #(#map_coll)*
            all.insert(#key, map);
        });
    }
    let all_len = all.len();
    quote!(
        {
            let mut all = #runtime_crate::__private::HashMap::with_capacity(#all_len);
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
