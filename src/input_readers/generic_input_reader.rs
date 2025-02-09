use std::num::NonZero;

use crossterm::style::Color;

use crate::{
    converters::parsable::Parsable,
    input_values::input_value::InputValue,
    user_io::{
        console_reader::ConsoleReader, console_writer::ConsoleWriter,
        default_console::DefaultConsole,
    },
    validators::validator::{BaseValidator, Validator},
};

pub(crate) struct GenericInputReader<T: Parsable> {
    message: Option<String>,
    error_message: Option<String>,
    console_writer: Box<dyn ConsoleWriter>,
    console_reader: Box<dyn ConsoleReader>,
    pre_validators: Vec<Box<dyn Validator>>,

    _phantom: std::marker::PhantomData<T>,
}

impl<T: Parsable> GenericInputReader<T> {
    pub(crate) fn new() -> Self {
        Self {
            message: None,
            error_message: None,
            console_writer: Box::new(DefaultConsole),
            console_reader: Box::new(DefaultConsole),
            pre_validators: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub(crate) fn with_message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }

    pub(crate) fn with_error_message(mut self, error_message: &str) -> Self {
        self.error_message = Some(error_message.to_string());
        self
    }

    pub(crate) fn read(&self) -> InputValue<T> {
        if let Some(ref message) = self.message {
            self.console_writer.write(message);
        }

        let input = self.console_reader.read_line();
        let parsed_value = self.validate_and_parse(&input);

        if parsed_value.is_none() {
            if let Some(ref error_message) = self.error_message {
                self.console_writer.writeln_with_color(error_message, Color::Red);
            }
        }

        InputValue::new(parsed_value)
    }

    fn validate_and_parse(&self, input: &str) -> Option<T> {
        for validator in &self.pre_validators {
            if !validator.validate(input) {
                return None;
            }
        }

        T::parse(input)
    }

    pub(crate) fn with_prevalidator(mut self, validator: Box<dyn Validator>) -> Self {
        self.pre_validators.push(validator);
        self
    }

    pub(crate) fn prevalidate<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&str) -> bool + 'static,
    {
        let validator = BaseValidator::new(predicate);
        self.pre_validators.push(Box::new(validator));
        self
    }
}

pub(crate) trait InputReaderExtensions<T: Parsable> {
    fn read_until<F>(&self, predicate: F) -> InputValue<T>
    where
        F: Fn(&InputValue<T>) -> bool,
    {
        loop {
            let v = self.read();

            if predicate(&v) {
                return v;
            }
        }
    }

    fn read_until_valid(&self) -> InputValue<T> {
        self.read_until(|i| i.value.is_some())
    }

    fn read(&self) -> InputValue<T>;
}

impl<T: Parsable> InputReaderExtensions<T> for GenericInputReader<T> {
    fn read(&self) -> InputValue<T> {
        GenericInputReader::read(self)
    }
}
