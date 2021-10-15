mod system;
use system::query_parser::QueryParser;
use system::types::{Section};

fn main() {
    let query = "CREATE AREA countries
        FIELDS
            id=(type=INT auto_increment=true)
            population=(type=INT interval=(max=100 min=5))
            is_europe=(type=BOOL default=false)
            description=(type=STRING length=(min=2 max=100) default='')
        RESTRICTION
            some=(options=true)
        INDEX
            population=(type=plain)
            description=(type=unique)";
    let mut qp = QueryParser::new(query);
    match qp.execute() {
        Ok(_) => (),
        Err(message) => println!("{}", message),
    }
}
