use std::env; // For working with command-line arguments
use std::io;  // For reading user input
use slug::slugify; // For creating a slug

fn main() {
    println!("This program offers various text transformations.");
    println!("Use arguments: 'lowercase', 'uppercase', 'trimspaces', 'slugify', 'reverse', or 'novowels'.");

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 || args[1].is_empty() {
        eprintln!("No argument was provided. To use, type: 'lowercase', 'uppercase', 'trimspaces', 'slugify', 'reverse', or 'novowels'.");
        return;
    }

    let action = args[1].as_str();
    println!("Enter text to process:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input.");
    let input = input.trim();

    let result = match action {
        "lowercase" => input.to_lowercase(),
        "uppercase" => input.to_uppercase(),
        "trimspaces" => input.replace(" ", ""),
        "slugify" => slugify(input),
        "reverse" => input.chars().rev().collect(),
        "novowels" => input.chars().filter(|c| !"aeiouAEIOU".contains(*c)).collect(),
        _ => {
            eprintln!("Unknown argument '{}'. To use, type: 'lowercase', 'uppercase', 'trimspaces', 'slug', 'reverse', or 'novowels'.", action);
            return;
        }
    };

    println!("Result: '{}'", result);
}

