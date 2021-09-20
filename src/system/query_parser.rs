use std::collections::HashMap;
use super::area::Area;
use super::record::Record;

pub struct QueryParser<'a> {
    parameters: HashMap<&'a str, String>,
}

impl QueryParser<'_> {
    pub fn new() -> Self {
        QueryParser {
            parameters: HashMap::new(),
        }
    }

    pub fn parse(&mut self, initial_query: &str) {
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
            self.parameters.insert("command", command.to_string());
            index_counter = index + 1;
        } else {
            panic!("Boom!");
        }
        // Get destination
        let sub_query = &query[index_counter..];
        if let Some(index) = sub_query.find(' ') {
            let destination = sub_query[0..index].to_string().to_lowercase();
            self.parameters.insert("destination", destination);
            index_counter = index + 1;
        } else {
            panic!("Boom!");
        }
        let attributes = &sub_query[index_counter..];
        self.parameters.insert("attributes", attributes.to_string());
    }

    pub fn execute(&self) {
        let command = self.parameters.get("command").unwrap().as_str();
        let destination = self.parameters.get("destination").unwrap().as_str();
        let attributes = self.parameters.get("attributes").unwrap().as_str();
        match destination {
            "area" => Area::new(command, attributes).execute(),
            "record" => Record::new().execute(),
            _ => (),
        };
    }
}
