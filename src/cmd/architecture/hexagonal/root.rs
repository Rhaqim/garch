use crate::cmd::{architecture::hexagonal::internal_folders, FileStructure, FolderStructure};

// Root Files
const README_FILE: &str = "README.md";
const LICENSE_FILE: &str = "LICENSE";
const GITIGNORE_FILE: &str = ".gitignore";

// Root Folders
const CMD_FOLDER: &str = "cmd";
const INTERNAL_FOLDER: &str = "internal";
const PKG_FOLDER: &str = "pkg";
const TESTDATA_FOLDER: &str = "testdata";

// CMD Folder
const MAIN_FILE: &str = "main.go";
const CONTENT: &str = r#"package main

import "fmt"

func main() {
    fmt.Println("Welcome to {}!")
}
"#;

// Hexagonal Architecture
pub fn root_folders() -> Vec<FolderStructure> {
    let internal_folders = internal_folders();

    vec![
        FolderStructure {
            folder_title: CMD_FOLDER.to_string(),
            sub_folders: None,
            files: Some(vec![FileStructure {
                file_name: MAIN_FILE.to_string(),
                content: CONTENT.to_string(),
            }]),
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
            files: None,
        },
    ]
}

pub fn root_files() -> Vec<FileStructure> {
    vec![
        FileStructure {
            file_name: README_FILE.to_string(),
            content: "".to_string(),
        },
        FileStructure {
            file_name: LICENSE_FILE.to_string(),
            content: "".to_string(),
        },
        FileStructure {
            file_name: GITIGNORE_FILE.to_string(),
            content: "".to_string(),
        },
    ]
}
