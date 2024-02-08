pub mod hexagonal;

pub use hexagonal::root::{root_files, root_folders};

use super::{FileStructure, FolderStructure};

pub fn architecture_map() -> Vec<(&'static str, (Vec<FolderStructure>, Vec<FileStructure>))> {
    vec![
        ("hexagonal", (root_folders(), root_files())),
        ("clean", (vec![], vec![])),
        ("onion", (vec![], vec![])),
        ("layered", (vec![], vec![])),
    ]
}
