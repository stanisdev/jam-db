pub struct Area<'a> {
    command: &'a str,
    attributes: &'a str,
}

impl Area<'_> {
    pub fn new<'a>(command: &'a str, attributes: &'a str) -> Area<'a> {
        Area {
            command,
            attributes,
        }
    }

    pub fn execute(&self) {
        println!("{}", 1);
    }
}
