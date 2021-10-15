use std::collections::HashMap;
use super::container::get_container;
use super::container::get_source;
use super::types::{Message, Section, AreaAttribute};
use super::field::Field;

pub struct Area<'a> {
    attributes: Vec<AreaAttribute<'a>>,
}

impl Section for Area<'_> {
    /**
     * Execute the query being intended to an area
     */
    fn execute(&mut self) -> Result<(), String> {
        let attributes_str = get_container().get("query:attributes");
        let mut sub_string = attributes_str;
        let area_name: &str;

        // Get area name
        if let Some(index) = sub_string.find(' ') {
            area_name = &attributes_str[0..index];
            sub_string = &attributes_str[index + 1..];
        } else {
            return self.build_error("Name of an area cannot be recognized");
        }
        let sub_string_lowercase = sub_string.to_lowercase();
        let mut option_indexes: HashMap<usize, &str> = HashMap::new(); // @todo: change this

        for option in get_source().config.area.options.iter() {
            if let Some(index) = sub_string_lowercase.find(option) {
                option_indexes.insert(index, option);
            }
        }
        if option_indexes.is_empty() {
            return self.build_error("The query have no any options");
        }
        let mut indexes: Vec<_> = option_indexes.keys().collect();
        indexes.sort();
        
        let mut counter = 0;

        while counter < indexes.len() {
            let index = indexes[counter];
            let option = option_indexes.get(index).unwrap();
            let from = *index + option.len();

            let range = if counter + 1 < indexes.len() {
                let to = *indexes[counter + 1];
                from..to
            } else {
                from..sub_string.len()
            };
            self.attributes.push(AreaAttribute {
                option,
                components: &sub_string[range].trim(),
            });
            counter += 1;
        }
        self.parse_attributes()
    }
}

impl<'a> Area<'a> {
    pub fn new() -> Area<'a> {
        Area {
            attributes: vec![],
        }
    }

    /**
     * Parse attributes of the query
     */
    fn parse_attributes(&self) -> Result<(), String> {
        for element in &self.attributes {
            let result = match element.option {
                "fields" => Field::new(element.components).execute(),
                "restriction" => Ok(()),
                "index" => Ok(()),
                _ => Err("The option '{}' is unknown".to_string()),
            };
            if let Err(message) = result {
                return Err(message);
            }
        }
        Ok(())
    }
}
