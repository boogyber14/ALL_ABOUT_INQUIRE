use inquire::{Confirm, Select, Text, validator::Validation};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Prompt for the user's name
    let name: String = Text::new("What's your name?").prompt()?;

    // Prompt for the user's age
    let age: u8 = Text::new("How old are you?")
        .with_validator(|age_str: &str| -> Result<Validation, Box<dyn Error + Send + Sync>> {
            let age: u8 = age_str.parse()?;
            if age >= 18 {
                Ok(Validation::Valid)
            } else {
                Err("You must be at least 18 years old.".into())
            }
        })
        .prompt()?
        .parse()?;

    // Prompt for confirmation
    let is_adult: bool = Confirm::new("Are you sure you're an adult?").prompt()?;

    if is_adult {
        println!("Welcome, {}! You are {} years old.", name, age);

        // Prompt for favorite programming language
        let language: String = Select::new("What's your favorite programming language?", vec![
            "Rust", "Python", "JavaScript", "C++", "Java"
        ]).prompt()?.to_string();

        println!("Nice choice! {} is a great language.", language);

        // Delete a directory
        let dir_to_delete = "/path/to/your/directory"; // Replace this with the directory you want to delete
        fs::remove_dir(dir_to_delete)?;
        println!("Directory deleted successfully.");
    } else {
        println!("Sorry, {}! You must be an adult to proceed.", name);
    }

    Ok(())
}
