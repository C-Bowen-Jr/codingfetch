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
        Ok(found) => {
            let res = output::ConfigSettings::set_config(output::ConfigSettings::new(found.gh_user));
            match res {
                Ok(_) => println!("Github username set"),
                Err(_) => println!("Config related failure"),
            }
        },
        Err(_) => (output::main()),   
    }

    
}