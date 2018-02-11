use regex::Regex;
use std::ops::Deref;
use std::path::Path;

pub trait Validator {
    fn validate(&self, data: &str) -> Option<String>;
}

#[derive(Clone, Debug)]
/// Ensure data is included
///
/// Examples
///
/// ```
/// use fui::validators::Required;
/// use fui::validators::Validator;
///
/// assert_eq!(Required.validate("some-data"), None);
/// assert_eq!(Required.validate(""), Some("Field is required".to_string()));
/// ```
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

/// Ensure data is dir path which exists
///
/// Examples
///
/// ```
/// extern crate fui;
/// extern crate tempdir;
///
/// use fui::validators::DirExists;
/// use fui::validators::Validator;
/// use tempdir::TempDir;
/// use std::fs::File;
///
/// # fn main() {
/// let existing_dir = TempDir::new("fui").unwrap();
/// let existing_file = existing_dir.path().join("some-file");
/// File::create(&existing_file).unwrap();
/// let missing_dir = existing_dir.path().join("missing-dir");
///
/// assert_eq!(DirExists.validate(existing_dir.path().to_str().unwrap()), None);
/// assert_eq!(DirExists.validate(existing_file.to_str().unwrap()), Some("It's not a dir".to_string()));
/// assert_eq!(DirExists.validate(missing_dir.to_str().unwrap()).unwrap(), "Dir doesn't exist");
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct DirExists;

impl Validator for DirExists {
    fn validate(&self, data: &str) -> Option<String> {
        let path = Path::new(data);
        if path.exists() {
            if path.metadata().unwrap().is_dir() {
                None
            } else {
                Some("It's not a dir".to_string())
            }
        } else {
            Some("Dir doesn't exist".to_string())
        }
    }
}

/// Ensure data is file path which exists
///
/// Examples
///
/// ```
/// extern crate fui;
/// extern crate tempdir;
///
/// use fui::validators::FileExists;
/// use fui::validators::Validator;
/// use tempdir::TempDir;
/// use std::fs::File;
///
/// # fn main() {
/// let existing_dir = TempDir::new("fui").unwrap();
/// let existing_file = existing_dir.path().join("some-file");
/// File::create(&existing_file).unwrap();
/// let missing_file = existing_dir.path().join("missing-file");
///
/// assert_eq!(FileExists.validate(existing_file.to_str().unwrap()), None);
/// assert_eq!(FileExists.validate(missing_file.to_str().unwrap()), Some("File doesn't exist".to_string()));
/// assert_eq!(FileExists.validate(existing_dir.path().to_str().unwrap()), Some("It's not a file".to_string()));
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct FileExists;

impl Validator for FileExists {
    fn validate(&self, data: &str) -> Option<String> {
        let path = Path::new(data);
        if path.exists() {
            if path.metadata().unwrap().is_file() {
                None
            } else {
                Some("It's not a file".to_string())
            }
        } else {
            Some("File doesn't exist".to_string())
        }
    }
}

/// Ensure if value is one of options
///
/// Examples
///
/// ```
/// use fui::validators::OneOf;
/// use fui::validators::Validator;
///
/// let v = OneOf(vec!["a", "b"]);
/// assert_eq!(v.validate("a"), None);
/// assert_eq!(v.validate("xxx"), Some("Value must be one of options".to_string()));
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
