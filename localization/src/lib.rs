#![doc = include_str!(env!("LOCALIZATION_README"))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("localization requires either the `std` or `alloc` feature");

#[doc(hidden)]
pub mod __private {
    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    pub use alloc::{format, string::String};
    #[cfg(feature = "std")]
    pub use std::{collections::HashMap, format, string::String};

    #[cfg(all(not(feature = "std"), feature = "alloc"))]
    pub use hashbrown::HashMap;
}

/// Expands to all loaded translations as a nested map.
///
/// The outer map is keyed by translation key such as `"default:hello"`.
/// Each value is another map from locale name to the localized string.
///
/// This is useful when you want to inspect or export the full translation set
/// at runtime.
/// The map is constructed each time the macro expansion runs; store the result
/// in a variable if you need to use it more than once.
///
/// ```rust,ignore
/// let all = localization::all!();
/// let hello = all["default:hello"]["en-US"];
/// ```
#[cfg(not(doc))]
pub use localization_macros::all;

/// Expands to all loaded translations as a nested map.
///
/// The outer map is keyed by translation key such as `"default:hello"`.
/// Each value is another map from locale name to the localized string.
///
/// This is useful when you want to inspect or export the full translation set
/// at runtime.
/// The map is constructed each time the macro expansion runs; store the result
/// in a variable if you need to use it more than once.
///
/// ```rust,ignore
/// let all = localization::all!();
/// let hello = all["default:hello"]["en-US"];
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! all {
    () => {};
}

/// Expands to the list of locales discovered at compile time.
///
/// The returned value is an array of locale names such as `"en-US"` and
/// `"ja-JP"`. The order is not guaranteed.
///
/// ```rust,ignore
/// let locales = localization::loc!();
/// assert!(locales.contains(&"en-US"));
/// ```
#[cfg(not(doc))]
pub use localization_macros::loc;

/// Expands to the list of locales discovered at compile time.
///
/// The returned value is an array of locale names such as `"en-US"` and
/// `"ja-JP"`. The order is not guaranteed.
///
/// ```rust,ignore
/// let locales = localization::loc!();
/// assert!(locales.contains(&"en-US"));
/// ```
#[cfg(doc)]
#[macro_export]
macro_rules! loc {
    () => {};
}

/// Expands to a localized string for the given locale and translation key.
///
/// The first argument is any expression that evaluates to the locale name,
/// such as `"en-US"` or `user.locale`. The second argument is a translation
/// key in the form `"<file>:<key>"`. Additional arguments replace placeholders
/// written as `{{name}}` in the translation string.
///
/// Positional replacements use the token text as the placeholder name:
///
/// ```rust,ignore
/// let name = "John";
/// let message = localization::t!("en-US", "default:hello", name);
/// ```
///
/// Named replacements let the placeholder name differ from the expression:
///
/// ```rust,ignore
/// let user_name = "John";
/// let age = 42;
/// let message = localization::t!("en-US", "default:hello", name = user_name, age);
/// ```
///
/// If the requested locale does not contain the key, the default locale
/// configured by `localization_build::set_default_locale` is used.
#[cfg(not(doc))]
pub use localization_macros::t;

/// Expands to a localized string for the given locale and translation key.
///
/// The first argument is any expression that evaluates to the locale name,
/// such as `"en-US"` or `user.locale`. The second argument is a translation
/// key in the form `"<file>:<key>"`. Additional arguments replace placeholders
/// written as `{{name}}` in the translation string.
///
/// Positional replacements use the token text as the placeholder name:
///
/// ```rust,ignore
/// let name = "John";
/// let message = localization::t!("en-US", "default:hello", name);
/// ```
///
/// Named replacements let the placeholder name differ from the expression:
///
/// ```rust,ignore
/// let user_name = "John";
/// let age = 42;
/// let message = localization::t!("en-US", "default:hello", name = user_name, age);
/// ```
///
/// If the requested locale does not contain the key, the default locale
/// configured by `localization_build::set_default_locale` is used.
#[cfg(doc)]
#[macro_export]
macro_rules! t {
    ($locale:expr, $key:literal $(, $name:ident)* $(,)?) => {};
    ($locale:expr, $key:literal $(, $name:ident = $value:expr)* $(,)?) => {};
    (
        $locale:expr,
        $key:literal
        $(, $name:ident)*
        $(, $named:ident = $value:expr)*
        $(,)?
    ) => {};
}
