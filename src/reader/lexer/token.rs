#[derive(Debug, PartialEq, Clone)]
/// Tokens are produced by RDF lexers.
pub enum Token {
  Comment(String),
  Literal(String),
  LiteralWithUrlDatatype(String, String),   // first element is the literal, second the data type URL
  LiteralWithQNameDatatype(String, String, String), // first element is the literal, second the prefix of the QName data type, third the QName path
  LiteralWithLanguageSpecification(String, String),
  Uri(String),
  BlankNode(String),
  TripleDelimiter,
  PrefixDirective(String, String),
  BaseDirective(String),
  QName(String, String),
  Prefix(String),
  KeywordA,                 // 'a'
  PredicateListDelimiter,   // e.g. for Turtle syntax -> ;
  ObjectListDelimiter,      // e.g. for Turtle syntax -> ,
  CollectionStart,          // e.g. for Turtle syntax -> (
  CollectionEnd,            // e.g. for Turtle syntax -> )
  UnlabeledBlankNodeStart,  // e.g. for Turtle syntax -> [
  UnlabeledBlankNodeEnd,    // e.g. for Turtle syntax -> ]
  EndOfInput,

  // SPARQL
  Select,
  Distinct,
  Reduced,
  Construct,
  Describe,
  Ask,
  From,
  Named,
  Order,
  By,
  Asc,
  Desc,
  Offset,
  Optional,
  Filter,
  Graph,
  Union,
  Regex,
  Where,
  GroupStart,
  GroupEnd,
  Asterisk,
  SparqlVariable(String)    // variable in SPARQL construct with name
}