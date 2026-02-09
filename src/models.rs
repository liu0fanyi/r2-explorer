use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PathType {
    Dir,
    File,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DufsItem {
    pub name: String,
    pub path_type: PathType,
    pub size: Option<u64>,
    pub mtime: u64,
}
