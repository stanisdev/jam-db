mod system;
use system::query_parser::QueryParser;

fn main() {
    let query = "CREATE AREA countries
        FIELDS
            id=(type=INT auto_increment=true)
            population=(type=INT length=(max=100 min=5))
            is_europe=(type=BOOL default=false)
            description=(type=STRING length=(min=2 max=200))";
    let mut qp = QueryParser::new();
    qp.parse(query);
    qp.execute();
}
