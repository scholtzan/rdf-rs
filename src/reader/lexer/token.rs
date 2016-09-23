
// todo
pub enum Token {
  Comment(&'static str),
  LanguageSpecification(&'static str),
  TripleDelimiter,
}