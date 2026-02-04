use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let command = args[1].as_str();

    match command {
        "clean" | "upper" | "lower" => {
            if args.len() != 4 {
                eprintln!("Error: '{}' needs <input_file> <output_file>", command);
                print_usage();
                return;
            }

            let input_file = &args[2];
            let output_file = &args[3];

            let text = match fs::read_to_string(input_file) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Could not read file '{}': {}", input_file, e);
                    return;
                }
            };

            let processed = match command {
                "clean" => clean_text(&text),
                "upper" => text.to_uppercase(),
                "lower" => text.to_lowercase(),
                _ => unreachable!(),
            };

            if let Err(e) = fs::write(output_file, &processed) {
                eprintln!("Could not write to file '{}': {}", output_file, e);
                return;
            }

            println!(
                "Command '{}' finished. Output saved to '{}'.",
                command, output_file
            );
        }

        "stats" => {
            if args.len() != 3 {
                eprintln!("Error: 'stats' needs <input_file>");
                print_usage();
                return;
            }

            let input_file = &args[2];

            let text = match fs::read_to_string(input_file) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("Could not read file '{}': {}", input_file, e);
                    return;
                }
            };

            print_stats(&text);
        }

        _ => {
            eprintln!("Unknown command: '{}'", command);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("TextTidy - simple text cleaning tool");
    println!();
    println!("Usage:");
    println!("  texttidy clean <input_file> <output_file>");
    println!("  texttidy upper <input_file> <output_file>");
    println!("  texttidy lower <input_file> <output_file>");
    println!("  texttidy stats <input_file>");
}

fn clean_text(text: &str) -> String {
    let no_punct: String = text
        .chars()
        .filter(|c| !c.is_ascii_punctuation())
        .collect();

    let mut cleaned_parts = Vec::new();
    for part in no_punct.split_whitespace() {
        cleaned_parts.push(part);
    }

    cleaned_parts.join(" ")
}

fn print_stats(text: &str) {
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    let line_count = text.lines().count();

    println!("Text statistics:");
    println!("  Words : {}", word_count);
    println!("  Chars : {}", char_count);
    println!("  Lines : {}", line_count);

    let minutes = word_count as f64 / 200.0;
    println!("  Estimated reading time: {:.2} minutes", minutes);
}

