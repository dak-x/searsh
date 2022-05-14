
use crate::word::Word;

type QueryNode = Box<QueryType>;
#[non_exhaustive]
enum QueryType {    
    AND(QueryNode, QueryNode),
    OR(QueryNode, QueryNode),
    NOT(QueryNode),
    MATCH(Word),
}
struct Query {

}