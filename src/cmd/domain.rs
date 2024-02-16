pub mod garch_cmd_domain {
    use std::io;

    use crate::core::cli::garch_cli::ProjectConfig;

    #[derive(Debug, Clone)]
    pub struct BoilerplateStructure {
        pub project_title: String,
        pub username: String,
        pub folders: Option<Vec<FolderStructure>>,
        pub files: Option<Vec<FileStructure>>,
        pub db_type: Option<String>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FolderStructure {
        pub folder_title: String,
        pub sub_folders: Option<Vec<FolderStructure>>,
        pub files: Option<Vec<FileStructure>>,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct FileStructure {
        pub file_name: String,
        pub content: String,
    }

    pub trait Boilerplate {
        fn new(project: &ProjectConfig) -> Self;
        fn generate(&self) -> io::Result<()>;
    }
}
