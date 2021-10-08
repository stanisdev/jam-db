use super::area::Area;
use super::record::Record;
use super::container::get_container;

pub struct QueryParser {}

impl QueryParser {
    pub fn new() -> Self {
        QueryParser {}
    }
    
    /// Initial parsing a string representing a query
    pub fn parse(&mut self, initial_query: &str) {
        let container = get_container();

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
            container.insert("query:command", Box::leak(command.to_string().into_boxed_str())); //@todo: change this
            index_counter = index + 1;
        } else {
            panic!("Boom!");
        }
        // Get destination
        let sub_query = &query[index_counter..];
        if let Some(index) = sub_query.find(' ') {
            let destination = sub_query[0..index].to_string().to_lowercase();
            container.insert("query:destination", Box::leak(destination.into_boxed_str()));
            index_counter = index + 1;
        } else {
            panic!("Boom!");
        }
        let attributes = &sub_query[index_counter..];
        container.insert("query:attributes", Box::leak(attributes.to_string().into_boxed_str()));
    }

    /// Continue executing the query based on
    /// the collected initial data
    pub fn execute(&self) {
        let destination: &str = get_container().get("query:destination").unwrap();
        match destination {
            "area" => {
                let mut area = Area::new();
                area.execute();
            },
            "record" => Record::new().execute(),
            _ => (),
        };
    }
}
