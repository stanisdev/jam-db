use std::fs::File;
use serde_derive::Deserialize;
use std::io::prelude::*;

#[derive(Deserialize)]
pub struct Config<'a> {
    #[serde(borrow)]
    pub area: Area<'a>,
    pub types: Types<'a>,
}

#[derive(Deserialize)]
pub struct Area<'a> {
    #[serde(borrow)]
    pub options: [&'a str; 3],
}

#[derive(Deserialize)]
pub struct Types<'a> {
    #[serde(borrow)]
    pub available: [&'a str; 3],
}

impl<'a> Config<'a> {
    pub fn get(contents: &'a mut String) -> Config<'a> {
        let mut file = File::open("src/config/config.toml") // @todo: change this
            .expect("The 'config.toml' file not found");

        file.read_to_string(contents)
            .expect("The content of the 'config.toml' file cannot be parsed");
        let config: Config = toml::from_str(contents.as_str()).unwrap();
        config
    }
}
