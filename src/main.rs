#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
mod system;
use system::query_parser::QueryParser;

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
    if let Err(message) = qp.execute() {
        println!("{}", message);
    }
}
