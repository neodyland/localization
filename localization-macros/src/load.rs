use once_cell::sync::OnceCell;
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::read_dir as std_read_dir;
use std::io::Result as IoResult;
use std::path::PathBuf;

fn read_dir_dirs(path: &str) -> IoResult<Vec<OsString>> {
    let mut dirs = vec![];
    for entry in std_read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            dirs.push(entry.file_name());
        }
    }
    Ok(dirs)
}

fn read_dir_get_all_files(path: PathBuf) -> IoResult<Vec<String>> {
    let mut files = vec![];
    for entry in std_read_dir(path)? {
        let entry = entry?;
        let mime = entry.file_type()?;
        if mime.is_file() {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
        if mime.is_dir() {
            let sub_files = read_dir_get_all_files(entry.path())?;
            files.append(
                &mut sub_files
                    .iter()
                    .map(|s| format!("{}/{}", entry.file_name().to_string_lossy(), s))
                    .collect(),
            );
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

fn json_kv(s: String) -> Vec<(String, String)> {
    let json: Value = serde_json::from_str(&s).unwrap();
    json.as_object().map(json_kv_obj).unwrap_or(vec![])
}

type H = HashMap<(String, String), String>;

fn dir_to_json(path: PathBuf) -> H {
    let mut hash = H::new();
    let all = read_dir_get_all_files(path.clone()).unwrap();
    let path = path.to_string_lossy().to_string();
    for file_name in all {
        let file_content = std::fs::read_to_string(format!("{}/{}", path, file_name)).unwrap();
        let file_name = file_name
            .chars()
            .rev()
            .skip_while(|c| *c != '.')
            .skip(1)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();
        let file = json_kv(file_content.clone());
        for (key, value) in file {
            hash.insert((file_name.clone(), key.clone()), value);
        }
    }
    hash
}

type LH = HashMap<String, HashMap<String, String>>;
static LOCALE: OnceCell<LH> = OnceCell::new();

fn root() -> String {
    std::env::var("LOCALIZATION_ROOT").unwrap_or("./translations".to_string())
}

pub fn default_locale() -> String {
    std::env::var("LOCALIZATION_DEFAULT").unwrap_or("en-US".to_string())
}

fn backhash(h: LH) -> LH {
    let mut new = LH::new();
    for (locale, map) in h {
        for (key, value) in map {
            let file_map = new.entry(key).or_insert(HashMap::new());
            file_map.insert(locale.clone(), value);
        }
    }
    new
}

fn init_locale() -> LH {
    let mut hash = LH::new();
    let locales = read_dir_dirs(&root()).unwrap();
    for locale in locales {
        let locale_path = locale.clone();
        let locale = locale.to_string_lossy().to_string();
        let ds = dir_to_json(PathBuf::from(root()).join(locale_path));
        let mut map = HashMap::new();
        for ((file_name, key), value) in ds {
            map.insert(format!("{}:{}", file_name, key), value);
        }
        hash.insert(locale, map);
    }
    backhash(hash)
}

pub fn get_locale() -> &'static LH {
    LOCALE.get_or_init(init_locale)
}
