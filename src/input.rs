use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Input {
    pub value: String,
}

impl Input {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(Self { value: contents })
    }
}
