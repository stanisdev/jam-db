use enum_derive::ParseEnumError;
use super::area::AreaParser;
use super::types::{Destination};
use super::utils::Utils;
use super::query_metadata::QueryMetadata;

pub struct QueryParser<'a> {
    initial_query: &'a str,
    metadata: QueryMetadata,
}

impl<'a> QueryParser<'a> {
    /**
     * Initial parsing a string representing a query
     */
    pub fn execute(&mut self) -> Result<(), &str> {
        let mut query_string = String::new();

        // Cut excess whitespace symbols
        for token in self.initial_query.split_whitespace().into_iter() {
            query_string.push_str(token);
            query_string.push(' ');
        }
        let query_string = &query_string[0..query_string.len() - 1];

        // Get a command
        let space_index = query_string.find(' ').ok_or("The query is incorrect")?;
        let command = &query_string[0..space_index].to_lowercase();
        self.metadata.command = Some(command.to_string());

        // Get a destination
        let query_string = &query_string[space_index + 1..];
        let space_index = query_string.find(' ').ok_or("Destination of the query cannot be recognized")?;
        let mut destination = query_string[0..space_index].to_string().to_lowercase();
        destination = Utils::capitalize_first_letter(destination.as_str());
        self.metadata.destination = Some(destination);
        
        self.metadata.attributes = Some((&query_string[space_index + 1..]).to_string());
        self.run_destination()
    }
}

impl<'a> QueryParser<'a> {
    /**
     * Continue executing the query based on
     * the collected initial data
     */
    fn run_destination(&self) -> Result<(), &str> {
        let result: Result<Destination, ParseEnumError> = self.metadata.destination
            .as_ref()
            .unwrap()
            .parse();
        let destination = result.ok().ok_or("The destination specified incorrectly")?;
        match destination {
            Destination::Area => {
                let area_parser = AreaParser::new(&self.metadata);
                area_parser.execute()
            },
            Destination::Record => Ok(()),
        }
    }

    pub fn new(initial_query: &'a str) -> Self {
        QueryParser {
            initial_query,
            metadata: QueryMetadata::new(),
        }
    }
}
