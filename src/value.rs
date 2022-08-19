use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub value: Vec<u8>,
}

impl Value {
    pub fn new(value: Vec<u8>) -> Self {
        Self { value }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(")?;
        if !self.value.is_empty() {
            let mut first = true;
            for v in self.value.iter() {
                if !first {
                    write!(f, " | ")?;
                } else {
                    first = false;
                }
                write!(f, "{}", v)?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl Default for Value {
    fn default() -> Self {
        Self { value: vec![] }
    }
}
