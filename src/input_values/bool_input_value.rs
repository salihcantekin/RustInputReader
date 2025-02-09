use super::input_value::InputValue;

pub trait BoolInputExtensions {
    fn is_true(&self) -> bool;
    fn is_false(&self) -> bool;
}

impl BoolInputExtensions for InputValue<bool> {
    fn is_true(&self) -> bool {
        self.value == Some(true)
    }
    fn is_false(&self) -> bool {
        self.value == Some(false)
    }
}