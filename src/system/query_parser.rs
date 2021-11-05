use enum_derive::ParseEnumError;
use super::area::{AreaParser};
use super::record::Record;
use super::container::get_container;
use super::types::{Section, Message, Destination};
use super::utils::Utils;

pub struct QueryParser<'a> {
    query: &'a str,
    destination: Option<String>,
}

impl Section for QueryParser<'_> {
    /**
     * Initial parsing a string representing a query
     */
    fn execute(&mut self) -> Result<(), String> {
        let container = get_container();

        // Cut excess whitespace symbols
        let mut query = String::new();
        for token in self.query.split_whitespace().into_iter() {
            query.push_str(token);
            query.push(' ');
        }
        let query = &query[0..query.len() - 1];
        let mut index_counter = 0;

        // Get command
        if let Some(index) = query.find(' ') {
            let command = &query[0..index].to_lowercase();
            container.set("query:command", command.to_string());
            index_counter = index + 1;
        } else {
            return self.build_error("The query is incorrect");
        }
        // Get destination
        let sub_query = &query[index_counter..];
        if let Some(index) = sub_query.find(' ') {
            let destination = sub_query[0..index].to_string().to_lowercase();
            self.destination = Some(Utils::capitalize_first_letter(destination.as_str()));
            index_counter = index + 1;
        } else {
            return self.build_error("Destination of the query cannot be recognized");
        }
        let attributes = &sub_query[index_counter..];
        container.set("query:attributes", attributes.to_string());
        self.run_destination()
    }
}

impl<'a> QueryParser<'a> {
    pub fn new(query: &'a str) -> Self {
        QueryParser {
            query,
            destination: None,
        }
    }

    /**
     * Continue executing the query based on
     * the collected initial data
     */
    fn run_destination(&self) -> Result<(), String> {
        let result: Result<Destination, ParseEnumError> = self.destination
            .as_ref()
            .unwrap()
            .parse();
        match result {
            Ok(destination) => match destination {
                Destination::Area => {

                    let mut area_paraser = AreaParser::new();
                    let result = area_paraser.execute();
                    if let Err(message) = result {
                        return Err(message);
                    }
                    let mut area_instance = area_paraser.instance;
                    area_instance.options = area_paraser.options;
                    area_instance.save();

                    Ok(())
                },
                Destination::Record => Record::new().execute(),
            },
            Err(_) => self.build_error("The destination specified incorrectly"),
        }
    }
}
