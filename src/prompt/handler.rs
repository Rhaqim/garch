pub mod garch_prompts {
    use std::io::{self, Write};

    pub fn _get_input(prompt: &str) -> String {
        print!("{}", prompt);

        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        input.trim().to_string()
    }

    pub fn prompt_question(question: &str) -> String {
        println!("{}", question);

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().to_string()
    }

    pub fn prompt_option(question: &str, options: Vec<&str>) -> String {
        println!("{}", question);

        for (i, option) in options.iter().enumerate() {
            println!("{}. {}", i + 1, option);
        }

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input.trim().to_string()
    }
}
