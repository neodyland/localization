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

fn read_dir_get_all_files(path: &Path) -> IoResult<Vec<PathBuf>> {
    let mut files = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let mime = entry.file_type()?;
        if mime.is_file() {
            files.push(entry.path());
        }
        if mime.is_dir() {
            files.append(&mut read_dir_get_all_files(&entry.path())?);
        }
    }
    Ok(files)
}

fn json_kv_obj(m: &Map<String, Value>) -> Vec<(String, String)> {
    let mut vec = vec![];
    for (key, value) in m {
        match value {
            Value::String(value) => vec.push((key.clone(), value.clone())),
            Value::Object(m) => {
                let sub_vec = json_kv_obj(m);
                for (sub_key, sub_value) in sub_vec {
                    vec.push((format!("{}.{}", key, sub_key), sub_value));
                }
            }
            _ => {}
        }
    }
    vec
}

fn json_kv(s: &str) -> Vec<(String, String)> {
    let json: Value = from_str(s).unwrap();
    json.as_object().map(json_kv_obj).unwrap_or(vec![])
}

type H = HashMap<(String, String), String>;

fn file_key(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap()
        .with_extension("")
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn dir_to_json(path: &Path) -> H {
    let mut hash = H::new();
    let all_files = read_dir_get_all_files(path).unwrap();
    for file_path in all_files {
        let file_content = fs::read_to_string(&file_path).unwrap();
        let file_name = file_key(path, &file_path);
        let file = json_kv(&file_content);
        for (key, value) in file {
            hash.insert((file_name.clone(), key.clone()), value);
        }
    }
    hash
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

fn backhash(h: LocaleKV) -> LocaleKV {
    let mut new = LocaleKV::new();
    for (locale, map) in h {
        for (key, value) in map {
            let file_map = new.entry(key).or_default();
            file_map.insert(locale.clone(), value);
        }
    }
    new
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
        let ds = dir_to_json(&locale_path);
        let mut map = HashMap::new();
        for ((file_name, key), value) in ds {
            map.insert(format!("{}:{}", file_name, key), value);
        }
        hash.insert(locale, map);
    }
    backhash(hash)
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
