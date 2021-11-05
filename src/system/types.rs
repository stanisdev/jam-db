use std::fmt;
use std::collections::HashMap;

pub trait Message {
    fn build_error(&self, message: &str) -> Result<(), String> {
        Err(String::from(message))
    }
}

pub trait Section {
    fn execute(&mut self) -> Result<(), String>;
}

impl<T> Message for T where T: Section {}

pub struct AreaField<'a> {
    pub name: &'a str,
    pub parameters: &'a str,
}

pub struct AreaAttribute<'a> {
    pub option: &'a str,
    pub components: &'a str,
}

pub type Dictionary<'a> = HashMap<&'a str, &'a str>;

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum Destination {
        Area,
        Record,
    }
}

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum SystemOption {
        Fields,
        Restriction,
        Index,
    }
}

impl fmt::Display for SystemOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

custom_derive! {
    #[derive(Debug, EnumFromStr)]
    pub enum FieldParameter {
        AutoIncrement,
        Default,
        Length,
        Interval,
    }
}
