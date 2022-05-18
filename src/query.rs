use std::io::{stdin, BufRead};
use crate::word::Word;
use regex::Regex;
use lazy_static::lazy_static;
use std::str::FromStr;
use recap::Recap;

//lazy static so that the query can be global
//recap because https://docs.rs/recap/latest/recap/ of string parsing
//other used as standard libraries

#[recap(regex = r"\s*(?P<word1>-?\w).*\s*(?P<operator_type>-?\w).*\s*(?P<word2>-?\w)")]

type QueryNode = Box<QueryType>;
#[non_exhaustive]
enum QueryType {    
    AND(QueryNode, QueryNode),
    OR(QueryNode, QueryNode),
    NOT(QueryNode),
    MATCH(Word),
}

struct query {
    word1: String,
    operator_type: String,
    word2: String
}


#[derive(Debug, Clone)]
struct ParsingError;

impl Display for ParsingError {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "Unable to parse.")
  }
}

impl Error for ParsingError {
  fn description(&self) -> &str {
    "Unable to parse."
  }

  fn cause(&self) -> Option<&Error> {
    None
  }
}

impl From<ParseError> for ParsingError {
  fn from(_error: ParseError) -> Self {
    ParsingError
  }
}

impl FromStr for query {
    type Err = ParsingError;
  
    fn from_str(query_str: &str) -> Result<Self, Self::Err> {
      lazy_static! {
        static ref regex: Regex = Regex::new(query_REGEX).unwrap();
      }
  
      regex.captures(query_str) 
        .ok_or(ParsingError)
        .and_then(|cap| Ok(query {
          word1: cap[1].parse()?,
          operator_type: cap[2].parse()?,
          word2: cap[3].parse()?
        }))
    }
  }

fn main() {
  let mut queries: Vec<query> = stdin()
    .lock()
    .lines()
    .filter_map(|line_result| line_result.ok()) 
    .filter_map(|line| line.parse().ok())       //returns iterator
    .collect();                                 

  
}
