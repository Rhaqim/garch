pub mod garch_cli {

    use clap::{Args, Parser, Subcommand};

    use crate::{
        cmd::{Boilerplate, BoilerplateStructure},
        core::prompts::garch_cli_prompts::get_project_config,
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
        pub db_type: String,
        // Add more fields for other configurations
    }

    impl ProjectConfig {
        pub fn new() -> Self {
            ProjectConfig {
                arch: String::new(),
                title: String::new(),
                author: String::new(),
                db_type: String::new(),
                // Initialize other fields as needed
            }
        }
    }

    pub async fn parse() {
        let cli = GarchCli::parse();

        match cli.subcmd {
            GarchCommands::Gen(_) => {
                // if Args is empty, prompt the user for input

                let mut config = ProjectConfig::new();

                println!("Welcome to Garch CLI!");

                // Get project configuration from the user
                get_project_config(&mut config);

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
