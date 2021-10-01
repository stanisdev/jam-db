use std::collections::HashMap;
use regex::Regex;
use super::parameter_type::ParameterType;
use super::source::Source;

pub struct Area<'a> {
    source: Source<'a>,
}

impl Area<'_> {
    pub fn new(source: Source) -> Area {
        Area { source }
    }

    pub fn execute(&self) {
        let attributes_str = self.source.query.parameters.get("attributes").unwrap().as_str();
        let mut sub_string = attributes_str;
        let area_name: &str;

        // Get area name
        if let Some(index) = sub_string.find(' ') {
            area_name = &attributes_str[0..index];
            sub_string = &attributes_str[index + 1..];
        } else {
            panic!("Boom");
        }
        let sub_string_lowercase = sub_string.to_lowercase();
        let mut option_indexes: HashMap<usize, &str> = HashMap::new(); // @todo: change this

        for option in self.source.config.area.options.iter() {
            if let Some(index) = sub_string_lowercase.find(option) {
                option_indexes.insert(index, option);
            }
        }
        if option_indexes.is_empty() {
            panic!("Boom");
        }
        let mut indexes: Vec<_> = option_indexes.keys().collect();
        indexes.sort();
        
        let mut counter = 0;
        let mut attributes: Vec<AreaAttribute> = Vec::new();

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
            attributes.push(AreaAttribute{
                option,
                components: &sub_string[range].trim(),
            });
            counter += 1;
        }
        self.parse_attributes(attributes);
    }

    fn parse_attributes(&self, attributes: Vec<AreaAttribute>) {
        for element in attributes {
            match element.option {
                "fields" => {
                    let mut components = element.components;
                    let mut fields: Vec<AreaField> = Vec::new();

                    loop {
                        if let Some(index) = components.find('=') {
                            let field_name = &components[0..index];
                            if !Self::is_field_name_correct(field_name) {
                                panic!("Boom 1");
                            }
                            components = &components[index + 1..];
                            if &components[0..1] != "(" {
                                panic!("Boom 2");
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
                                panic!("Boom");
                            }
                            fields.push(AreaField {
                                name: field_name,
                                parameters: &components_snapshot[0..substring_length - 1],
                            });
                        } else {
                            break;
                        }
                    };
                    Self::compose_fields(fields);
                },
                _ => (),
            }
        }
    }

    fn compose_fields(fields: Vec<AreaField>) {
        for element in fields {
            let mut parameters = element.parameters;
            let mut parsed_parameters: HashMap<&str, &str> = HashMap::new();

            loop {
                if let Some(index) = parameters.find('=') {
                    let parameter_name = &parameters[0..index];
                    parameters = &parameters[index + 1..];

                    let (parameter_value, shift) = if &parameters[0..1] == "(" {
                        if let Some(index) = parameters.find(')') {
                            (&parameters[1..index], index + 1)
                        } else {
                            panic!("Parameter does not contain closing bracket");
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
                    parsed_parameters.insert(parameter_name, parameter_value);
                } else {
                    break;
                }
            };
            if !parsed_parameters.contains_key("type") {
                panic!("Boom");
            }
            let field_type = parsed_parameters.get("type").unwrap().to_lowercase();
            parsed_parameters.remove("type");

            let parameter_type = ParameterType::new(field_type.as_str());
            if !parameter_type.is_correct() {
                panic!("Boom");
            }
            for (name, value) in parsed_parameters {
                match name {
                    "auto_increment" => parameter_type.auto_increment(value),
                    "default" => parameter_type.default(value),
                    "length" => parameter_type.length(value),
                    "interval" => parameter_type.interval(value),
                    _ => panic!("Boom"),
                };
            }
        }
    }

    fn is_field_name_correct(value: &str) -> bool {
        let re = Regex::new(r"^[\da-zA-Z_]+$").unwrap();
        re.is_match(value)
    }
}

struct AreaAttribute<'a> {
    option: &'a str,
    components: &'a str,
}

struct AreaField<'a> {
    name: &'a str,
    parameters: &'a str,
}
