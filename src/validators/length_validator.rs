use super::validator::Validator;

pub(crate) struct LengthValidator;

impl Validator for LengthValidator {
     fn validate(&self, input: &str) -> bool {
        !input.trim().is_empty()
    }
}