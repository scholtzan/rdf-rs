#[derive(Debug, PartialEq, Clone)]
/// Tokens produced when reading SPARQL.
pub enum SparqlToken{
  Select,
  Ask,
  Describe
}