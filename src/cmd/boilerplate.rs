pub mod garch_boilerplate {
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    use std::process::Command;

    use crate::cmd::architecture::architecture_map;
    use crate::cmd::{Boilerplate, BoilerplateStructure, FolderStructure};
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
            }
        }

        /// Generates the project structure and files.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the generation process.
        fn generate(&self) -> io::Result<()> {
            // Create project folder and change directory
            self.create_folder(&self.project_title)?;
            self.change_directory(&self.project_title)?;

            // Run git init
            self.run_git_init()?;

            // Run go mod init
            self.run_go_init()?;

            // Generate project structure
            self.generate_recursive(&self.folders)?;

            // Generate root files
            if let Some(files) = &self.files {
                for file in files {
                    self.create_file(&file.file_name, &file.content)?;
                }
            }

            Ok(())
        }

        /// Recursively generates the folder structure and files.
        ///
        /// # Arguments
        ///
        /// * `items` - The list of folder structures to generate.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the generation process.
        fn generate_recursive(&self, items: &Option<Vec<FolderStructure>>) -> io::Result<()> {
            if let Some(items) = items {
                for item in items {
                    self.create_folder(&item.folder_title)?;
                    self.change_directory(&item.folder_title)?;

                    if let Some(files) = &item.files {
                        for file in files {
                            self.create_file(&file.file_name, &file.content)?;
                        }
                    }

                    self.generate_recursive(&item.sub_folders)?;
                    self.change_directory("..")?;
                }
            }
            Ok(())
        }

        /// Creates a new folder with the specified name.
        ///
        /// # Arguments
        ///
        /// * `folder_name` - The name of the folder to create.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the folder creation process.
        fn create_folder(&self, folder_name: &str) -> io::Result<()> {
            fs::create_dir(folder_name)?;
            Ok(())
        }

        /// Changes the current directory to the specified folder.
        ///
        /// # Arguments
        ///
        /// * `folder_name` - The name of the folder to change to.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the directory change process.
        fn change_directory(&self, folder_name: &str) -> io::Result<()> {
            let folder_path = Path::new(folder_name);
            std::env::set_current_dir(folder_path)?;
            Ok(())
        }

        /// Generates a new file with the specified name and content.
        ///
        /// # Arguments
        ///
        /// * `file_name` - The name of the file to create.
        /// * `content` - The content of the file.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the file generation process.
        fn create_file(&self, file_name: &str, content: &str) -> io::Result<()> {
            let mut file = File::create(file_name)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }

        /// Runs the `git init` command to initialize a new Git repository.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the `git init` command.
        fn run_git_init(&self) -> io::Result<()> {
            let output = Command::new("git").arg("init").output()?;

            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;

            if !output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to run git init",
                ));
            }

            Ok(())
        }

        /// Runs the `go mod init` command to initialize a new Go module.
        ///
        /// # Returns
        ///
        /// An `io::Result` indicating the success or failure of the `go mod init` command.
        fn run_go_init(&self) -> io::Result<()> {
            let output = Command::new("go")
                .arg("mod")
                .arg("init")
                .arg(format!(
                    "github.com/{}/{}",
                    self.username, self.project_title
                ))
                .output()?;

            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;

            if !output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to run go mod init",
                ));
            }

            Ok(())
        }
    }
}
