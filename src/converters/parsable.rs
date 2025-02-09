use std::str::FromStr;

pub trait Parsable: FromStr {
    fn parse(input: &str) -> Option<Self>
        where
            Self: Sized,
    {
        input.parse::<Self>().ok()
    }
}