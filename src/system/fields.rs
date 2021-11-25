use super::types::{AreaField};
use super::data_type_validator::DataTypeValidator;
use super::common_structs::AreaOptionElement;
use regex::Regex;

pub struct Fields<'a> {
    pub instances: Vec<AreaOptionElement<'a>>
}

impl Fields<'_> {
    pub fn new() -> Self {
        Fields {
            instances: vec![],
        }
    }
}

impl<'a> Fields<'a> {
    /**
     * Decompose a plain string to the fields
     */
    pub fn recognize(&mut self, mut components_string: &'a str) -> Result<(), &'static str> {
        let mut fields: Vec<AreaField> = Vec::new();
        loop {
            if let Some(index) = components_string.find('=') {
                let field_name = &components_string[0..index];
                if !Self::is_field_name_correct(field_name) {
                    return Err("The name '{}' cannot be used to entitle a field");
                }
                components_string = &components_string[index + 1..];
                if &components_string[0..1] != "(" {
                    return Err("The query is incorrect");
                }
                components_string = &components_string[1..];
                let components_snapshot = components_string;
                let mut substring_length = 0;

                loop {
                    if let Some(index) = components_string.find(')') {
                        let substring = &components_string[0..index];
                        components_string = &components_string[index + 1..];
                        
                        substring_length += index + 1;
                        match substring.find('(') {
                            Some(_) => (),
                            None => break,
                        }
                    } else {
                        break;
                    }
                }
                components_string = components_string.trim_start();
                if substring_length < 1 {
                    return Err("Please, consider using another way to build the query");
                }
                fields.push(AreaField {
                    name: field_name,
                    parameters_string: &components_snapshot[0..substring_length - 1],
                });
            } else {
                break;
            }
        };
        self.compose(fields)
    }
}

impl<'a> Fields<'a> {
    /**
     * Analyse section 'fields' by making up appropriate
     * HashMap of the fields and their parameters
     */
    pub fn compose(&mut self, fields: Vec<AreaField<'a>>) -> Result<(), &'static str> {
        for field in fields {
            let mut parameters_string = field.parameters_string;
            let mut data_type_validator = DataTypeValidator::new();

            loop {
                if let Some(index) = parameters_string.find('=') {
                    let parameter_name = &parameters_string[0..index];
                    parameters_string = &parameters_string[index + 1..];

                    let (parameter_value, shift) = if &parameters_string[0..1] == "(" {
                        if let Some(index) = parameters_string.find(')') {
                            (&parameters_string[1..index], index + 1)
                        } else {
                            return Err("Parameter '{}' does not contain closing bracket")
                        }
                    } else if parameters_string.contains(' ') {
                        let index = parameters_string.find(' ').unwrap();
                        (&parameters_string[0..index], index)
                    } else {
                        (parameters_string, 0) // If we've reached the end of a string
                    };
                    parameters_string = &parameters_string[shift..].trim_start();
                    data_type_validator.add_parameter(parameter_name, parameter_value);
                } else {
                    break;
                }
            };
            data_type_validator.validate()?;
            let field_instance = AreaOptionElement
                ::new(field.name, data_type_validator.parameters_instances);
            self.instances.push(field_instance);
        }
        Ok(())
    }
}

impl Fields<'_> {
    /**
     * Check correctness of a field name
     */
    fn is_field_name_correct(value: &str) -> bool {
        let re = Regex::new(r"^[\da-zA-Z_]+$").unwrap();
        re.is_match(value)
    }
}
