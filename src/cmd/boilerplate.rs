pub mod garch_boilerplate {
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    use std::process::Command;

    use crate::cmd::architecture::architecture_map;
    use crate::cmd::{Boilerplate, BoilerplateStructure, FolderStructure};
    use crate::core::cli::garch_cli::ProjectConfig;

    impl Boilerplate for BoilerplateStructure {
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

        fn generate(&self) -> io::Result<()> {
            self.create_folder(&self.project_title)?;
            self.change_directory(&self.project_title)?;
            self.run_go_init()?;

            self.generate_recursive(&self.folders)?;

            if let Some(files) = &self.files {
                for file in files {
                    self.generate_file(&file.file_name, &file.content)?;
                }
            }

            Ok(())
        }

        fn generate_recursive(&self, items: &Option<Vec<FolderStructure>>) -> io::Result<()> {
            if let Some(items) = items {
                for item in items {
                    self.create_folder(&item.folder_title)?;
                    self.change_directory(&item.folder_title)?;

                    if let Some(files) = &item.files {
                        for file in files {
                            self.generate_file(&file.file_name, &file.content)?;
                        }
                    }

                    self.generate_recursive(&item.sub_folders)?;
                    self.change_directory("..")?;
                }
            }
            Ok(())
        }

        fn create_folder(&self, folder_name: &str) -> io::Result<()> {
            fs::create_dir(folder_name)?;
            Ok(())
        }

        fn change_directory(&self, folder_name: &str) -> io::Result<()> {
            let folder_path = Path::new(folder_name);
            std::env::set_current_dir(folder_path)?;
            Ok(())
        }

        fn generate_file(&self, file_name: &str, content: &str) -> io::Result<()> {
            let mut file = File::create(file_name)?;
            file.write_all(content.as_bytes())?;
            Ok(())
        }

        fn run_go_init(&self) -> io::Result<()> {
            let output = Command::new("go")
                .arg("mod")
                .arg("init")
                .arg(format!("github.com/{}/{}", self.username, self.project_title))
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
