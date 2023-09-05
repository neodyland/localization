pub use proc_macro::TokenStream as RawTokenStream;
use proc_macro2::{TokenStream, TokenTree};

pub fn parse_t(items: RawTokenStream) -> (TokenTree, String, Vec<(TokenTree, Option<TokenTree>)>) {
    let mut items = TokenStream::from(items).into_iter();
    let locale = match items
        .next()
        .map(|item| match item {
            TokenTree::Literal(_) | TokenTree::Ident(_) => Some(item),
            _ => None,
        })
        .flatten()
    {
        Some(item) => item,
        None => panic!("this macro requires a locale for the first argument"),
    };
    let _comma = match items.next() {
        Some(TokenTree::Punct(punct)) => {
            if punct.as_char() == ',' {
                punct
            } else {
                panic!("expected a comma after the locale")
            }
        }
        _ => panic!("expected a comma after the locale"),
    };
    let key = match items
        .next()
        .map(|item| match item {
            TokenTree::Literal(item) => {
                let item = item.to_string();
                Some(item[1..item.len() - 1].to_string())
            }
            _ => None,
        })
        .flatten()
    {
        Some(item) => item,
        None => panic!("this macro requires a key for the first argument"),
    };
    let has_comma = match items.next() {
        Some(TokenTree::Punct(punct)) => {
            if punct.as_char() == ',' {
                true
            } else {
                false
            }
        }
        _ => false,
    };
    let replaces = if has_comma {
        let mut replaces = vec![];
        while let Some(item) = items.next() {
            let item = match item {
                TokenTree::Literal(_) | TokenTree::Ident(_) => item,
                _ => panic!("expected a literal which is the name to replace"),
            };
            let has_equal = match items.clone().peekable().peek() {
                Some(TokenTree::Punct(punct)) => {
                    if punct.as_char() == '=' {
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            };
            if !has_equal {
                if items.clone().peekable().peek().is_none() {
                    replaces.push((item, None));
                    break;
                }
                let _comma = match items.next() {
                    Some(TokenTree::Punct(punct)) => {
                        if punct.as_char() == ',' {
                            punct
                        } else {
                            panic!("expected a comma after the replacement")
                        }
                    }
                    _ => panic!("expected a comma after the replacement"),
                };
                replaces.push((item, None));
                continue;
            }
            items.next();
            let replacement = match items.next() {
                Some(item) => match item {
                    TokenTree::Literal(_) | TokenTree::Ident(_) => Some(item),
                    _ => panic!("expected a literal which is the replacement"),
                },
                _ => None,
            };
            replaces.push((item, replacement));
            if items.clone().peekable().peek().is_none() {
                break;
            }
            let _comma = match items.next() {
                Some(TokenTree::Punct(punct)) => {
                    if punct.as_char() == ',' {
                        punct
                    } else {
                        panic!("expected a comma after the replacement")
                    }
                }
                _ => panic!("expected a comma after the replacement"),
            };
        }
        replaces
    } else {
        Vec::with_capacity(0)
    };
    (locale, key, replaces)
}
