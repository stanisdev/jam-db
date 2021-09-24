pub struct ParameterType<'a> {
    name: &'a str,
}

impl ParameterType<'_> {
    const AVAILABLE_TYPES: [&'static str; 3] = ["int", "bool", "string"];

    pub fn new(name: &str) -> ParameterType {
        ParameterType { name }
    }

    pub fn is_correct(&self) -> bool {
        Self::AVAILABLE_TYPES.contains(&self.name)
    }

    pub fn auto_increment(&self, value: &str) {

    }

    pub fn default(&self, value: &str) {

    }

    pub fn length(&self, value: &str) {

    }

    pub fn interval(&self, value: &str) {

    }
}
