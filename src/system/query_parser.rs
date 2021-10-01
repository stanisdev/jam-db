use super::area::Area;
use super::record::Record;
use super::source::Source;

pub struct QueryParser {}

impl QueryParser {
    pub fn new() -> Self {
        QueryParser {}
    }

    pub fn parse(&mut self, initial_query: &str) {
        let mut config_contents = String::new();
        let mut source = Source::new(&mut config_contents);

        // Cut excess whitespace symbols
        let mut query = String::new();
        for token in initial_query.split_whitespace().into_iter() {
            query.push_str(token);
            query.push(' ');
        }
        let query = &query[0..query.len() - 1];
        let mut index_counter = 0;

        // Get command
        if let Some(index) = query.find(' ') {
            let command = &query[0..index].to_lowercase();
            source.query.parameters.insert("command", command.to_string());
            index_counter = index + 1;
        } else {
            panic!("Boom!");
        }
        // Get destination
        let sub_query = &query[index_counter..];
        if let Some(index) = sub_query.find(' ') {
            let destination = sub_query[0..index].to_string().to_lowercase();
            source.query.parameters.insert("destination", destination);
            index_counter = index + 1;
        } else {
            panic!("Boom!");
        }
        let attributes = &sub_query[index_counter..];
        source.query.parameters.insert("attributes", attributes.to_string());
        self.execute(source);
    }

    pub fn execute(&self, source: Source) {
        let destination = source.query.parameters.get("destination").unwrap();
        match destination.as_str() {
            "area" => Area::new(source).execute(),
            "record" => Record::new().execute(),
            _ => (),
        };
    }
}
