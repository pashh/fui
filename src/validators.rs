use regex::Regex;
use std::ops::Deref;

pub trait Validator {
    fn validate(&self, data: &str) -> Option<String>;
}

#[derive(Clone, Debug)]
pub struct Required;

impl Validator for Required {
    fn validate(&self, data: &str) -> Option<String> {
        if data.len() == 0 {
            Some("Field is required".to_string())
        } else {
            None
        }
    }
}

/// Ensure if value is one of options
///
/// ```
/// use fui::validators::OneOf;
/// use fui::validators::Validator;
/// let v = OneOf(vec!["a", "b"]);
/// assert_eq!(v.validate("a"), None)
/// ```
#[derive(Clone, Debug)]
pub struct OneOf<T>(pub T);

impl<T> Validator for OneOf<Vec<T>>
where
    T: Deref<Target = str>,
{
    fn validate(&self, data: &str) -> Option<String> {
        if let None = self.0.iter().position(|x| &**x == data) {
            Some("Value must be one of options".to_string())
        } else {
            None
        }
    }
}

impl Validator for Regex {
    fn validate(&self, data: &str) -> Option<String> {
        if self.is_match(data) {
            None
        } else {
            Some(format!(
                "Value {:?} does not match: \"{:?}\" regular exp.",
                data, self
            ))
        }
    }
}
