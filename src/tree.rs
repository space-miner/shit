use std::collections::HashMap;
use std::path::PathBuf;

use crate::blob::Kind;
use crate::entry::Entry;
use crate::traits::Object;
use crate::utils::get_root_path;

#[derive(Debug)]
pub struct Tree {
    pub entries: Vec<Entry>,
    pub kind: Kind,
    pub object_id: String,
}

#[derive(Debug)]
pub struct TreeNode {
    pub entries: HashMap<PathBuf, TreeNode>
}

impl TreeNode {
    pub fn new() -> Self {
        todo!("unimplemented")
    }

    pub fn add_entry(parent_directories: Vec<PathBuf>, entry: Entry) {
        todo!("unimplemented")
    }

}

impl Tree {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self {
            entries,
            kind: Kind::Tree,
            object_id: String::new(),
        }
    }

    pub fn build(entries: Vec<Entry>) -> TreeNode {
        entries.sort_by(|a, b| a.filename.cmp(&b.filename));
        let mut root = TreeNode::new();
        for entry in entries {
            let cwd = get_root_path();
            root.add_entry(entry.parent_directories(), entry);
        }
        root
    }
}

impl Object for Tree {
    fn to_string(&self) -> String {
        let kind = format!("{:?}", self.kind).to_lowercase();
        let mut content = String::new();
        for entry in &self.entries {
            content.push_str(&format!(
                "{} {}\0{}",
                entry.mode(),
                entry.filename,
                entry.object_id
            ))
        }
        // metadata + content
        format!("{} {}\0{}", kind, content.bytes().len(), content)
    }

    fn get_object_id(&self) -> String {
        self.object_id.clone()
    }

    fn set_object_id(&mut self, object_id: String) {
        self.object_id = object_id;
    }
}
