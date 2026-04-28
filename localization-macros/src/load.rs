use std::{
    collections::{BTreeSet, HashMap},
    env, fs,
    io::Result as IoResult,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use serde_json::{Map, Value, from_str};

fn read_dir_dirs(path: &Path) -> IoResult<Vec<PathBuf>> {
    let mut dirs = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            dirs.push(entry.path());
        }
    }
    Ok(dirs)
}

fn visit_files(path: &Path, visitor: &mut impl FnMut(PathBuf)) -> IoResult<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let mime = entry.file_type()?;
        if mime.is_file() {
            visitor(entry.path());
        }
        if mime.is_dir() {
            visit_files(&entry.path(), visitor)?;
        }
    }
    Ok(())
}

fn visit_json_kv_obj(
    m: Map<String, Value>,
    prefix: Option<&str>,
    visitor: &mut impl FnMut(String, String),
) {
    for (key, value) in m {
        match value {
            Value::String(value) => {
                let key = match prefix {
                    Some(prefix) => format!("{prefix}.{key}"),
                    None => key,
                };
                visitor(key, value);
            }
            Value::Object(m) => {
                let key = match prefix {
                    Some(prefix) => format!("{prefix}.{key}"),
                    None => key,
                };
                visit_json_kv_obj(m, Some(&key), visitor);
            }
            _ => {}
        }
    }
}

fn visit_json_kv(s: &str, visitor: &mut impl FnMut(String, String)) {
    let json: Value = from_str(s).unwrap();
    if let Value::Object(m) = json {
        visit_json_kv_obj(m, None, visitor);
    }
}

fn file_key(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap()
        .with_extension("")
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn add_locale(path: &Path, locale: &str, hash: &mut LocaleKV) {
    visit_files(path, &mut |file_path| {
        let file_content = fs::read_to_string(&file_path).unwrap();
        let file_name = file_key(path, &file_path);
        visit_json_kv(&file_content, &mut |key, value| {
            hash.entry(format!("{file_name}:{key}"))
                .or_default()
                .insert(locale.to_string(), value);
        });
    })
    .unwrap();
}

type LocaleKV = HashMap<String, HashMap<String, String>>;
static LOCALE: LazyLock<LocaleKV> = LazyLock::new(init_locale);
static ROOT: LazyLock<PathBuf> = LazyLock::new(|| {
    env::var("LOCALIZATION_ROOT").map_or_else(|_| "./translations".into(), Into::into)
});
static DEFAULT_LOCALE: LazyLock<String> =
    LazyLock::new(|| env::var("LOCALIZATION_DEFAULT").unwrap_or_else(|_| "en-US".to_string()));

fn root() -> &'static Path {
    &ROOT
}

pub fn default_locale() -> &'static str {
    &DEFAULT_LOCALE
}

fn init_locale() -> LocaleKV {
    let mut hash = LocaleKV::new();
    let locales = read_dir_dirs(root()).unwrap();
    for locale_path in locales {
        let locale = locale_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        add_locale(&locale_path, &locale, &mut hash);
    }
    hash
}

pub fn get_locale() -> &'static LocaleKV {
    &LOCALE
}

pub fn get_locale_list() -> Vec<String> {
    let mut locs = BTreeSet::new();
    for entry in get_locale().values() {
        for loc in entry.keys() {
            locs.insert(loc.clone());
        }
    }
    locs.into_iter().collect()
}
