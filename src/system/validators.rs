use std::collections::HashMap;

pub struct LengthValidator<'a> {
    data_type: &'a str,
    parameters: HashMap<&'a str, &'a str>,
    min: Option<u32>,
    max: Option<u32>,
}

impl<'a> LengthValidator<'_> {
    pub fn new(data_type: &'a str, parameters: HashMap<&'a str, &'a str>) -> LengthValidator<'a> {
        LengthValidator {
            data_type,
            parameters,
            min: None,
            max: None,
        }
    }

    pub fn verify(&mut self) -> Result<(), &'a str> {
        for (parameter, value) in &self.parameters {
            let number = match value.parse::<u32>() {
                Ok(number) => number,
                Err(_) => panic!("The '{}' value not a number", value),
            };
            match *parameter {
                "min" => self.min = Some(number),
                "max" => self.max = Some(number),
                _ => panic!("Boom"),
            };
        }
        if self.min != None {
            let min = self.min.unwrap();
            if min < 1 {
                panic!("Min value cannot be less than 1");
            } else if min > 1000 {
                panic!("Min value cannot be more than 1000"); // @todo: move to a config
            }
        }
        if self.max != None {
            let max = self.max.unwrap();
            if max < 1 {
                panic!("Max value cannot be less than 1");
            } else if max > 1000 {
                panic!("Max value cannot be more than 1000");
            }
        }
        if
            self.min != None &&
            self.max != None &&
            self.min.unwrap() >= self.max.unwrap()
        {
            panic!("Min value cannot be greater or equal max");
        }
        Ok(())
    }
}
