use node::Node;

/// Represents a pattern in the `WHERE` clauses
pub trait Pattern {}

/// Describes a group of triples the SPARQL `WHERE` clause should match.
pub struct GroupPattern {
    patterns: Vec<Box<Pattern>>,
    is_union: bool,
    is_optional: bool,
}

impl Pattern for GroupPattern {}

impl GroupPattern {
    /// Constructor for `GroupPattern`
    ///
    /// # Examples
    ///
    /// todo
    ///
    pub fn new() -> GroupPattern {
        GroupPattern {
            patterns: Vec::new(),
            is_union: false,
            is_optional: false,
        }
    }

    /// Store that the group pattern is preceded by `UNION`.
    ///
    /// # Examples
    ///
    /// todo
    ///
    pub fn set_is_union(&mut self) {
        self.is_union = true;
    }

    /// Store that the group pattern is preceded by `OPTIONAL`.
    ///
    /// # Examples
    ///
    /// todo
    ///
    pub fn set_is_optional(&mut self) {
        self.is_optional = true;
    }

    /// Adds a new pattern to the group.
    ///
    /// # Examples
    ///
    /// todo
    ///
    pub fn add_pattern(&mut self, pattern: Box<Pattern>) {
        self.patterns.push(pattern);
    }
}

/// Describes a triple that should be matched in a SPARQL `WHERE` clause.
pub struct TriplePattern {
    subject: NodePattern,
    predicate: NodePattern,
    object: NodePattern,
    is_union: bool,
    is_optional: bool,
}

impl Pattern for TriplePattern {}

impl TriplePattern {
    /// Constructor of `TriplePattern`.
    ///
    /// todo
    ///
    pub fn new(
        subject: &NodePattern,
        predicate: &NodePattern,
        object: &NodePattern,
    ) -> TriplePattern {
        TriplePattern {
            subject: subject.clone(),
            predicate: predicate.clone(),
            object: object.clone(),
            is_optional: false,
            is_union: false,
        }
    }
}

/// Describes nodes in a `TriplePattern` which can either be variables or nodes with specific values.
#[derive(Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum NodePattern {
    VariableNode(String), // variable
    FixedNode(Node),      // node that has a specific value
}

impl Pattern for NodePattern {}

// @todo: implement filter pattern
// filters should be applied to graphs
pub struct FilterPattern {}
