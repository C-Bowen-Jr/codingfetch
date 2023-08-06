use clap::{Parser};
mod output;

/// Neofetch like for programers, check and show off compilers
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Set github username
    #[arg(long)]
    gh_user: String,
}

fn main() {
    let args = Args::try_parse();

    match args {
        Ok(found) => println!("Found {}", found.gh_user),
        Err(_) => (),   
    }
    output::main();
}