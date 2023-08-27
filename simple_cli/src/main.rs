use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(short, long, default_value = "help")]
    pattern: String,
    /// The path to the file to read
    #[arg(short, long, default_value = "help me")]
    search_text: String,
}

fn main() {
    let args = Cli::parse();
    if args.search_text.contains(&args.pattern) {
        println!("Found!");
    } else {
        println!("Not Found!");
    }
}
