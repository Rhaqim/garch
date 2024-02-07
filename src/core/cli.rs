pub mod garch_cli {
    use clap::{Args, Parser, Subcommand};

    use crate::cmd::boilerplate::garch_boilerplate::boilerplate;

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

    #[derive(Args, Debug)]
    pub struct ProjectConfig {
        pub title: String,
        // Add more fields for other configurations
    }

    impl ProjectConfig {
        pub fn new() -> Self {
            ProjectConfig {
                title: String::new(),
                // Initialize other fields as needed
            }
        }
    }

    pub async fn parse() {
        let cli = GarchCli::parse();

        match cli.subcmd {
            GarchCommands::Gen(_) => {
                let mut config = ProjectConfig::new();

                println!("Welcome to Garch CLI!");

                // Example: Ask the user for the title of the project
                config.title = prompt_user("What is the title of this project?");

                // Add more questions here, saving the answers to the config object

                // Once all questions are answered, generate boilerplate
                generate_boilerplate(&config).await;
            }
        }
    }

    fn prompt_user(question: &str) -> String {
        println!("{}", question);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }

    async fn generate_boilerplate(config: &ProjectConfig) {
        println!("Generating boilerplate for project: {}", config.title);
        // Generate boilerplate code based on the configuration

        boilerplate(&config.title);

        println!("Boilerplate generated successfully!");
    }
}
