use std::io;
use yaml_rust::{Yaml, YamlLoader};
use std::fs::OpenOptions;
use std::io::Read;
use std::collections::BTreeMap;
use std::error::Error;
use std::i32;
use std::u32;

pub trait ConfigAt<'a, T> {
    fn at(&'a self, pos: T) -> ConfigValue<'a>;
}

pub trait ConfigGet<'a, T> {
    fn get(&'a self) -> Option<T>;
}

#[derive(Debug)]
pub struct Config {
    yaml: Yaml,
}

#[derive(Debug)]
enum IndexType<'a> {
    String(&'a str),
    Int(usize),
}

#[derive(Debug)]
pub struct ConfigValue<'a> {
    index: IndexType<'a>,
    parent: Option<&'a ConfigValue<'a>>,
    root: &'a Config,
}

impl<'a> ConfigValue<'a> {
    fn get_yaml_value(&self) -> &Yaml {
        let mut v = Vec::new();
        let mut current = self;
        loop {
            v.push(&current.index);

            match current.parent {
                Some(p) => current = p,
                None => break,
            }
        }

        let mut yval = &self.root.yaml;
        for item in v.into_iter().rev() {
            match *item {
                IndexType::String(s) => yval = &yval[s],
                IndexType::Int(i) => yval = &yval[i],
            }
        }

        return yval;
    }
}

impl<'a> ConfigGet<'a, i64> for ConfigValue<'a> {
    fn get(&'a self) -> Option<i64> {
        match *self.get_yaml_value() {
            Yaml::Integer(i) => Some(i),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, u64> for ConfigValue<'a> {
    fn get(&'a self) -> Option<u64> {
        match *self.get_yaml_value() {
            Yaml::Integer(i) if i >= 0 => Some(i as u64),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, isize> for ConfigValue<'a> {
    fn get(&'a self) -> Option<isize> {
        match *self.get_yaml_value() {
            Yaml::Integer(i) if i >= isize::min_value() as i64 &&
                                i <= isize::max_value() as i64 => Some(i as isize),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, usize> for ConfigValue<'a> {
    fn get(&'a self) -> Option<usize> {
        match *self.get_yaml_value() {
            Yaml::Integer(i) if i >= usize::min_value() as i64 &&
                                i <= usize::max_value() as i64 => Some(i as usize),
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, u32> for ConfigValue<'a> {
    fn get(&'a self) -> Option<u32> {
        match *self.get_yaml_value() {
            Yaml::Integer(i) if i >= u32::min_value() as i64 && i <= u32::max_value() as i64 => {
                Some(i as u32)
            }
            _ => None,
        }
    }
}

impl<'a> ConfigGet<'a, i32> for ConfigValue<'a> {
    fn get(&'a self) -> Option<i32> {
        match *self.get_yaml_value() {
            Yaml::Integer(i) if i >= i32::min_value() as i64 && i <= i32::max_value() as i64 => {
                Some(i as i32)
            }
            _ => None,
        }
    }
}

impl<'a, T> ConfigGet<'a, Vec<T>> for ConfigValue<'a>
    where for<'b> ConfigValue<'b>: ConfigGet<'b, T>
{
    fn get(&'a self) -> Option<Vec<T>> {
        match *self.get_yaml_value() {
            Yaml::Array(ref a) => {
                let mut v: Vec<T> = Vec::new();
                for i in 0..a.len() {
                    let maybe_val = self.at(i).get();

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

impl<'a, T, U> ConfigGet<'a, (T, U)> for ConfigValue<'a>
    where for<'b> ConfigValue<'b>: ConfigGet<'b, T> + ConfigGet<'b, U>
{
    fn get(&'a self) -> Option<(T, U)> {
        match *self.get_yaml_value() {
            Yaml::Array(ref a) if a.len() == 2 => {
                let x = self.at(0).get();
                let y = self.at(1).get();
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

impl<'a, T, U, V> ConfigGet<'a, (T, U, V)> for ConfigValue<'a>
    where for<'b> ConfigValue<'b>: ConfigGet<'b, T> + ConfigGet<'b, U> + ConfigGet<'b, V>
{
    fn get(&'a self) -> Option<(T, U, V)> {
        match *self.get_yaml_value() {
            Yaml::Array(ref a) if a.len() == 3 => {
                let x = self.at(0).get();
                let y = self.at(1).get();
                let z = self.at(2).get();
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

impl<'a, S, T, U, V> ConfigGet<'a, (S, T, U, V)> for ConfigValue<'a>
    where for<'b> ConfigValue<'b>:
          ConfigGet<'b, S>
        + ConfigGet<'b, T>
        + ConfigGet<'b, U>
        + ConfigGet<'b, V>
{
    fn get(&'a self) -> Option<(S, T, U, V)> {
        match *self.get_yaml_value() {
            Yaml::Array(ref a) if a.len() == 4 => {
                let w = self.at(0).get();
                let x = self.at(1).get();
                let y = self.at(2).get();
                let z = self.at(3).get();

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

impl<'a> ConfigAt<'a, &'a str> for ConfigValue<'a> {
    fn at(&'a self, pos: &'a str) -> ConfigValue<'a> {
        ConfigValue {
            parent: Some(&self),
            root: self.root,
            index: IndexType::String(pos),
        }
    }
}

impl<'a> ConfigAt<'a, &'a str> for Config {
    fn at(&'a self, pos: &'a str) -> ConfigValue<'a> {
        ConfigValue {
            parent: None,
            root: &self,
            index: IndexType::String(pos),
        }
    }
}

impl<'a> ConfigAt<'a, usize> for ConfigValue<'a> {
    fn at(&'a self, pos: usize) -> ConfigValue<'a> {
        ConfigValue {
            parent: Some(&self),
            root: self.root,
            index: IndexType::Int(pos),
        }
    }
}

impl<'a> ConfigAt<'a, usize> for Config {
    fn at(&'a self, pos: usize) -> ConfigValue<'a> {
        ConfigValue {
            parent: None,
            root: &self,
            index: IndexType::Int(pos),
        }
    }
}

impl Config {
    pub fn from_yaml(yaml: Yaml) -> Config {
        Config { yaml: yaml }
    }

    pub fn new() -> Config {
        Config::from_yaml(Yaml::Hash(BTreeMap::new()))
    }

    pub fn from_file_or_empty(filename: &str) -> io::Result<Config> {
        match OpenOptions::new().read(true).open(filename) {
            // found file, get contents
            Ok(mut file) => {
                let mut content = String::new();
                let _ = file.read_to_string(&mut content);

                // parse yaml
                match YamlLoader::load_from_str(content.as_str()) {
                    Ok(mut yamls) => {
                        match yamls.pop() {
                            Some(yaml) => Ok(Config::from_yaml(yaml)),

                            // empty document, we'll asume null yaml structure
                            None => Ok(Config::new()),
                        }
                    }
                    Err(e) => Err(io::Error::new(io::ErrorKind::InvalidData, e.description())),
                }
            }

            // file not found, we'll asume null yaml structure
            Err(ref e) if e.kind() == io::ErrorKind::NotFound => Ok(Config::new()),

            // another kind of error
            Err(e) => Err(e),
        }
    }
}
