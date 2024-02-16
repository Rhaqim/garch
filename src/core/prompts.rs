pub mod garch_cli_prompts {
    use std::process::Command;

    use crate::{
        core::cli::garch_cli::ProjectConfig,
        prompt::{prompt_option, prompt_question},
    };

    fn get_git_from_config() -> String {
        let output = Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.name")
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    fn select_arch(config: &mut ProjectConfig) {
        const ARCHITECTURES: [&str; 3] = ["hexagonal", "onion", "clean"];

        let default_arch = "hexagonal";

        // Ask what architecture the user wants to use and present options
        let prompt_arch = prompt_option(
                    &format!(
                        "What architecture would you like to use? input the number of the option (default: {})",
                        default_arch
                    ),
                    (&ARCHITECTURES).to_vec(),
                );

        if !prompt_arch.is_empty() {
            // check that the input is a number and within the range of the options
            let idx = prompt_arch.parse::<usize>().unwrap();

            if idx > 0 && idx <= ARCHITECTURES.len() {
                config.arch = ARCHITECTURES[idx - 1].to_string();
            } else {
                print!(
                    "Invalid option, using default architecture: {}",
                    default_arch
                );
                config.arch = default_arch.to_string();
            }
        } else {
            config.arch = default_arch.to_string();
        }
    }

    fn get_project_title(config: &mut ProjectConfig) {
        // Ask the user for the title of the project
        let title = prompt_question("What is the title of this project?");

        if !title.is_empty() {
            // if there's a space in the title, replace it with an underscore
            config.title = title.replace(" ", "_");
        } else {
            config.title = "garch_project".to_string();
        }
    }

    fn get_author(config: &mut ProjectConfig) {
        // Ask the user for the author of the project
        let git_username = get_git_from_config();
        let prompt_username = prompt_question(&format!(
            "Who is the author of this project? (default: {})",
            git_username
        ));

        if !prompt_username.is_empty() {
            config.author = prompt_username;
        } else {
            config.author = git_username.trim_end().to_string();
        }
    }

    fn select_database(config: &mut ProjectConfig) {
        // Ask the user if they want to add a database
        let prompt_db = prompt_option(
            "Would you like to add a database to this project? (default: No)",
            vec!["Yes", "No"],
        );

        let databases = vec!["PostgreSQL", "MySQL", "SQLite", "MongoDB"];

        if !prompt_db.is_empty() {
            // if the user selects yes, prompt for the database type
            if prompt_db == "1" {
                let db_selection = prompt_option(
                    "What type of database would you like to use? input the number of the option",
                    databases.clone(),
                );

                if !db_selection.is_empty() {
                    // save the database type to the config

                    let idx = db_selection.parse::<usize>().unwrap();

                    if idx > 0 && idx <= databases.len() {
                        config.db_type = databases[idx - 1].to_string();
                    } else {
                        print!("Invalid option, not adding a database");
                    }
                }
            }
        }
    }

    pub fn get_project_config(config: &mut ProjectConfig) {
        select_arch(config);
        get_project_title(config);
        get_author(config);
        select_database(config);
    }
}
