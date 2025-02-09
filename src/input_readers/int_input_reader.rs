use std::ops::Deref;

use super::generic_input_reader::GenericInputReader;
use crate::{
    input_values::input_value::InputValue, validators::validator::Validator, InputReaderExtensions, LengthValidator
};


impl InputReaderExtensions<i32> for IntInputReader {
    fn read(&self) -> InputValue<i32> {
        self.reader.read()
    }
}

pub(crate) struct IntInputReader {
    reader: GenericInputReader<i32>,
    pre_validators: Vec<Box<dyn Validator>>,
}

impl IntInputReader {
    pub(crate) fn new() -> Self {
        let reader = Self {
            reader: GenericInputReader::new(),
            pre_validators: vec![Box::new(LengthValidator)],
        };
        reader
    }

    pub(crate) fn with_message(mut self, message: &str) -> Self {
        self.reader = self.reader.with_message(message);
        self
    }

    pub(crate) fn with_error_message(mut self, error_message: &str) -> Self {
        self.reader = self.reader.with_error_message(error_message);
        self
    }

    // pub(crate) fn read(&self) -> InputValue<i32> {
    //     self.reader.read()
    // }
}
