#![doc = include_str!("../../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod load;
mod t;

use std::collections::HashMap;

use load::{default_locale, get_locale};
use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::quote;
use t::{RawTokenStream, parse_t};

fn hashmap_to_tokens(
    h: &HashMap<String, String>,
    default_locale: &str,
) -> (TokenStream, Vec<TokenStream>, usize) {
    let mut locales = Vec::new();
    let mut values = Vec::new();
    let mut default_index = 0;
    for (i, (key, value)) in h.iter().enumerate() {
        if key == default_locale {
            default_index = i + 1;
        }
        let key = Literal::string(key);
        locales.push(quote! {
            #key => #i,
        });
        let value = Literal::string(value);
        values.push(value);
    }
    if default_index == 0 {
        panic!("Default locale not found");
    }
    (
        quote! {
            let values = [
                #(#values),*
            ];
        },
        locales,
        default_index - 1,
    )
}

fn append(l: Literal) -> Literal {
    let s = l.to_string();
    let mut s = s[1..s.len() - 1].to_string();
    s.insert(0, '{');
    s.insert(0, '{');
    s.insert(s.len(), '}');
    s.insert(s.len(), '}');
    Literal::string(&s)
}

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

fn replacement_to_tokens(r: &[(TokenStream, Option<TokenStream>)]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for (key, value) in r {
        let value = value.as_ref().map_or(key, |value| value);
        let key = append(into_literal(key));
        tokens.extend(quote! {
            value = value.replace(
                #key,
                &format!("{}", #value)
            );
        });
    }
    tokens
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
    let replacement = replacement_to_tokens(&replacement);
    let (values, names, default_index) = hashmap_to_tokens(map, &default_locale());
    quote!(
        {
            #values;
            let mut value = values[match format!("{}", #locale).as_str() {
                #(#names)*
                _ => #default_index,
            }].to_string();
            #replacement
            value
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
