pub mod garch_boilerplate {

    use std::io;

    use crate::cmd::architecture::architecture_map;
    use crate::cmd::{
        add_database, change_directory, create_file, create_folder, generate_recursive,
        run_git_init, run_go_init, Boilerplate, BoilerplateStructure,
    };
    use crate::core::cli::garch_cli::ProjectConfig;

    /// Implementation of the `Boilerplate` trait for the `BoilerplateStructure` struct.
    impl Boilerplate for BoilerplateStructure {
        /// Creates a new instance of `BoilerplateStructure` based on the provided `ProjectConfig`.
        ///
        /// # Arguments
        ///
        /// * `project` - The project configuration.
        ///
        /// # Returns
        ///
        /// A new instance of `BoilerplateStructure`.
        fn new(project: &ProjectConfig) -> Self {
            let architecture = project.arch.as_str();

            let binding = architecture_map();

            let (folders, files) = &binding
                .iter()
                .find(|(arch, _)| *arch == architecture)
                .unwrap()
                .1;

            BoilerplateStructure {
                username: project.author.clone(),
                project_title: project.title.clone(),
                folders: Some(folders.clone()),
                files: Some(files.clone()),
                db_type: Some(project.db_type.clone()),
            }
        }

        /// Generates the project structure and files.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the generation process.
        fn generate(&self) -> io::Result<()> {
            // Create project folder and change directory
            create_folder(&self.project_title)?;
            change_directory(&self.project_title)?;

            // Run git init
            run_git_init()?;

            // Run go mod init
            run_go_init(&self.username, &self.project_title)?;

            // if db_type is not empty, add the database to the project
            if let Some(db_type) = &self.db_type {
                add_database(db_type)?;
            }

            // Generate project structure
            generate_recursive(&self.folders)?;

            // Generate root files
            if let Some(files) = &self.files {
                for file in files {
                    create_file(&file.file_name, &file.content)?;
                }
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        fn project_config() -> ProjectConfig {
            ProjectConfig {
                author: String::from("John Doe"),
                title: String::from("My Project"),
                arch: String::from("hexagonal"),
                db_type: String::from("mongodb"),
            }
        }

        #[test]
        fn test_new() {
            let architectures = vec![
                String::from("hexagonal"),
                String::from("clean"),
                String::from("onion"),
            ];

            let project = project_config();

            let boilerplate = BoilerplateStructure::new(&project);

            assert_eq!(boilerplate.username, project.author);
            assert_eq!(boilerplate.project_title, project.title);
            assert!(architectures.contains(&project.arch));
        }

        #[test]
        fn test_generate() {
            let project = project_config();

            let boilerplate = BoilerplateStructure::new(&project);

            assert!(boilerplate.generate().is_ok());

            // Add additional assertions to verify the generated project structure and files
        }

        // Add more tests for other methods if needed
    }
}
