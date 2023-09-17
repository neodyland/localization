pub use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::{token_stream::IntoIter, Literal, Punct, TokenStream, TokenTree};

fn until_comma(items: &mut IntoIter) -> Option<TokenStream> {
    let mut ts = TokenStream::new();
    while let Some(item) = items.clone().peekable().peek() {
        match item {
            TokenTree::Punct(i) => {
                if i.as_char() == ',' {
                    break;
                }
                if i.as_char() == '=' {
                    break;
                }
                ts.extend(items.next());
            }
            _ => {
                ts.extend(items.next());
            }
        }
    }
    if ts.is_empty() {
        None
    } else {
        Some(ts)
    }
}

fn next_is(items: &mut IntoIter, c: char) -> Option<Punct> {
    match items.clone().peekable().peek() {
        Some(TokenTree::Punct(punct)) => {
            if punct.as_char() == c {
                Some(match items.next() {
                    Some(TokenTree::Punct(punct)) => punct,
                    _ => panic!("expected a punct"),
                })
            } else {
                None
            }
        }
        _ => None,
    }
}

pub(crate) fn literal_trim(l: Literal) -> String {
    let mut s = l.to_string();
    if s.starts_with('\"') {
        s = s[1..s.len() - 1].to_string();
    }
    s
}

fn literal_string(items: &mut IntoIter) -> Option<String> {
    items
        .next()
        .and_then(|item| match item {
            TokenTree::Literal(item) => Some(literal_trim(item)),
            _ => None,
        })
}

pub fn parse_t(
    items: RawTokenStream,
) -> (TokenStream, String, Vec<(TokenStream, Option<TokenStream>)>) {
    let mut items = TokenStream::from(items).into_iter();
    let locale = match until_comma(&mut items) {
        Some(item) => item,
        None => panic!("this macro requires a locale for the first argument"),
    };
    let _comma = match next_is(&mut items, ',') {
        Some(i) => i,
        _ => panic!("expected a comma after the locale"),
    };
    let key = match literal_string(&mut items) {
        Some(item) => item,
        None => panic!("this macro requires a key for the first argument"),
    };
    let mut replaces = vec![];
    while items.size_hint().0 > 0 {
        let _comma = match next_is(&mut items, ',') {
            Some(i) => i,
            _ => panic!("expected a comma"),
        };
        let item = match until_comma(&mut items) {
            Some(i) => i,
            _ => panic!("expected a literal which is the name to replace"),
        };
        let has_equal = next_is(&mut items, '=').is_some();
        if !has_equal {
            if items.size_hint().0 == 0 {
                replaces.push((item, None));
                break;
            }
            replaces.push((item, None));
            continue;
        }
        let replacement = if items.clone().peekable().peek().is_some() {
            until_comma(&mut items)
        } else {
            panic!("expected a literal which is the replacement")
        };
        replaces.push((item, replacement));
    }
    (locale, key, replaces)
}
