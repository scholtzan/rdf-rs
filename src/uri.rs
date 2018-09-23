// todo: implement

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Uri {
    uri: String,
}

// todo: look into using servo URI
impl Uri {
    pub fn new(uri: String) -> Uri {
        Uri { uri }
    }

    /// Returns the string representation of the URI.
    pub fn to_string(&self) -> &String {
        &self.uri
    }

    /// todo
    pub fn append_resource_path(&mut self, path: &str) {
        // todo: check if URI ends with '/', if not add '/'
        self.uri.push_str(&path.to_string());
    }
}
