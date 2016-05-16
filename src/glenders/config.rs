use std::io;
use yaml_rust::{Yaml, YamlLoader};
use std::fs::OpenOptions;
use std::io::Read;
use std::collections::BTreeMap;
use std::error::Error;
use std::ops::Index;
use std::path::Path;

pub trait ConfigGet<'a, T> {
    fn get(&'a self) -> Option<T>;

    fn unwrap_or(&'a self, default: T) -> T {
        self.get().unwrap_or(default)
    }

    fn unwrap(&'a self) -> T {
        self.get().unwrap()
    }

    fn unwrap_or_else<F: FnOnce() -> T>(&'a self, f: F) -> T {
        self.get().unwrap_or_else(f)
    }

    fn is_some(&'a self) -> bool {
        self.get().is_some()
    }

    fn is_none(&'a self) -> bool {
        self.get().is_none()
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord, Hash)]
pub enum Config {
    Real(String),
    Int(i64),
    String(String),
    Bool(bool),
    Array(Vec<Config>),
    Hash(BTreeMap<Config, Config>),
    Null,
    Invalid,
}

impl Config {
    pub fn is_invalid(&self) -> bool {
        match *self {
            Config::Invalid => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Config::Null => true,
            _ => false,
        }
    }

    pub fn is_hash(&self) -> bool {
        match *self {
            Config::Hash(_) => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Config::Array(_) => true,
            _ => false,
        }
    }

