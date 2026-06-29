use alloc::{string::String, vec::Vec};
use lazy_static::lazy_static;
use spin::Mutex;

pub struct FileSystem {
    files: Vec<(String, Vec<u8>)>,
}

impl FileSystem {
    const fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn create(&mut self, path: &str, data: Vec<u8>) {
        if let Some(entry) = self.files.iter_mut().find(|(p, _)| p == path) {
            entry.1 = data;
        } else {
            self.files.push((String::from(path), data));
        }
    }

    pub fn read(&self, path: &str) -> Option<&[u8]> {
        self.files
            .iter()
            .find(|(p, _)| p == path)
            .map(|(_, data)| data.as_slice())
    }

    pub fn delete(&mut self, path: &str) -> bool {
        if let Some(pos) = self.files.iter().position(|(p, _)| p == path) {
            self.files.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn list(&self) -> impl Iterator<Item = &str> {
        self.files.iter().map(|(p, _)| p.as_str())
    }
}

lazy_static! {
    pub static ref FS: Mutex<FileSystem> = Mutex::new(FileSystem::new());
}
