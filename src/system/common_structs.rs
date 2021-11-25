use std::collections::HashMap;
use super::types::{SystemOption, Dictionary};

/**
 *  AreaInstance
 */
pub struct AreaInstance<'a> {
    name: &'a str,
    pub options: Vec<AreaOption<'a>>,
}

impl<'a> AreaInstance<'a> {
    pub fn new(name: &'a str) -> AreaInstance<'a> {
        AreaInstance {
            name,
            options: vec![],
        }
    }
}

impl<'a> AreaInstance<'a> {
    pub fn save(&self) -> Result<(), &'static str> {
        let mut result_string = String::from(self.name);
        result_string.push_str("[");

        // Iterate through options
        for option in &self.options {

            let option_kind = option.kind.to_string().to_lowercase();
            result_string.push_str(
                format!("{}[", option_kind).as_str()
            );

            // Iterate through the elements of an option
            for option_element in &option.elements {

                // Iterate parameters of an element
                let mut conjugate_parameters: Dictionary = HashMap::new();
                let mut nested_parameters: HashMap<&str, &Dictionary> = HashMap::new();

                for option_element_parameter in &option_element.parameters {
                    if !option_element_parameter.conjugate.is_empty() {
                        conjugate_parameters.extend(&option_element_parameter.conjugate);
                    }
                    if !option_element_parameter.nested.is_empty() {
                        for (key, value) in &option_element_parameter.nested {
                            nested_parameters.insert(key, value);
                        }
                    }
                }
                
                // Get the type
                let the_type = conjugate_parameters.get("type").unwrap();
                result_string.push_str(
                    format!("{}:{}[", option_element.name, the_type).as_str()
                );
                let mut interim_string = String::new();
                
                // Add other conjugate parameters
                let iter = conjugate_parameters
                    .into_iter()
                    .filter(|(key, _)| *key != "type");
                for (key, value) in iter {
                    interim_string.push_str(
                        format!("{}:{},", key, value).as_str()
                    );
                }

                // Add nested parameters
                for (name, parameters) in nested_parameters {
                    interim_string.push_str(
                        format!("{}:[", name).as_str()
                    );
                    for (key, value) in parameters {
                        interim_string.push_str(
                            format!("{}:{},", key, value).as_str()
                        );
                    }
                    // Remove trailing comma
                    let interim_string_len = interim_string.len();
                    if &interim_string[interim_string_len - 1..interim_string_len] == "," {
                        interim_string = String::from(
                            &interim_string[0..interim_string_len - 1]
                        );
                    }
                    interim_string.push_str("],");
                }
                if interim_string.len() > 0 {
                    result_string.push_str(
                        &interim_string[..interim_string.len() - 1]
                    );
                }
                result_string.push_str("]");
            }
            result_string.push_str("]");
        }
        result_string.push_str("]");
        Ok(())
    }
}

/**
 * AreaOption
 */
#[derive(Debug)]
pub struct AreaOption<'a> { // fields, restriction, index
    kind: SystemOption,
    elements: Vec<AreaOptionElement<'a>>,
}

impl<'a> AreaOption<'a> {
    pub fn new(kind: SystemOption, elements: Vec<AreaOptionElement<'a>>) -> AreaOption<'a> {
        AreaOption {
            kind,
            elements,
        }
    }
}

/**
 * AreaOptionElement
 */
#[derive(Debug)]
pub struct AreaOptionElement<'a> { // id=(type=INT auto_increment=true)
    name: &'a str,
    parameters: Vec<Parameter<'a>>,
}

impl<'a> AreaOptionElement<'a> {
    pub fn new(name: &'a str, parameters: Vec<Parameter<'a>>) -> AreaOptionElement<'a> {
        AreaOptionElement {
            name,
            parameters,
        }
    }
}

/**
 * Parameter
 */
#[derive(Debug)]
pub struct Parameter<'a> {
    pub conjugate: Dictionary<'a>, // default=false
    pub nested: HashMap<&'a str, Dictionary<'a>>, // length=(min=2 max=100)
}

impl<'a> Parameter<'a> {
    pub fn new() -> Parameter<'a> {
        Parameter {
            conjugate: HashMap::new(),
            nested: HashMap::new(),
        }
    }

    pub fn add_conjugate(&mut self, name: &'a str, value: &'a str) {
        self.conjugate.insert(name, value);
    }

    pub fn add_nested(&mut self, name: &'a str, values: Vec<&'a str>) {
        let mut result: Dictionary = HashMap::new();
        let iter = values.iter().enumerate().step_by(2);
        for (index, value) in iter {
            result.insert(value, values.get(index + 1).unwrap());
        }
        self.nested.insert(name, result);
    }
}
