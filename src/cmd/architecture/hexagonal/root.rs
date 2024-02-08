use crate::cmd::{architecture::hexagonal::internal_folders, FileStructure, FolderStructure};

// Root Files
const README_FILE: &str = "README.md";
const README_CONTENT: &str = "
 # Hexagonal Architecture Boilerplate
    ## Introduction

    This is a boilerplate for a hexagonal architecture in Go.

    ## Getting Started

    To get started, clone the repository and run the following command:

    ```bash
    go run cmd/main.go
    ```
    ## License
    This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

    ## Acknowledgments
    - [Hexagonal Architecture](https://en.wikipedia.org/wiki/Hexagonal_architecture_(software))
    - [Go](https://golang.org/)";

const LICENSE_FILE: &str = "LICENSE";
const LICENSE_CONTENT: &str = "MIT License";

const GITIGNORE_FILE: &str = ".gitignore";
const GITIGNORE_CONTENT: &str = "bin
*.exe
*.exe~
*.dll
*.so
*.dylib
";

// Root Folders
const CMD_FOLDER: &str = "cmd";
const INTERNAL_FOLDER: &str = "internal";
const PKG_FOLDER: &str = "pkg";
const TESTDATA_FOLDER: &str = "testdata";

// CMD Folder
const MAIN_FILE: &str = "main.go";
const MAIN_CONTENT: &str = r#"package main

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
                content: MAIN_CONTENT.to_string(),
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
            files: Some(
                vec![
                    FileStructure {
                        file_name: "test.txt".to_string(),
                        content: "This is a test file".to_string(),
                    },
                ]
            )
        },
    ]
}

pub fn root_files() -> Vec<FileStructure> {
    vec![
        FileStructure {
            file_name: README_FILE.to_string(),
            content: README_CONTENT.to_string(),
        },
        FileStructure {
            file_name: LICENSE_FILE.to_string(),
            content: LICENSE_CONTENT.to_string(),
        },
        FileStructure {
            file_name: GITIGNORE_FILE.to_string(),
            content: GITIGNORE_CONTENT.to_string(),
        },
    ]
}
