
/// A helper for correctly formatting strings for assembly.
/// Will have no preceding newline and a trailing newline.
pub struct AssemblyBuilder {
    inner: String
}

impl Default for AssemblyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AssemblyBuilder {
    pub fn new() -> AssemblyBuilder {
        AssemblyBuilder { inner: String::new() }
    }

    /// Adds a single, indented line with a trailing newline
    pub fn line(&mut self, line: &str) {
        self.inner += "    ";
        self.inner += line;
        self.inner.push('\n');
    }

    /// Adds a single, non-indented line with a trailing newline
    pub fn toplevel(&mut self, line: &str) {
        self.inner += line;
        self.inner.push('\n');
    }

    /// Adds the output from another `AssemblyBuilder`.
    /// Adds no indentation and assumes `other` has its own trailing newline
    pub fn other(&mut self, other: &str) {
        self.inner += other;
    }

    /// Returns the internal string - no preceding newline with a trailing newline
    pub fn finish(self) -> String {
        self.inner
    }
}