pub mod garch_boilerplate {
    use std::fs::{self, File};
    use std::io::{self, Write};
    use std::path::Path;
    use std::process::{Command, Output};

    const MAIN_FILE: &str = "main.go";
    const README_FILE: &str = "README.md";
    const LICENSE_FILE: &str = "LICENSE";
    const GITIGNORE_FILE: &str = ".gitignore";

    const CMD_FOLDER: &str = "cmd";
    const INTERNAL_FOLDER: &str = "internal";
    const PKG_FOLDER: &str = "pkg";
    const TESTDATA_FOLDER: &str = "testdata";

    const CONTENT: &str = r#"package main

import "fmt"

func main() {
    fmt.Println("Welcome to {}!")
}
"#;

    fn create_folder(folder_title: &str) -> io::Result<()> {
        fs::create_dir(folder_title)?;
        Ok(())
    }

    fn change_directory(folder_title: &str) -> io::Result<()> {
        std::env::set_current_dir(folder_title)?;
        Ok(())
    }

    fn generate_file(file_name: &str, content: &str) -> io::Result<()> {
        let mut file = File::create(file_name)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn boilerplate(project_title: &str) -> io::Result<()> {
        create_folder(project_title)?;
        change_directory(project_title)?;

        let username = get_git_from_config();

        run_go_init(&username, project_title)?;
        generate_file(README_FILE, "")?;
        generate_file(LICENSE_FILE, "")?;
        generate_file(GITIGNORE_FILE, "")?;

        create_folder(CMD_FOLDER)?;
        create_folder(INTERNAL_FOLDER)?;
        create_folder(PKG_FOLDER)?;
        create_folder(TESTDATA_FOLDER)?;

        generate_file(MAIN_FILE, CONTENT)?;

        Ok(())
    }

    fn get_git_from_config() -> String {
        let output = Command::new("git").arg("config").arg("--get").arg("user.name").output().unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    fn run_go_init(username: &str, project_title: &str) -> io::Result<()> {
        let output = Command::new("go").arg("mod").arg("init").arg(format!("github.com/{}/{}", username, project_title)).output()?;

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
