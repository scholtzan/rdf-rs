use uri::Uri;
use namespace::*;
use sparql::pattern::Pattern;
use Result;


/// Query type.
#[derive(Clone, Debug)]
pub enum SparqlQueryType {
  Ask,
  Select,
  SelectDistinct,
  SelectReduced,
  SelectAll,
  SelectAllDistinct,
  SelectAllReduced,
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
  namespaces: NamespaceStore,

  // Variables used in the query.
  variables: Vec<String>,

  // Patterns used as constraints.
  patterns: Vec<Box<Pattern>>
}


impl SparqlQuery {
  /// Constructor of `SparqlQuery`.
  pub fn new(query_type: SparqlQueryType) -> SparqlQuery {
    SparqlQuery {
      query_type: query_type,
      base_uri: None,
      variables: Vec::new(),
      patterns: Vec::new(),
      namespaces: NamespaceStore::new()
    }
  }

  /// Add variables to the query.
  /// Ordering in vector reflects position the variables appear.
  pub fn add_variables(&mut self, variables: Vec<String>) {
    self.variables = variables;
  }

  /// Add pattern to the query.
  pub fn add_pattern(&mut self, pattern: Box<Pattern>) {
    self.patterns.push(pattern);
  }

  /// Returns the type of the SPARQL query.
  ///
  /// todo
  ///
  pub fn get_query_type(&self) -> &SparqlQueryType {
    &self.query_type
  }

  /// Get query variables.
  ///
  /// todo
  ///
  pub fn get_query_variables(&self) -> &Vec<String> {
    &self.variables
  }

  /// Get the query patterns in the `WHERE` expression.
  ///
  /// todo
  ///
  pub fn get_query_patterns(&self) -> &Vec<Pattern> {
    &self.patterns
  }

  /// Returns the URI of a namespace with the provided prefix.
  ///
  /// # Examples
  ///
  /// todo
  ///
  /// # Failures
  ///
  /// - No namespace with the provided prefix exists
  ///
  pub fn get_namespace_uri_by_prefix(&self, prefix: String) -> Result<&Uri> {
    self.namespaces.get_uri_by_prefix(prefix)
  }
}