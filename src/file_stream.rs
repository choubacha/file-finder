use walkdir::{DirEntry, WalkDir};

/// The filestream can produce a handle for receiving incoming files. The filters that are applied
/// are optional but can limit the number of files and depth of search.
pub struct FileStream {
    include_hidden: bool,
}

pub enum Msg {
    File(String),
    EOF,
}

impl FileStream {
    pub fn new() -> FileStream {
        FileStream { include_hidden: false }
    }

    pub fn with_hidden(mut self) -> FileStream {
        self.include_hidden = true;
        self
    }

    pub fn stream(self, f: impl Fn(Msg)) {
        let walker = WalkDir::new(".")
            .into_iter()
            .filter_entry(|e| !self.is_hidden(&e))
            .filter_map(|e| e.ok());
        for entry in walker {
            if entry.file_type().is_file() {
                if let Some(path) = entry.path().to_str() {
                    f(Msg::File(path.to_owned()));
                }
            }
        }
        f(Msg::EOF);
    }

    fn is_hidden(&self, entry: &DirEntry) -> bool {
        if self.include_hidden { return false }

        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with(".") && s != ".")
            .unwrap_or(false)
    }
}
