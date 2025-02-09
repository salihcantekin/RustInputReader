pub trait Validator {
    fn validate(&self, input: &str) -> bool;
}

pub struct BaseValidator<F>
where
    F: Fn(&str) -> bool,
{
    validation_fn: F,
}

impl<F> Validator for BaseValidator<F>
where
    F: Fn(&str) -> bool,
{
    fn validate(&self, input: &str) -> bool {
        (self.validation_fn)(input)
    }
}

impl<F> BaseValidator<F>
where
    F: Fn(&str) -> bool,
{
    pub fn new(validation_fn: F) -> Self {
        BaseValidator { validation_fn }
    }
}