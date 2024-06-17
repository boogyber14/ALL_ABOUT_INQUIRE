use inquire::{Confirm, Select, Text};
use std::error::Error;
use std::fs;

fn get_name() -> Result<String, Box<dyn Error>> {
    Ok(Text::new("What's your name?").prompt()?)
}

fn get_age() -> Result<u8, Box<dyn Error>> {
    let age_str = Text::new("How old are you?").prompt()?;
    let age = age_str.parse().map_err(|e| Box::new(e) as Box<dyn Error>)?;
    if age >= 18 {
        Ok(age)
    } else {
        Err("You must be at least 18 years old.".into())
    }
}

fn confirm_adult() -> Result<bool, Box<dyn Error>> {
    Ok(Confirm::new("Are you sure you're an adult?").prompt()?)
}

fn get_favorite_language() -> Result<String, Box<dyn Error>> {
    let lang = Select::new("What's your favorite programming language?", vec![
        "Rust", "Python", "JavaScript", "C++", "Java"
    ]).prompt()?;
    Ok(lang.to_string())
}

fn delete_directory(path: &str) -> Result<(), Box<dyn Error>> {
    fs::remove_dir(path)?;
    println!("Directory deleted successfully.");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let name = get_name()?;
    let age = get_age()?;
    let is_adult = confirm_adult()?;

    if is_adult {
        println!("Welcome, {}! You are {} years old.", name, age);

        let language = get_favorite_language()?;
        println!("Nice choice! {} is a great language.", language);

        let dir_to_delete = "/path/to/your/directory";
        delete_directory(dir_to_delete)?;
    } else {
        println!("Sorry, {}! You must be an adult to proceed.", name);
    }

    Ok(())
}
