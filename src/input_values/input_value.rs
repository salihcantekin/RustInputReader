use std::fmt::{self};

pub struct InputValue<T> {
    pub value: Option<T>,
}

impl<T> InputValue<T> {
    pub(crate) fn new(value: Option<T>) -> Self {
        Self { value }
    }
}
impl<T: fmt::Debug> fmt::Debug for InputValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}", &self.value)
    }
}

impl<T: fmt::Display> fmt::Display for InputValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.value {
            Some(val) => write!(f, "{}", val),
            None => write!(f, "None"),
        }
    }
}