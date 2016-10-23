
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
  PrefixDirective,
  BaseDirective,
  QName(String, String),
  Prefix(String),
  PredicateListDelimiter,   // e.g. for Turtle syntax -> ;
  ObjectListDelimiter,      // e.g. for Turtle syntax -> ,
  EndOfInput,
}