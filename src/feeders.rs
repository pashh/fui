use glob::glob;
use std::env;
use std::fmt::Display;
use std::rc::Rc;

// TODO: this should be replaced with regular Iterator?
pub trait Feeder: 'static {
    fn query(&self, text: &str, position: isize, items_count: usize) -> Vec<String>;
}

#[derive(Clone, Debug)]
enum DirItemType {
    Dir,
    All,
}

#[derive(Clone, Debug)]
pub struct DirItems {
    dir_item_type: DirItemType,
}

impl DirItems {
    pub fn new() -> Self {
        DirItems {
            dir_item_type: DirItemType::All,
        }
    }
    pub fn dirs() -> Self {
        DirItems {
            dir_item_type: DirItemType::Dir,
        }
    }
}

impl Feeder for DirItems {
    // TODO: text: Option<&str>?
    fn query(&self, text: &str, _position: isize, items_count: usize) -> Vec<String> {
        let path = if text == "" {
            format!("./")
        } else if text.starts_with('~') {
            let path = text.replace("~", env::home_dir().unwrap().to_str().unwrap());
            format!("{}", path)
        } else {
            format!("{}", text)
        };
        let path =
            //TODO: replace current solution with:
            // like /home/user/xxx -> /home/user/*xxx*
            // like /home/user/*xxx -> /home/user/*xxx
            // like /home/user/xxx* -> /home/user/xxx*
            // like **/xxx -> **/*xxx*
            // like **/*xxx -> **/*xxx
            // like **/xxx* -> **/xxx*
            if path.contains("*") {
                path
            } else {
                format!("{}*", path)
            };
        //TODO: use https://doc.rust-lang.org/glob/glob/struct.MatchOptions.html
        // to smart-case

        if let Ok(v) = glob(&path) {
            v.filter(|x| {
                if let Err(e) = x.as_ref() {
                    eprintln!("{:?}", e);
                    false
                } else {
                    true
                }
            }).filter(|x| {
                    let path = x.as_ref().unwrap().metadata().unwrap();
                    match self.dir_item_type {
                        DirItemType::Dir => path.is_dir(),
                        DirItemType::All => true,
                    }
                })
                .map(|x| {
                    let path = x.unwrap();
                    // TODO: show full path as option
                    // use std::fs;
                    // let full_path = fs::canonicalize(path);
                    let full_path = Some(path);
                    let text = format!("{}", full_path.unwrap().display());
                    text
                })
                .skip(_position as usize)
                .take(items_count)
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::fs;
    use std::iter::FromIterator;

    fn expected(start: &str) -> HashSet<String> {
        let found = {
            if let Ok(v) = fs::read_dir(start) {
                v.map(|x| {
                    let p = format!("{}", x.as_ref().unwrap().path().display());
                    p.replace("./", "")
                }).collect()
            } else {
                Vec::new()
            }
        };
        HashSet::<String>::from_iter(found)
    }

    #[test]
    fn test_dir_item_works_with_current_dir() {
        let di = DirItems::new();
        let found = di.query("", 0, 100);
        assert_eq!(HashSet::<String>::from_iter(found), expected("./"));
    }

    #[test]
    fn test_dir_item_works_with_current_subdir() {
        let di = DirItems::new();
        let found = di.query("examples/", 0, 100);
        assert_eq!(HashSet::<String>::from_iter(found), expected("./examples"));
    }

    #[test]
    fn test_dir_item_works_with_current_missing_dir() {
        let di = DirItems::new();
        let found = di.query("missing-dir", 0, 10);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            expected("./missing-dir")
        );
    }

    #[test]
    fn test_dir_item_works_with_homedir() {
        let di = DirItems::new();
        let found = di.query("~/", 0, 200);
        let homedir = env::home_dir().unwrap();
        assert_eq!(
            HashSet::<String>::from_iter(found),
            expected(homedir.to_str().unwrap())
        );
    }

    #[test]
    fn test_dir_item_works_with_root_dir() {
        let di = DirItems::new();
        let found = di.query("/root", 0, 100);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::from_iter(vec!["/root".to_string()])
        );
    }

    #[test]
    fn test_dir_item_works_with_root_subdir() {
        let di = DirItems::new();
        let found = di.query("/root/", 0, 100);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::new()
        );
    }

    #[test]
    fn test_dir_item_works_with_top_missing_dir() {
        let di = DirItems::new();
        let found = di.query("/missing-dir", 0, 10);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::new()
        );
    }

    #[test]
    fn test_dir_item_works_with_broken_glob() {
        let di = DirItems::new();
        let found = di.query("**.", 0, 10);
        assert_eq!(
            HashSet::<String>::from_iter(found),
            HashSet::<String>::new()
        );
    }
}

impl<T: Display + 'static> Feeder for Vec<T> {
    fn query(&self, text: &str, _position: isize, items_count: usize) -> Vec<String> {
        self.iter()
            .map(|x| format!("{}", x))
            .filter(|x| x.to_lowercase().contains(text))
            .take(items_count)
            .collect()
    }
}

impl Feeder for Rc<Feeder> {
    fn query(&self, text: &str, position: isize, items_count: usize) -> Vec<String> {
        (**self).query(text, position, items_count)
    }
}
