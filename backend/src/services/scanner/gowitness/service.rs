use std::path::{Path, PathBuf};

pub struct GowitnessService {
    scans_dir: PathBuf,
}




impl GowitnessService {
    pub fn new(scans_dir: impl AsRef<Path>) -> Self {
        Self { scans_dir: scans_dir.as_ref().to_path_buf() }
    }
}


