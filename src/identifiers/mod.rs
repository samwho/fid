mod rust;
mod toml;

use lazy_static::lazy_static;
use std::io::BufReader;
use std::{fs::File, sync::RwLock};

pub(crate) type Identifier = fn(&mut BufReader<File>) -> Option<String>;

lazy_static! {
    static ref IDENTIFIERS: RwLock<Vec<Identifier>> = RwLock::new(Vec::new());
}

pub(crate) fn register(identifier: Identifier) {
    IDENTIFIERS.write().unwrap().push(identifier);
}

pub(crate) fn all() -> Vec<Identifier> {
    IDENTIFIERS.read().unwrap().to_vec()
}

pub(crate) fn init() {
    IDENTIFIERS.write().unwrap().clear();
    register(rust::identify);
    register(toml::identify);
}
