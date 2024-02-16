pub mod architecture;
pub mod boilerplate;
pub mod commands;
pub mod database;
pub mod domain;
pub mod files_folders;

pub use domain::garch_cmd_domain::{
    Boilerplate, BoilerplateStructure, FileStructure, FolderStructure,
};

pub use files_folders::garch_files_folders::{
    change_directory, create_file, create_folder, generate_recursive,
};

pub use commands::garch_cli_commands::{add_database, run_git_init, run_go_init};
