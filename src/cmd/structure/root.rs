use crate::cmd::{FileStructure, FolderStructure};

const MAIN_FILE: &str = "main.go";
const README_FILE: &str = "README.md";
const LICENSE_FILE: &str = "LICENSE";
const GITIGNORE_FILE: &str = ".gitignore";

const CMD_FOLDER: &str = "cmd";
const INTERNAL_FOLDER: &str = "internal";
const PKG_FOLDER: &str = "pkg";
const TESTDATA_FOLDER: &str = "testdata";

const CONTENT: &str = r#"package main

import "fmt"

func main() {
    fmt.Println("Welcome to {}!")
}
"#;

pub fn root_folders() -> Vec<FolderStructure> {
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
            sub_folders: None,
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
