mod load;
mod t;
use std::collections::HashMap;

use load::{default_locale, get_locale};
use proc_macro2::{Literal, TokenStream, TokenTree};
use quote::quote;
use t::{parse_t, RawTokenStream};

fn hashmap_to_tokens(h: &HashMap<String, String>) -> TokenStream {
    let mut tokens = TokenStream::new();
    tokens.extend(quote! {
        use std::collections::HashMap;
        let mut kv = HashMap::new();
    });
    for (key, value) in h {
        let key = Literal::string(&key);
        let value = Literal::string(&value);
        tokens.extend(quote! {
            kv.insert(#key, #value);
        });
    }
    tokens
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

fn replacement_to_tokens(r: &Vec<(TokenTree, Option<TokenTree>)>) -> TokenStream {
    let mut tokens = TokenStream::new();
    for (key, value) in r {
        if let Some(value) = value {
            let key = match key {
                TokenTree::Ident(l) => Literal::string(&l.to_string()),
                TokenTree::Literal(l) => l.clone(),
                _ => panic!("Invalid replacement key"),
            };
            let key = append(key);
            tokens.extend(quote! {
                value = value.replace(#key, #value);
            });
        } else {
            let key_str = match key {
                TokenTree::Ident(l) => Literal::string(&l.to_string()),
                _ => panic!("Invalid replacement key"),
            };
            let key_str = append(key_str);
            tokens.extend(quote! {
                value = value.replace(
                    #key_str,
                    &format!("{}", #key)
                );
            });
        }
    }
    tokens
}

/// Use the localization thing
/// # Example
/// ```
/// use localization::t;
/// fn main() {
///   let name = "John";
///   let age = 42;
///   let s = t!("ja-JP","default:hello", name, age);
///   println!("{}", s);
/// }
/// ```
#[proc_macro]
pub fn t(item: RawTokenStream) -> RawTokenStream {
    let (locale, key, replacement) = parse_t(item);
    let map = match get_locale().get(&key) {
        Some(map) => map,
        None => panic!("Key not found: {}", key),
    };
    let default = match map.get(&default_locale()) {
        Some(default) => default,
        None => panic!("Default locale text not found: {} {}", locale, key),
    };
    let default = Literal::string(default);
    let replacement = replacement_to_tokens(&replacement);
    let map = hashmap_to_tokens(map);
    quote!(
        {
            #map;
            let value = kv.get(#locale).cloned();
            let mut value = value.unwrap_or(#default).to_string();
            #replacement
            value
        }
    )
    .into()
}
