use crate::cmd::{
    architecture::{
        hexagonal::internal_folders,
        {general_main_file, general_root_files},
    },
    FileStructure, FolderStructure,
};

// Root Folders
const CMD_FOLDER: &str = "cmd";
const INTERNAL_FOLDER: &str = "internal";
const PKG_FOLDER: &str = "pkg";
const TESTDATA_FOLDER: &str = "testdata";

// Hexagonal Architecture
pub fn hex_root_folders() -> Vec<FolderStructure> {
    let internal_folders = internal_folders();

    vec![
        FolderStructure {
            folder_title: CMD_FOLDER.to_string(),
            sub_folders: None,
            files: Some(vec![general_main_file()]),
        },
        FolderStructure {
            folder_title: INTERNAL_FOLDER.to_string(),
            sub_folders: Some(internal_folders),
            files: None,
        },
        FolderStructure {
            folder_title: PKG_FOLDER.to_string(),
            sub_folders: None,
            files: None,
        },
        FolderStructure {
            folder_title: TESTDATA_FOLDER.to_string(),
            sub_folders: None,
            files: Some(vec![FileStructure {
                file_name: "test.txt".to_string(),
                content: "This is a test file".to_string(),
            }]),
        },
    ]
}

pub fn hex_root_files() -> Vec<FileStructure> {
    //append the general root files to the hexagonal root files
    let root_files = general_root_files();
    root_files
}
