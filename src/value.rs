use std::fmt;
use std::ops::{Add, Sub, Mul};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

impl Into<bool> for Value {
    fn into(self) -> bool {
        !self.value.is_empty() && self.value.iter().all(|&v| v != 0)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::new(
            if value {
                vec![1]
            } else {
                vec![0]
            }
        )
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.value.iter().zip(rhs.value.iter())
                .map(|(x, y)| x + y)
                .collect()
        )
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(
            self.value.iter().zip(rhs.value.iter())
                .map(|(x, y)| x - y)
                .collect()
        )
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.value.iter().zip(rhs.value.iter())
                .map(|(x, y)| x * y)
                .collect()
        )
    }
}
