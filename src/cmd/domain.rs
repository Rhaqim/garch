pub mod garch_cmd_domain {
    use std::io;

    use crate::core::cli::garch_cli::ProjectConfig;

    #[derive(Debug, Clone)]
    pub struct BoilerplateStructure {
        pub project_title: String,
        pub username: String,
        pub folders: Option<Vec<FolderStructure>>,
        pub files: Option<Vec<FileStructure>>,
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
        fn generate_recursive(&self, items: &Option<Vec<FolderStructure>>) -> io::Result<()>;
        fn create_folder(&self, folder_name: &str) -> io::Result<()>;
        fn change_directory(&self, folder_name: &str) -> io::Result<()>;
        fn create_file(&self, file_name: &str, content: &str) -> io::Result<()>;
        fn run_git_init(&self) -> io::Result<()>;
        fn run_go_init(&self) -> io::Result<()>;
        fn delete_folder(&self, folder_name: &str) -> io::Result<()>;
    }
}