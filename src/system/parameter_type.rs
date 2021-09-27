use std::collections::HashMap;
use super::validators::LengthValidator;

pub struct ParameterType<'a> {
    name: &'a str,
}

impl ParameterType<'_> {
    const AVAILABLE_TYPES: [&'static str; 3] = ["int", "bool", "string"];

    pub fn new(name: &str) -> ParameterType {
        ParameterType { name }
    }

    pub fn is_correct(&self) -> bool {
        Self::AVAILABLE_TYPES.contains(&self.name)
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
        let mut validator = LengthValidator::new(self.name, parsed_value);
        match validator.verify() {
            Err(message) => println!("{}", message),
            Ok(_) => (),
        };
    }

    pub fn interval(&self, value: &str) {
        
    }
}
