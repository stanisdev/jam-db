use std::fmt;
use std::collections::HashMap;

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
    pub enum DataType {
        Int,
        String,
        Bool,
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

#[derive(Debug)]
pub struct AreaOption<'a> {
    pub name: &'a str,
    pub components_string: &'a str,
}

#[derive(Debug)]
pub struct AreaField<'a> {
    pub name: &'a str,
    pub parameters_string: &'a str,
}

pub type Dictionary<'a> = HashMap<&'a str, &'a str>;
