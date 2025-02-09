use super::input_value::InputValue;

pub trait IntInputExtensions {
    fn is_zero(&self) -> bool;
    fn is_negative(&self) -> bool;
    fn is(&self, value: i32) -> bool ;
}

impl IntInputExtensions for InputValue<i32> {
    fn is_zero(&self) -> bool {
        self.value == Some(0)
    }
    
    fn is_negative(&self) -> bool {
        self.value.map_or(false, |v| v < 0)
    }
    
    fn is(&self, value: i32) -> bool  {
        self.value == Some(value)
    }
}