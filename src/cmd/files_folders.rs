pub mod garch_files_folders {
    use std::{
        fs::{self, File},
        io::Write,
        path::Path,
    };

    use tokio::io;

    use crate::cmd::FolderStructure;

    /// Recursively generates the folder structure and files.
    ///
    /// # Arguments
    ///
    /// * `items` - The list of folder structures to generate.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating the success or failure of the generation process.
    pub fn generate_recursive(items: &Option<Vec<FolderStructure>>) -> io::Result<()> {
        if let Some(items) = items {
            for item in items {
                create_folder(&item.folder_title)?;
                change_directory(&item.folder_title)?;

                if let Some(files) = &item.files {
                    for file in files {
                        create_file(&file.file_name, &file.content)?;
                    }
                }

                generate_recursive(&item.sub_folders)?;
                change_directory("..")?;
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
    pub fn create_folder(folder_name: &str) -> io::Result<()> {
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
    pub fn change_directory(folder_name: &str) -> io::Result<()> {
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
    pub fn create_file(file_name: &str, content: &str) -> io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    /// Deletes the specified folder.
    ///
    /// # Arguments
    ///
    ///
    /// * `folder_name` - The name of the folder to delete.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating the success or failure of the folder deletion process.
    ///
    /// # Note
    ///
    /// This method is not currently used in the application, but is included for future use.
    ///
    /// # Example
    ///
    /// ```
    /// use garch::cmd::garch_boilerplate::Boilerplate;
    ///
    /// let boilerplate = BoilerplateStructure::new(&project);
    /// boilerplate.delete_folder("folder_name");
    ///
    /// ```
    pub fn _delete_folder(folder_name: &str) -> io::Result<()> {
        fs::remove_dir(folder_name)?;
        Ok(())
    }
}
