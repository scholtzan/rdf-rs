
// todo
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Comment(String),
  LanguageSpecification(String),
  Literal(String),
  Uri(String),
  BlankNode(String),
  TripleDelimiter,
  DataTypeStart,
  EndOfInput,
}