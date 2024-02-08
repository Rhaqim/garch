pub mod garch_cli {
    use std::process::Command;

    use clap::{Args, Parser, Subcommand};

    use crate::{
        cmd::{Boilerplate, BoilerplateStructure},
        prompt::{prompt_question, prompt_option}
    };

    #[derive(Parser, Debug)]
    #[clap(name = "garch-cli", version = "0.1.0", author = "Garch")]
    #[clap(about = "A CLI for scaffolding Go projects")]
    pub struct GarchCli {
        #[clap(subcommand)]
        pub subcmd: GarchCommands,

        #[clap(flatten)]
        pub args: GarchArgs,
    }

    #[derive(Subcommand, Debug)]
    pub enum GarchCommands {
        #[command(about = "Generate default Go project")]
        Gen(GarchArgs),
    }

    #[derive(Args, Debug, Clone)]
    pub struct GarchArgs {}

    #[derive(Args, Debug, Clone)]
    pub struct ProjectConfig {
        pub arch: String,
        pub title: String,
        pub author: String,
        // Add more fields for other configurations
    }

    impl ProjectConfig {
        pub fn new() -> Self {
            ProjectConfig {
                arch: String::new(),
                title: String::new(),
                author: String::new(),
                // Initialize other fields as needed
            }
        }
    }

    fn get_git_from_config() -> String {
        let output = Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("user.name")
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    pub async fn parse() {
        let cli = GarchCli::parse();

        match cli.subcmd {
            GarchCommands::Gen(_) => {
                // if Args is empty, prompt the user for input

                let mut config = ProjectConfig::new();

                println!("Welcome to Garch CLI!");

                let default_arch = "hexagonal";

                // Ask what architecture the user wants to use and present options
                let prompt_arch = prompt_option(
                    &format!(
                        "What architecture would you like to use? (default: {})",
                        default_arch
                    ),
                    vec!["hexagonal", "onion", "clean"],
                );

                if !prompt_arch.is_empty() {
                    config.arch = prompt_arch;
                } else {
                    config.arch = default_arch.to_string();
                }

                // Example: Ask the user for the title of the project
                config.title = prompt_question("What is the title of this project?");
                let git_username = get_git_from_config();

                // Example: Ask the user for the author of the project
                let prompt_username = prompt_question(&format!(
                    "Who is the author of this project? (default: {})",
                    git_username
                ));

                if !prompt_username.is_empty() {
                    config.author = prompt_username;
                } else {
                    config.author = git_username.trim_end().to_string();
                }

                // Add more questions here, saving the answers to the config object

                // Once all questions are answered, generate boilerplate
                generate_boilerplate(&config).await;
            }
        }
    }

    async fn generate_boilerplate(config: &ProjectConfig) {
        println!("Generating boilerplate for project: {}", config.title);
        // Generate boilerplate code based on the configuration

        let boilerplate: BoilerplateStructure = Boilerplate::new(config);
        let result = boilerplate.generate();

        if let Err(e) = result {
            println!("Error generating boilerplate: {}", e);
            return;
        }

        println!("Boilerplate generated successfully!");
    }
}
