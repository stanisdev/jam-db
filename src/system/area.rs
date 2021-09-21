use std::collections::HashMap;
use regex::Regex;

pub struct Area<'a> {
    command: &'a str,
    attributes: &'a str,
}

impl Area<'_> {
    const OPTIONS: [&'static str; 3] = ["restriction", "fields", "index"];

    pub fn new<'a>(command: &'a str, attributes: &'a str) -> Area<'a> {
        Area {
            command,
            attributes,
        }
    }

    pub fn execute(&self) {
        let mut sub_string = self.attributes;
        let area_name: &str;

        // Get area name
        if let Some(index) = sub_string.find(' ') {
            area_name = &self.attributes[0..index];
            sub_string = &self.attributes[index + 1..];
        } else {
            panic!("Boom");
        }
        let sub_string_lowercase = sub_string.to_lowercase();
        let mut option_indexes: HashMap<usize, &str> = HashMap::new(); // @todo: change this

        for option in Area::OPTIONS.iter() {
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
            let mut parameters = String::new();

            if counter + 1 < indexes.len() {
                let to = *indexes[counter + 1];
                parameters.push_str(&sub_string[from..to].trim());
                
            } else {
                parameters.push_str(&sub_string[from..].trim());
            }
            attributes.push(AreaAttribute{ option, parameters });
            counter += 1;
        }
        self.parse_attributes(attributes);
    }

    fn parse_attributes(&self, attributes: Vec<AreaAttribute>) {
        for element in attributes {
            match element.option {
                "fields" => {
                    let mut substring = element.parameters.as_str();
                    let mut fields: HashMap<&str, &str> = HashMap::new();

                    loop {
                        if let Some(index) = substring.find('=') {
                            let field_name = &substring[0..index];
                            if !Self::is_field_name_correct(field_name) {
                                panic!("Boom 1");
                            }
                            substring = &substring[index + 1..];
                            if &substring[0..1] != "(" {
                                panic!("Boom 2");
                            }
                            substring = &substring[1..];
                            let substring_copy = substring;

                            let mut sub_length = 0;
                            loop {
                                if let Some(index) = substring.find(')') {
                                    let sub = &substring[0..index];
                                    substring = &substring[index + 1..];
                                    
                                    sub_length += index + 1;
                                    match sub.find('(') {
                                        Some(_) => (),
                                        None => break,
                                    }
                                } else {
                                    break;
                                }
                            }
                            substring = substring.trim_start();

                            if sub_length < 1 {
                                panic!("Boom");
                            }
                            let field_parameters = &substring_copy[0..sub_length - 1];
                            fields.insert(field_name, field_parameters);
                        } else {
                            break;
                        }
                    };
                },
                _ => (),
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
    parameters: String,
}
