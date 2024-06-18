
pub struct AssemblyBuilder {
    inner: String
}

impl AssemblyBuilder {
    pub fn new() -> AssemblyBuilder {
        AssemblyBuilder { inner: String::new() }
    }

    pub fn line(&mut self, line: &str) {
        self.inner += "    ";
        self.inner += line;
        self.inner.push('\n');
    }

    pub fn toplevel(&mut self, line: &str) {
        self.inner += line;
        self.inner.push('\n');
    }

    pub fn other(&mut self, other: &str) {
        self.inner += other;
    }

    pub fn finish(self) -> String {
        self.inner
    }
}