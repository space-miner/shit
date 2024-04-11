use std::{fs::Metadata, os::unix::fs::PermissionsExt, path::{Path, PathBuf}};

use crate::utils::get_root_path;

#[derive(Debug)]
pub struct Entry {
    pub filename: String,
    pub object_id: String,
    pub stat: Metadata,
}

impl Entry {
    pub fn new(filename: String, object_id: &str, stat: Metadata) -> Self {
        Entry {
            filename,
            object_id: object_id.to_string(),
            stat,
        }
    }

    pub fn mode(&self) -> String {
        let permissions = self.stat.permissions();
        // Check if owner has executable permission set.
        if permissions.mode() & 0o100 != 0 {
            String::from("100755")
        } else {
            String::from("100644")
        }
    }

    pub fn parent_directories(&self) -> Vec<PathBuf>  {
        let mut parents = Vec::new(); 
        let path = Path::new(&self.filename);
        while path != get_root_path() {
            let parent_opt = path.parent();
            if parent_opt.is_some() {
                let parent_path = parent_opt.unwrap().to_path_buf();
                parents.push(parent_path);
            }
        }
        parents
    }
}
