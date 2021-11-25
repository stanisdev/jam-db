#[derive(Debug)]
pub struct QueryMetadata {
    pub command: Option<String>,
    pub attributes: Option<String>,
    pub destination: Option<String>,
}

impl QueryMetadata {
    pub fn new() -> Self {
        QueryMetadata {
            command: None,
            attributes: None,
            destination: None,
        }
    }
}
