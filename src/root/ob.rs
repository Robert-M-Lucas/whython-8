/// Represents data that may be owned or borrowed
pub enum OB<'a, T> {
    Owned(T),
    Borrowed(&'a T)
}

impl<'a, T> OB<'a, T> {
    pub fn o(o: T) -> OB<'a, T> {
        OB::Owned(o)
    }

    pub fn b(b: &'a T) -> OB<'a, T> {
        OB::Borrowed(b)
    }

    pub fn get(&self) -> &T {
        match &self {
            OB::Owned(o) => o,
            OB::Borrowed(b) => b
        }
    }
}

