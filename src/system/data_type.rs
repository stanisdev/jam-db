use std::collections::HashMap;
use super::validators::LengthValidator;
use super::container::get_container;
use super::container::get_source;
use super::types::{Message, Section};

pub struct DataType<'a> {
    data_type: &'a str,
    field_parameters: HashMap<&'a str, &'a str>,
}

impl Section for DataType<'_> {
    fn execute(&mut self) -> Result<(), String> {
        if !self.is_correct() {
            return self.build_error(
                "The data type '{}' does not exist or is not put into operation yet"
            );
        }
        for (name, value) in &self.field_parameters {
            let result = match *name {
                "auto_increment" => self.auto_increment(value),
                "default" => self.default(value),
                "length" => self.length(value),
                "interval" => self.interval(value),
                _ => self.build_error("The parameter '{}' is incorrect"),
            };
            if let Err(message) = result {
                return Err(message);
            }
        }
        Ok(())
    }
}

impl<'a> DataType<'a> {
    pub fn new(field_parameters: HashMap<&'a str, &'a str>) -> DataType<'a> {
        DataType {
            data_type: get_container().get("field:data_type"),
            field_parameters,
        }
    }

    pub fn is_correct(&self) -> bool {
        get_source().config.types.available.contains(&self.data_type)
    }

    pub fn auto_increment(&self, value: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn default(&self, value: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn length(&self, value: &str) -> Result<(), String> {
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
        Ok(())
    }

    pub fn interval(&self, value: &str) -> Result<(), String> {
        Ok(())
    }
}
