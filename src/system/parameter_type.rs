use std::collections::HashMap;
use super::validators::LengthValidator;
use super::container::get_container;
use super::container::get_source;

pub struct ParameterType<'a> {
    data_type: &'a str,
}

impl<'a> ParameterType<'a> {
    pub fn new() -> ParameterType<'a> {
        ParameterType {
            data_type: get_container().get("field:data_type").unwrap(),
        }
    }

    pub fn is_correct(&self) -> bool {
        get_source().config.types.available.contains(&self.data_type)
    }

    pub fn auto_increment(&self, value: &str) {

    }

    pub fn default(&self, value: &str) {

    }

    pub fn length(&self, value: &str) {
        let mut parsed_value: HashMap<&str, &str> = HashMap::new();

        if value.contains(' ') {
            for token in value.split_whitespace().into_iter() {
                let elements: Vec<&str> = token.split('=').collect();
                parsed_value.insert(elements[0], elements[1]);
            }
        } else {
            let elements = value
                .split('=')
                .collect::<Vec<&str>>();
            parsed_value.insert(elements[0], elements[1]);
        }
        let mut validator = LengthValidator::new(self.data_type, parsed_value);
        match validator.verify() {
            Err(message) => println!("{}", message),
            Ok(_) => (),
        };
    }

    pub fn interval(&self, value: &str) {

    }
}
