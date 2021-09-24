mod system;
use system::query_parser::QueryParser;

fn main() {
    let query = "CREATE AREA countries
        FIELDS
            id=(type=INT auto_increment=true)
            population=(type=INT another=(one=1 two=2) interval=(max=100 min=5))
            is_europe=(type=BOOL default=false)
            description=(type=STRING length=(min=2 max=200))
        RESTRICTION
            some=(options=true)
        INDEX
            population=(type=plain)
            description=(type=unique)";
    let mut qp = QueryParser::new();
    qp.parse(query);
    qp.execute();
}
