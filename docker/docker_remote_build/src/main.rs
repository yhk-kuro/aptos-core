use clap;
use clap::Parser;
use reqwest;

/// Simple program to greet a person
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    // pattern: String,
    // /// The path to the file to read
    // #[clap(parse(from_os_str))]
    // path: std::path::PathBuf,
    /// Number of times to greet
    #[clap(long)]
    branch: String,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    println!("branch = {:?}", args.branch);
    let body = reqwest::blocking::get("https://www.rust-lang.org")?
        .text()?;

    println!("body = {:?}", body);
    Ok(())
}
