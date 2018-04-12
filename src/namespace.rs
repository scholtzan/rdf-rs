use std::collections::HashMap;
use uri::Uri;
use Result;
use error::{Error, ErrorType};

/// Representation of a specific namespace.
#[derive(Debug)]
pub struct Namespace {
    prefix: String,
    uri: Uri,
}

impl Namespace {
    /// `Namespace` constructor.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::namespace::Namespace;
    /// use rdf::uri::Uri;
    ///
    /// let ns = Namespace::new("example".to_string(),
    ///                         Uri::new("http://example.org/".to_string()));
    /// ```
    pub fn new(prefix: String, uri: Uri) -> Namespace {
        Namespace {
            prefix: prefix,
            uri: uri,
        }
    }

    /// Returns the prefix of the namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::namespace::Namespace;
    /// use rdf::uri::Uri;
    ///
    /// let ns = Namespace::new("example".to_string(),
    ///                         Uri::new("http://example.org/".to_string()));
    ///
    /// assert_eq!(ns.prefix(), "example");
    /// ```
    pub fn prefix(&self) -> &String {
        &self.prefix
    }

    /// Returns the URI of the namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::namespace::Namespace;
    /// use rdf::uri::Uri;
    ///
    /// let ns = Namespace::new("example".to_string(),
    ///                         Uri::new("http://example.org/".to_string()));
    ///
    /// assert_eq!(ns.uri(), &Uri::new("http://example.org/".to_string()));
    /// ```
    pub fn uri(&self) -> &Uri {
        &self.uri
    }
}

/// Storage for multiple namespaces.
#[derive(PartialEq, Debug)]
pub struct NamespaceStore {
    /// The namespace prefix is associated with the namespace URI.
    namespaces: HashMap<String, Uri>,
}

impl NamespaceStore {
    /// Constructor for `NamespaceStore`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::namespace::NamespaceStore;
    ///
    /// let nss = NamespaceStore::new();
    /// ```
    pub fn new() -> NamespaceStore {
        NamespaceStore {
            namespaces: HashMap::new(),
        }
    }

    /// Returns the stored namespaces with prefixes.
    pub fn namespaces(&self) -> &HashMap<String, Uri> {
        &self.namespaces
    }

    /// Adds a new namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::namespace::NamespaceStore;
    /// use rdf::namespace::Namespace;
    /// use rdf::uri::Uri;
    ///
    /// let mut nss = NamespaceStore::new();
    ///
    /// let ns = Namespace::new("example".to_string(),
    ///                         Uri::new("http://example.org/".to_string()));
    ///
    /// nss.add(&ns);
    /// ```
    pub fn add(&mut self, ns: &Namespace) {
        &self.namespaces.insert(ns.prefix().clone(), ns.uri.clone());
    }

    /// Returns the URI of a specific namespace.
    ///
    /// # Examples
    ///
    /// ```
    /// use rdf::namespace::NamespaceStore;
    /// use rdf::namespace::Namespace;
    /// use rdf::uri::Uri;
    ///
    /// let mut nss = NamespaceStore::new();
    ///
    /// let ns = Namespace::new("example".to_string(),
    ///                         Uri::new("http://example.org/".to_string()));
    ///
    /// nss.add(&ns);
    ///
    /// assert_eq!(nss.get_uri_by_prefix("example".to_string()).unwrap(),
    ///            &Uri::new("http://example.org/".to_string()))
    /// ```
    pub fn get_uri_by_prefix(&self, prefix: String) -> Result<&Uri> {
        match self.namespaces.get(&prefix) {
            Some(uri) => Ok(uri),
            None => Err(Error::new(
                ErrorType::InvalidNamespace,
                "Namespace does not exists for prefix: ".to_string() + &prefix.to_string(),
            )),
        }
    }
}
