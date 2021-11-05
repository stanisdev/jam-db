use enum_derive::ParseEnumError;
use std::collections::HashMap;
use super::container::get_source;
use super::utils::Utils;
use super::area::Parameter;
use super::types::{
    Dictionary,
    FieldParameter,
};

pub struct DataType<'a> {
    data_type: &'a str,
    field_parameters: Dictionary<'a>,
}

impl<'a> DataType<'a> {
    pub fn execute(&mut self) -> Result<Vec<Parameter<'a>>, String> {
        if !self.is_correct() {
            return Err("The data type '{}' does not exist or is not put into operation yet"
                .to_string());
        }
        let mut result: Vec<Parameter> = vec![];
        for (name, value) in &self.field_parameters {
            let parameter_name: Result<FieldParameter, ParseEnumError> = Utils
                ::snake_case_to_camel_case(name)
                .parse();

            let parameter_instance = match parameter_name {
                Ok(field_parameter) => {
                    let result = match field_parameter {
                        FieldParameter::AutoIncrement => self.auto_increment(value),
                        FieldParameter::Default => self.default(value),
                        FieldParameter::Length => self.length(value),
                        FieldParameter::Interval => self.interval(value),
                    };
                    result
                },
                Err(_) => Err("The parameter '{}' is incorrect".to_string()),
            };
            if let Err(message) = parameter_instance {
                return Err(message);
            }
            result.push(parameter_instance.unwrap());
        }
        let mut type_parameter = Parameter::new(); // @todo: optimize this
        let mut payload: Dictionary = HashMap::new();
        payload.insert("type", self.data_type);
        type_parameter.conjugate = Some(payload);

        result.push(type_parameter);
        Ok(result)
    }
}

impl<'a> DataType<'a> {
    pub fn new(field_parameters: Dictionary<'a>, data_type: &'a str) -> DataType<'a> {
        DataType {
            data_type,
            field_parameters,
        }
    }

    /**
     * The "auto_increment" parameter
     */
    pub fn auto_increment(&self, value: &'a str) -> Result<Parameter<'a>, String> {
        let mut parameter = Parameter::new();
        let mut payload: Dictionary = HashMap::new();
        payload.insert("auto_increment", value);
        parameter.conjugate = Some(payload);

        Ok(parameter)
    }

    /**
     * The "default" parameter
     */
    pub fn default(&self, value: &'a str) -> Result<Parameter<'a>, String> {
        let mut parameter = Parameter::new();
        let mut payload: Dictionary = HashMap::new();
        payload.insert("default", value);
        parameter.conjugate = Some(payload);

        Ok(parameter)
    }

    /**
     * Length parameter
     */
    pub fn length(&self, value: &'a str) -> Result<Parameter<'a>, String> {
        let mut parsed_value: Dictionary = HashMap::new();

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
        let mut parameter = Parameter::new();
        let mut payload = HashMap::new();
        payload.insert("length", parsed_value);
        parameter.nested = Some(payload);

        // let mut validator = LengthValidator::new(self.data_type, parsed_value);
        Ok(parameter)
    }

    /**
     * Interval
     */
    pub fn interval(&self, value: &str) -> Result<Parameter<'a>, String> {
        Ok(Parameter::new()) // @todo: complete this
    }

    /**
     * Is a data type is correct
     */
    pub fn is_correct(&self) -> bool {
        get_source().config.types.available.contains(&self.data_type)
    }
}
