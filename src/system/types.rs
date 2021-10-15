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
