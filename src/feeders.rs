use std::env::current_dir;
use std::fmt::Display;
use std::path::PathBuf;
use std::rc::Rc;

use walkdir::WalkDir;

// TODO: this should be replaced with regular Iterator?
pub trait Feeder: 'static {
    fn query(&self, text: &str, position: isize, items_count: usize) -> Vec<String>;
}

#[derive(Clone, Debug)]
enum FileType {
    File,
    Dir,
    All,
}

#[derive(Clone, Debug)]
pub struct DirItems {
    working_dir: PathBuf,
    kind: FileType,
}

impl DirItems {
    pub fn current_dir() -> Self {
        DirItems {
            working_dir: current_dir().expect("Can't get current dir"),
            kind: FileType::All,
        }
    }

    pub fn dir(path: PathBuf) -> Self {
        DirItems {
            working_dir: path,
            kind: FileType::All,
        }
    }

    pub fn dirs(mut self) -> DirItems {
        self.kind = FileType::Dir;
        self
    }

    pub fn files(mut self) -> DirItems {
        self.kind = FileType::File;
        self
    }
}

impl Feeder for DirItems {
    fn query(&self, text: &str, _position: isize, items_count: usize) -> Vec<String> {
        WalkDir::new(&self.working_dir)
            .into_iter()
            .filter(|x| {
                let path = x.as_ref().unwrap().path();
                match self.kind {
                    FileType::File => path.is_file(),
                    FileType::Dir => path.is_dir(),
                    FileType::All => true,
                }
            })
            .filter(|x| {
                let path = x.as_ref().unwrap().path();
                path.to_str().unwrap().to_lowercase().contains(text)
            })
            .map(|x| format!("{}", x.unwrap().path().display()))
            .take(items_count)
            .collect()
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
