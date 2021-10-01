use std::collections::HashMap;
use super::config::Config;

pub struct Source<'a> {
    pub config: Config<'a>,
    pub query: Query<'a>,
}

pub struct Query<'a> {
    pub parameters: HashMap<&'a str, String>,
}

impl Source <'_>{
    pub fn new(config_contents: &mut String) -> Source {
        let config = Config::get(config_contents);
        Source {
            config,
            query: Query {
                parameters: HashMap::new(),
            },
        }
    }
}
