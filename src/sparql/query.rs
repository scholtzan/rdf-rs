use uri::Uri;
use namespace::*;


/// Query type.
pub enum SparqlQueryType {
  Ask,
  Select,
  Construct,
  Describe
}

/// Represents a SPARQL query that can be applied to an RDF graph.
/// `SparqlQuery`s are created when parsing a SPARQL string using `SparqlParser`.
pub struct SparqlQuery {
  /// SPARQL query type.
  query_type: SparqlQueryType,

  /// Base URI denoted in SPARQL query.
  base_uri: Option<Uri>,

  /// Namespaces stated in SPARQL query.
  namespaces: Option<NamespaceStore>,

  // todo: variables, order by, ...
}


impl SparqlQuery {
  /// Constructor of `SparqlQuery`.
  pub fn new(query_type: SparqlQueryType) -> SparqlQuery {
    SparqlQuery {
      query_type: query_type,
      base_uri: None,
      namespaces: None
    }
  }
}