    pub fn is_int(&self) -> bool {
        match *self {
            Config::Int(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match *self {
            Config::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Config::String(_) => true,
            _ => false,
        }
    }

    pub fn is_real(&self) -> bool {
        match *self {
            Config::Real(_) => true,
            _ => false,
        }
    }

    pub fn is_numeric(&self) -> bool {
        self.is_real() || self.is_int()
    }

    pub fn remove_key(&mut self, key: &str) {
        match *self {
            Config::Hash(ref mut h) => {
                h.remove(&Config::String(key.to_owned()));
            }
            _ => (),
        }
    }
}

impl<'a> ConfigGet<'a, i64> for Config {
    fn get(&'a self) -> Option<i64> {
        match *self {
            Config::Int(i) => Some(i),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, u64> for Config {
    fn get(&'a self) -> Option<u64> {
        match *self {
            Config::Int(i) if i >= 0 => Some(i as u64),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, isize> for Config {
    fn get(&'a self) -> Option<isize> {
        match *self {
            Config::Int(i) if i >= isize::min_value() as i64 && i <= isize::max_value() as i64 => {
                Some(i as isize)
            }
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, usize> for Config {
    fn get(&'a self) -> Option<usize> {
        match *self {
            Config::Int(i) if i >= 0 && i <= usize::max_value() as i64 => Some(i as usize),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, u32> for Config {
    fn get(&'a self) -> Option<u32> {
        match *self {
            Config::Int(i) if i >= 0 && i <= u32::max_value() as i64 => Some(i as u32),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, bool> for Config {
    fn get(&'a self) -> Option<bool> {
        match *self {
            Config::Bool(b) => Some(b),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, f32> for Config {
    fn get(&'a self) -> Option<f32> {
        match *self {
            Config::Real(ref r) => {
                match r.parse() {
                    Ok(f) => Some(f),
                    _ => None,
                }
            }
            Config::Int(i) => Some(i as f32),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, f64> for Config {
    fn get(&'a self) -> Option<f64> {
        match *self {
            Config::Real(ref r) => {
                match r.parse() {
                    Ok(f) => Some(f),
                    _ => None,
                }
            }
            Config::Int(i) => Some(i as f64),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, i32> for Config {
    fn get(&'a self) -> Option<i32> {
        match *self {
            Config::Int(i) if i >= i32::min_value() as i64 && i <= i32::max_value() as i64 => {
                Some(i as i32)
            }
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, String> for Config {
    fn get(&'a self) -> Option<String> {
        match *self {
            Config::String(ref s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, &'a str> for Config {
    fn get(&'a self) -> Option<&'a str> {
        match *self {
            Config::String(ref s) => Some(&s[..]),
            _ => None,
        }
    }
}

/// Retrieve a vector of arbitrairy length from the config.
/// Vectors can only be retrieved from arrays.
impl<'a, T> ConfigGet<'a, Vec<T>> for Config
    where Config: ConfigGet<'a, T>
{
    fn get(&'a self) -> Option<Vec<T>> {
        match *self {
            Config::Array(ref a) => {
                let mut v: Vec<T> = Vec::new();
                for ref elem in a {
                    let maybe_val = elem.get();

                    match maybe_val {
                        Some(val) => v.push(val),
                        None => return None,
                    }
                }
                Some(v)
            }
            _ => None,
        }
    }
}

/// Retrieve a tuple of 2 elements from the config.
/// Note that this requires the config to contain an array of values.
impl<'a, T, U> ConfigGet<'a, (T, U)> for Config
    where Config: ConfigGet<'a, T> + ConfigGet<'a, U>
{
    fn get(&'a self) -> Option<(T, U)> {
        match *self {
            Config::Array(ref a) if a.len() == 2 => {
                let x = a[0].get();
                let y = a[1].get();
                if x.is_none() || y.is_none() {
                    None
                } else {
                    Some((x.unwrap(), y.unwrap()))
                }
            }
            _ => None,
        }
    }
}

/// Retrieve a tuple of 3 elements from the config.
/// Note that this requires the config to contain an array of values.
impl<'a, T, U, V> ConfigGet<'a, (T, U, V)> for Config
    where Config: ConfigGet<'a, T> + ConfigGet<'a, U> + ConfigGet<'a, V>
{
    fn get(&'a self) -> Option<(T, U, V)> {
        match *self {
            Config::Array(ref a) if a.len() == 3 => {
                let x = a[0].get();
                let y = a[1].get();
                let z = a[2].get();
                if x.is_none() || y.is_none() || z.is_none() {
                    None
                } else {
                    Some((x.unwrap(), y.unwrap(), z.unwrap()))
                }
            }
            _ => None,
        }
    }
}

/// Retrieve a tuple of 4 elements from the config.
/// Note that this requires the config to contain an array of values.
impl<'a, S, T, U, V> ConfigGet<'a, (S, T, U, V)> for Config
    where Config: ConfigGet<'a, S> + ConfigGet<'a, T> + ConfigGet<'a, U> + ConfigGet<'a, V>
{
    fn get(&'a self) -> Option<(S, T, U, V)> {
        match *self {
            Config::Array(ref a) if a.len() == 4 => {
                let w = a[0].get();
                let x = a[1].get();
                let y = a[2].get();
                let z = a[3].get();

                if w.is_none() || x.is_none() || y.is_none() || z.is_none() {
                    None
                } else {
                    Some((w.unwrap(), x.unwrap(), y.unwrap(), z.unwrap()))
                }
            }
            _ => None,
        }
    }
}

/// Retrieve an array with a single element from the config.
impl<'a, T> ConfigGet<'a, [T; 1]> for Config
    where Config: ConfigGet<'a, T>
{
    fn get(&'a self) -> Option<[T; 1]> {
        match *self {
            Config::Array(ref a) if a.len() == 1 => {
                let x = a[0].get();
                if x.is_none() {
                    None
                } else {
                    Some([x.unwrap()])
                }
            }
            _ => None,
        }
    }
}

/// Retrieve an array of 2 elements from the config.
impl<'a, T> ConfigGet<'a, [T; 2]> for Config
    where Config: ConfigGet<'a, T>
{
    fn get(&'a self) -> Option<[T; 2]> {
        match *self {
            Config::Array(ref a) if a.len() == 2 => {
                let x = a[0].get();
                let y = a[1].get();

                if x.is_none() || y.is_none() {
                    None
                } else {
                    Some([x.unwrap(), y.unwrap()])
                }
            }
            _ => None,
        }
    }
}

/// Retrieve an array of 3 elements from the config.
impl<'a, T> ConfigGet<'a, [T; 3]> for Config
    where Config: ConfigGet<'a, T>
{
    fn get(&'a self) -> Option<[T; 3]> {
        match *self {
            Config::Array(ref a) if a.len() == 3 => {
                let x = a[0].get();
                let y = a[1].get();
                let z = a[2].get();

                if x.is_none() || y.is_none() || z.is_none() {
                    None
                } else {
                    Some([x.unwrap(), y.unwrap(), z.unwrap()])
                }
            }
            _ => None,
        }
    }
}

/// Retrieve an array of 4 elements from the config.
impl<'a, T> ConfigGet<'a, [T; 4]> for Config
    where Config: ConfigGet<'a, T>
{
    fn get(&'a self) -> Option<[T; 4]> {
        match *self {
            Config::Array(ref a) if a.len() == 4 => {
                let w = a[0].get();
                let x = a[1].get();
                let y = a[2].get();
                let z = a[3].get();

                if w.is_none() || x.is_none() || y.is_none() || z.is_none() {
                    None
                } else {
                    Some([w.unwrap(), x.unwrap(), y.unwrap(), z.unwrap()])
                }
            }
            _ => None,
        }
    }
}

/// Standard value for when a path does not exist in the config
static INVALID_VALUE: Config = Config::Invalid;

/// Allow indexing over hashmap keys
impl<'a> Index<&'a str> for Config {
    type Output = Config;

    fn index(&self, idx: &'a str) -> &Config {
        let key = Config::String(idx.to_owned());
        match *self {
            Config::Hash(ref h) => h.get(&key).unwrap_or(&INVALID_VALUE),
            _ => &INVALID_VALUE,
        }
    }
}

/// Allow indexing over arrays
impl Index<usize> for Config {
    type Output = Config;

    fn index(&self, idx: usize) -> &Config {
        match *self {
            Config::Array(ref v) => v.get(idx).unwrap_or(&INVALID_VALUE),
            _ => &INVALID_VALUE,
        }
    }
}

impl Config {
    /// Read the yaml contents into this config, consuming the yaml structure.
    pub fn from_yaml(yaml: Yaml) -> Config {
        match yaml {
            Yaml::Real(r) => Config::Real(r),
            Yaml::Integer(i) => Config::Int(i),
            Yaml::String(s) => Config::String(s),
            Yaml::Boolean(b) => Config::Bool(b),
            Yaml::Null => Config::Null,
            Yaml::BadValue => Config::Invalid,
            Yaml::Array(orig) => {
                let mut res = Vec::new();
                for val in orig {
                    res.push(Config::from_yaml(val));
                }
                Config::Array(res)
            }
            Yaml::Hash(orig) => {
                let mut res = BTreeMap::new();
                for (key, value) in orig {
                    res.insert(Config::from_yaml(key), Config::from_yaml(value));
                }
                Config::Hash(res)
            }
            Yaml::Alias(_) => panic!("Aliases in yaml file not supported"),
        }
    }

    /// Creates a new empty config.
    pub fn new() -> Config {
        Config::Null
    }

    /// Merge two configurations together into one configuration.
    /// Values inside self are overwritten by values in the other config. However, if a value
    /// is both a hash on self and the other side, then keys are merged instead.
    pub fn merge(self, other: Config) -> Config {
        match (self, other) {
            (Config::Hash(mut orig), Config::Hash(new)) => {
                for (key, mut value) in new {
                    if orig.contains_key(&key) {
                        value = orig.remove(&key).unwrap().merge(value);
                    }
                    orig.insert(key, value);
                }
                Config::Hash(orig)
            }
            (_, new) => new,
        }
    }

    /// Determine the file type of the given path and read the contents into a config.
    pub fn from_file<P: AsRef<Path>>(filename: P) -> io::Result<Config> {
        match filename.as_ref().extension().and_then(|ext| ext.to_str()) {
            Some("yml") => Config::from_yaml_file(filename),
            _ => {
                Err(io::Error::new(io::ErrorKind::InvalidInput,
                                   "Unkown filetype, supported: yml"))
            }
        }
    }
    /// Read config file on the given path as yaml.
    /// The file must exist. If the file content is empty, a config without any entries is
    /// assumed.
    pub fn from_yaml_file<P: AsRef<Path>>(filename: P) -> io::Result<Config> {
        let path = filename.as_ref().clone();
        let dir = path.parent().unwrap_or_else(|| Path::new("."));

        match OpenOptions::new().read(true).open(path) {
            // found file, get contents
            Ok(mut file) => {
                let mut content = String::new();
                let _ = file.read_to_string(&mut content);

                // parse yaml
                match YamlLoader::load_from_str(content.as_str()) {
                    Ok(mut yamls) => {
                        match yamls.pop() {
                            Some(yaml) => {
                                let mut config = Config::from_yaml(yaml);
                                if config["includes"].is_array() {
                                    let includes: Vec<String> = config["includes"]
                                                                    .unwrap_or_else(|| Vec::new());
                                    config.remove_key("includes");

                                    for include in includes {
                                        match Config::from_file(dir.join(include)) {
                                            Ok(included_config) => {
                                                config = config.merge(included_config)
                                            }
                                            Err(e) => return Err(e),
                                        }
                                    }
                                }
                                Ok(config)
                            }

                            // empty document, we'll asume null yaml structure
                            None => Ok(Config::new()),
                        }
                    }
                    Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e.description())),
                }
            }
            Err(e) => Err(e),
        }
    }
}
