use super::config::Config;

pub struct Source<'a> {
    pub config: Config<'a>,
}

impl Source <'_>{
    pub fn new(config_contents: &mut String) -> Source {
        let config = Config::get(config_contents);
        Source {
            config,
        }
    }
}
