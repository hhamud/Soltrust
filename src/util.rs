use std::{
    fs,
    path::{Components, PathBuf},
};

pub fn read_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            // Maybe the user is too deep in the directory tree.
            // Try skipping some components from the front of the path.
            let path = PathBuf::from(path);
            path_compensating_read(path.components(), 3)
                .map_err(|_| e)
                .expect("failed to read file")
        }
    }
}
