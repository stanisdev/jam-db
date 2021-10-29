use enum_derive::ParseEnumError;
use std::collections::HashMap;
use super::container::get_container;
use super::container::get_source;
use super::field::Field;
use super::utils::Utils;
use super::types::{
    Message,
    Section,
    AreaAttribute,
    SystemOption,
    Dictionary,
};

pub struct AreaParser<'a> {
    attributes: Vec<AreaAttribute<'a>>,
    pub instance: AreaInstance<'a>,
    pub options: Vec<AreaOption<'a>>,
}

impl Section for AreaParser<'_> {
    /**
     * Execute the query being intended to an area
     */
    fn execute(&mut self) -> Result<(), String> {
        let attributes_str = get_container().get("query:attributes");
        let mut sub_string = attributes_str;

        // Get area name
        if let Some(index) = sub_string.find(' ') {
            self.instance.name = Some(&attributes_str[0..index]);
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

impl<'a> AreaParser<'a> {
    pub fn new() -> AreaParser<'a> {
        AreaParser {
            attributes: vec![],
            instance: AreaInstance::new(),
            options: vec![],
        }
    }

    /**
     * Parse attributes of the query
     */
    fn parse_attributes(&mut self) -> Result<(), String> {
        for element in &self.attributes {
            let option = Utils::capitalize_first_letter(element.option).parse();
            let result: Result<SystemOption, ParseEnumError> = option;
            let execution_result = match result {
                Ok(option) => match option {
                    SystemOption::Fields => {

                        let mut field = Field::new(element.components);
                        if let Err(message) = field.execute() {
                            return Err(message);
                        }
                        let area_option = AreaOption::new(SystemOption::Fields, field.instances);
                        self.options.push(area_option);
                        Ok(())
                    },
                    SystemOption::Restriction => Ok(()),
                    SystemOption::Index => Ok(()),
                },
                Err(_) => self.build_error("The option '{}' is unknown"),
            };
            if let Err(message) = execution_result {
                return Err(message);
            }
        }
        Ok(())
    }
}

/**
 *  AreaInstance
 */
pub struct AreaInstance<'a> {
    name: Option<&'a str>,
    pub options: Vec<AreaOption<'a>>,
}

impl<'a> AreaInstance<'a> {
    pub fn new() -> AreaInstance<'a> {
        AreaInstance {
            name: None,
            options: vec![],
        }
    }

    /**
     * Save a new area
     */
    pub fn save(&self) {}
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
    pub conjugate: Option<Dictionary<'a>>, // default=false
    pub nested: Option<HashMap<&'a str, Dictionary<'a>>>, // length=(min=2 max=100)
}

impl<'a> Parameter<'a> {
    pub fn new() -> Parameter<'a> {
        Parameter {
            conjugate: None,
            nested: None,
        }
    }
}
