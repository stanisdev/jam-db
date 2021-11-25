use enum_derive::ParseEnumError;
use std::collections::HashMap;
use super::utils::Utils;
use super::types::{
    DataType,
    Dictionary,
    FieldParameter,
};
use super::common_structs::Parameter;
use super::field_parameter_validator::FieldParameterValidator;

#[derive(Debug)]
pub struct DataTypeValidator<'a> {
    the_type: Option<DataType>,
    parameters: Dictionary<'a>,
    pub parameters_instances: Vec<Parameter<'a>>,
}

impl<'a> DataTypeValidator<'a> {
    pub fn new() -> Self {
        DataTypeValidator {
            the_type: None,
            parameters: HashMap::new(),
            parameters_instances: vec![],
        }
    }
}

impl<'a> DataTypeValidator<'a> {
    pub fn add_parameter(&mut self, name: &'a str, value: &'a str) {
        self.parameters.insert(name, value);
    }
}

impl<'a> DataTypeValidator<'a> {
    /**
     * Describe me
     */
    pub fn validate(&mut self) -> Result<(), &'static str> {
        if !self.parameters.contains_key("type") {
            return Err("Specify a type for the field '{}'");
        }
        let the_type = self.parameters.get("type");
        let parsed_result_type: Result<DataType, ParseEnumError> = (
            *Utils::capitalize_first_letter(the_type.unwrap())
        ).parse();
        
        self.the_type = Some(parsed_result_type.or(Err("The type '{}' is incorrect"))?);
        let mut parameter_instance = Parameter::new();
        parameter_instance.add_conjugate("type", the_type.unwrap());

        self.parameters_instances.push(parameter_instance);
        self.parameters.remove("type");
        self.parse()
    }
}

impl<'a> DataTypeValidator<'a> {
    pub fn parse(&mut self) -> Result<(), &'static str> {
        for (name, value) in &self.parameters {
            let parsed_parameter_name: Result<FieldParameter, ParseEnumError> = Utils
                ::snake_case_to_camel_case(name)
                .parse();
            let parameter_name: FieldParameter = parsed_parameter_name.or(Err("The parameter '{}' is incorrect"))?;
            let field_parameter_validator = FieldParameterValidator::new(parameter_name, value);
            let parameter_instance = field_parameter_validator.validate()?;

            self.parameters_instances.push(parameter_instance);
        }
        Ok(())
    }
}
