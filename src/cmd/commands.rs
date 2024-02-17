pub mod garch_cli_commands {
    use std::{
        io::{self, Write},
        process::Command,
    };

    use crate::cmd::database::run_database_commands;

    /// Runs the `git init` command to initialize a new Git repository.
    ///
    /// # Returns
    ///
    /// An `io::Result` indicating the success or failure of the `git init` command.
    pub fn run_git_init() -> io::Result<()> {
        let output = Command::new("git")
            .arg("init")
            .arg("&&")
            .arg("git")
            .arg("branch")
            .arg("-m")
            .arg("main")
            .output()?;

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
    pub fn run_go_init(username: &str, project_title: &str) -> io::Result<()> {
        let output = Command::new("go")
            .arg("mod")
            .arg("init")
            .arg(format!("github.com/{}/{}", username, project_title))
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

    /// Add databsaes to the Go project.
    ///
    /// # Arguments
    ///
    /// * `db_type` - The type of database to add to the project.
    ///
    /// # Returns
    ///
    /// A `Vec` of `String` containing the commands to run to add the specified database to the project.
    pub fn add_database(db_type: &str) -> io::Result<()> {
        let commands = run_database_commands(db_type);

        // run the commands on the shell
        for command in commands {
            let output = Command::new("sh").arg("-c").arg(command).output()?;

            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;

            if !output.status.success() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to run database commands",
                ));
            }
        }

        Ok(())
    }
}
