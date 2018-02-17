// Partially reflected `tar` command with these actions:
// * Create an archive from files
// * Extract an archive in a target folder
// * List the contents of a tar file

extern crate fui;

use fui::feeders::DirItems;
use fui::fields::{Autocomplete, Text};
use fui::form::FormView;
use fui::utils::cwd;
use fui::validators::{FileExists, OneOf, Required};
use fui::{Fui, Value};

fn hdlr(v: Value) {
    println!("user input (from hdlr) {:?}", v);
}

fn main() {
    let formats = vec!["none", "gzip", "bzip2"];
    let compression = Autocomplete::new("file_to_archive", formats.clone())
        .initial("gzip")
        .validator(Required)
        .validator(OneOf(formats))
        .help("Archive format");

    Fui::new()
        .action(
            "ARCHIVE-FILES: Create an archive from files",
            FormView::new()
                .field(
                    Autocomplete::new("file-to-archive", DirItems::new())
                        .help("Files which should be archived")
                        //TODO: .multi(true)
                        .validator(Required)
                        .validator(FileExists),
                )
                .field(
                    Text::new("target")
                        .help("Name of archive file")
                        // TODO: PathFree?
                        .validator(Required),
                )
                .field(compression.clone()),
            hdlr,
        )
        .action(
            "EXTRACT-TO-DIR: Extract an archive in a target folder",
            FormView::new()
                .field(
                    Autocomplete::new("archive-path", DirItems::new())
                        .help("Path to compressed file")
                        .validator(Required)
                        .validator(FileExists),
                )
                .field(
                    Autocomplete::new("dst-dir", DirItems::dirs())
                        .initial(cwd())
                        .help("Dir where extracted files should land")
                        .validator(Required),
                )
                .field(compression.clone()),
            hdlr,
        )
        .action(
            "LIST-ARCHIVE: List the contents of a tar file",
            FormView::new()
                .field(
                    Autocomplete::new("archive-file", DirItems::new())
                        .help("Path to archive")
                        .validator(FileExists),
                )
                .field(compression),
            hdlr,
        )
        .run();
}
