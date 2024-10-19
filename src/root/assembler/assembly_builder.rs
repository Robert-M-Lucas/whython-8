pub type Assembly = String;
pub type AssemblyRef<'a> = &'a str;

/// A helper for correctly formatting strings for assembly.
/// Will have no preceding newline and a trailing newline.
pub struct AssemblyBuilder {
    inner: Assembly,
}

impl Default for AssemblyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AssemblyBuilder {
    pub fn new() -> AssemblyBuilder {
        AssemblyBuilder {
            inner: String::new(),
        }
    }

    /// Adds a single, indented line with a trailing newline
    pub fn line(&mut self, line: AssemblyRef) {
        self.inner += "    ";
        self.inner += line;
        self.inner.push('\n');
    }

    /// Adds a single, non-indented line with a trailing newline
    pub fn toplevel(&mut self, line: AssemblyRef) {
        self.inner += line;
        self.inner.push('\n');
    }

    /// Adds the output from another `AssemblyBuilder`.
    /// Adds no indentation and assumes `other` has its own trailing newline
    pub fn other(&mut self, other: AssemblyRef) {
        self.inner += other;
    }

    /// Returns the internal string - no preceding newline with a trailing newline
    pub fn finish(self) -> Assembly {
        self.inner
    }
}
