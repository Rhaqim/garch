pub mod hexagonal;

pub use hexagonal::root::{hex_root_files, hex_root_folders};

use super::{FileStructure, FolderStructure};

// Root Files
const README_FILE: &str = "README.md";
const README_CONTENT: &str = "
# Hexagonal Architecture Boilerplate

## Introduction

This is a boilerplate for a hexagonal architecture in Go was generated using Garch.

## Getting Started

To get started, run the following command:

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

// Main File
const MAIN_FILE: &str = "main.go";
const MAIN_CONTENT: &str = r#"package main

import "fmt"

func main() {
    fmt.Println("Welcome to {}!")
}
"#;

pub fn general_main_file() -> FileStructure {
    FileStructure {
        file_name: MAIN_FILE.to_string(),
        content: MAIN_CONTENT.to_string(),
    }
}

pub fn general_root_files() -> Vec<FileStructure> {
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


pub fn architecture_map() -> Vec<(&'static str, (Vec<FolderStructure>, Vec<FileStructure>))> {
    vec![
        ("hexagonal", (hex_root_folders(), hex_root_files())),
        ("clean", (vec![], vec![])),
        ("onion", (vec![], vec![])),
        ("layered", (vec![], vec![])),
    ]
}
