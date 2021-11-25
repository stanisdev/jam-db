use super::types::{FieldParameter};
use super::common_structs::Parameter;

pub struct FieldParameterValidator<'a> {
    name: FieldParameter,
    value: &'a str,
}

impl<'a> FieldParameterValidator<'a> {
    pub fn new(name: FieldParameter, value: &'a str) -> Self {
        FieldParameterValidator {
            name,
            value,
        }
    }
}

impl<'a> FieldParameterValidator<'a> {
    pub fn validate(&self) -> Result<Parameter<'a>, &'static str> {
        match self.name {
            FieldParameter::AutoIncrement => self.auto_increment(),
            FieldParameter::Default => self.default(),
            FieldParameter::Length => self.length(),
            FieldParameter::Interval => self.interval(),
        }
    }
}

impl<'a> FieldParameterValidator<'a> {
    /**
     * The "auto_increment" parameter
     */
    fn auto_increment(&self) -> Result<Parameter<'a>, &'static str> {
        let mut parameter_instance = Parameter::new();
        parameter_instance.add_conjugate("auto_increment", self.value);
        Ok(parameter_instance)
    }

    /**
     * The "default" parameter
     */
    fn default(&self) -> Result<Parameter<'a>, &'static str> {
        let mut parameter_instance = Parameter::new();
        parameter_instance.add_conjugate("default", self.value);
        Ok(parameter_instance)
    }

    /**
     * Length parameter
     */
    fn length(&self) -> Result<Parameter<'a>, &'static str> {
        let value = self.value;

        let tokens: Vec<&str> = if value.contains(' ') {
            value.split_whitespace().collect::<Vec<&str>>()
        } else {
            vec![value]
        };
        let mut result: Vec<&str> = vec![];
        for token in tokens {
            let params = token.split('=').collect::<Vec<&str>>();
            if params.len() != 2 {
                return Err("Incorrect arguments of the 'length' parameter");
            }
            for param in params {
                result.push(param);
            }
        }
        let mut parameter_instance = Parameter::new();
        parameter_instance.add_nested("length", result);
        Ok(parameter_instance)
    }

    /**
     * Interval
     */
    fn interval(&self) -> Result<Parameter<'a>, &'static str> {
        // @todo: complete this
        Ok(Parameter::new())
    }
}
