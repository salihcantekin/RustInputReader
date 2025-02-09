use crate::{input_values::input_value::InputValue, validators::validator::Validator, LengthValidator};

use super::generic_input_reader::GenericInputReader;

pub struct BoolInputReader {
    reader: GenericInputReader<bool>,
    pre_validators: Vec<Box<dyn Validator>>,
}

impl BoolInputReader {
    
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
    
    pub(crate) fn read(&self) -> InputValue<bool> {
        self.reader.read()
    }
}