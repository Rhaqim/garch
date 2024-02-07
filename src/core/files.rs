pub mod garch_files {
    use std::io::{self, Write};
    use std::process::Command;

    use chrono::format;

    /* Root Files */
    const MAIN_FILE: &str = "main.go";
    const README_FILE: &str = "README.md";
    const LICENSE_FILE: &str = "LICENSE";
    const GITIGNORE_FILE: &str = ".gitignore";

    /* Root Folders */
    const CMD_FOLDER: &str = "cmd";
    const INTERNAL_FOLDER: &str = "internal";
    const PKG_FOLDER: &str = "pkg";
    const TESTDATA_FOLDER: &str = "testdata";

    const CONTENT: &str = r#"package main

    func main() {{
        fmt.Println("Welcome to {}!")
    }}

    "#;

    fn create_folder(folder_title: &str) {
        std::fs::create_dir(folder_title).expect("Failed to create directory");
    }

    fn change_directory(folder_title: &str) {
        std::env::set_current_dir(folder_title).expect("Failed to change directory");
    }

    fn generate_file(file_name: &str, content: &str) {
        let mut file = std::fs::File::create(file_name).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    pub fn Bboilerplate(project_title: &str) {
        // Create the Project Root Folder
        create_folder(project_title);
        change_directory(project_title);

        // Create the Root Files
        generate_file(README_FILE, "");
        generate_file(LICENSE_FILE, "");
        generate_file(GITIGNORE_FILE, "");

        // Create the Root Folders
        create_folder(CMD_FOLDER);
        create_folder(INTERNAL_FOLDER);
        create_folder(PKG_FOLDER);
        create_folder(TESTDATA_FOLDER);

        // Create the Main File
        generate_file(&format!("{}/{}", project_title, MAIN_FILE), CONTENT);
    }

    pub fn run_project(project_title: &str) {
        let output = Command::new("go")
            .arg("run")
            .arg("main.go")
            .output()
            .expect("Failed to run the project");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
}
