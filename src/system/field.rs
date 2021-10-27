use std::collections::HashMap;
use regex::Regex;
use super::container::get_container;
use super::data_type::DataType;
use super::types::{Message, Section, AreaField};
use super::types::SystemOption;
use super::area::{
    AreaOption,
    AreaOptionElement,
    Parameter,
};

pub struct Field<'a> {
    components: &'a str,
    all: Vec<Parameter<'a>>,
}

impl Section for Field<'_> {
    fn execute(&mut self) -> Result<(), String> {
        self.recognize_fields()
    }
}

impl<'a> Field<'a> {
    pub fn new(components: &str) -> Field {
        Field {
            components,
            all: vec![],
        }
    }

    /**
     * Decompose a plain string to the fields
     */
    fn recognize_fields(&mut self) -> Result<(), String> {
        let mut components = self.components;
        let mut fields: Vec<AreaField> = Vec::new();
        let mut area_option = AreaOption::new();
        area_option.kind = Some(SystemOption::Fields);

        loop {
            if let Some(index) = components.find('=') {
                let field_name = &components[0..index];
                if !Self::is_field_name_correct(field_name) {
                    return self.build_error("The name '{}' cannot be used to entitle a field");
                }
                components = &components[index + 1..];
                if &components[0..1] != "(" {
                    return self.build_error("The query is incorrect");
                }
                components = &components[1..];
                let components_snapshot = components;

                let mut substring_length = 0;
                loop {
                    if let Some(index) = components.find(')') {
                        let substring = &components[0..index];
                        components = &components[index + 1..];
                        
                        substring_length += index + 1;
                        match substring.find('(') {
                            Some(_) => (),
                            None => break,
                        }
                    } else {
                        break;
                    }
                }
                components = components.trim_start();

                if substring_length < 1 {
                    return self.build_error("Please, consider using another way to build the query");
                }
                fields.push(AreaField {
                    name: field_name,
                    parameters: &components_snapshot[0..substring_length - 1],
                });
            } else {
                break;
            }
        };
        self.compose_fields(fields)
    }

    /**
     * Analyse section 'fields' by making up appropriate
     * HashMap of the fields and their parameters
     */
    fn compose_fields(&mut self, fields: Vec<AreaField<'a>>) -> Result<(), String> {
        let mut all: Vec<AreaOptionElement> = vec![];
        for field in fields {
            let mut field_instance = AreaOptionElement::new();
            field_instance.name = Some(field.name);

            let mut parameters = field.parameters;
            let mut field_parameters: HashMap<&str, &str> = HashMap::new();

            loop {
                if let Some(index) = parameters.find('=') {
                    let parameter_name = &parameters[0..index];
                    parameters = &parameters[index + 1..];

                    let (parameter_value, shift) = if &parameters[0..1] == "(" {
                        if let Some(index) = parameters.find(')') {
                            (&parameters[1..index], index + 1)
                        } else {
                            return self.build_error("Parameter '{}' does not contain closing bracket")
                        }
                    }
                    else if parameters.contains(' ') {
                        let index = parameters.find(' ').unwrap();
                        (&parameters[0..index], index)
                    }
                    else {
                        (parameters, 0) // If we've reached the end of a string
                    };
                    parameters = &parameters[shift..].trim_start();
                    field_parameters.insert(parameter_name, parameter_value);
                } else {
                    break;
                }
            };
            if !field_parameters.contains_key("type") {
                return self.build_error("Specify a type for the field '{}'"); // element.name
            }
            let field_data_type = field_parameters
                .get("type")
                .unwrap()
                .to_lowercase()
                .to_string();

            get_container().set("field:data_type", field_data_type);
            field_parameters.remove("type");

            let s = match DataType::new(field_parameters).execute() {
                Ok(parameters) => parameters,
                Err(message) => return Err(message),
            };
            self.all = s;
            // field_instance.parameters = s;
            all.push(field_instance);
        }
        Ok(())
    }

    /**
     * Check correctness of a field name
     */
    fn is_field_name_correct(value: &str) -> bool {
        let re = Regex::new(r"^[\da-zA-Z_]+$").unwrap();
        re.is_match(value)
    }
}

