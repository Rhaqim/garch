pub mod boilerplate;
pub mod domain;
pub mod architecture;
pub mod database;

pub use domain::garch_cmd_domain::{
    Boilerplate, BoilerplateStructure, FileStructure, FolderStructure,
};
