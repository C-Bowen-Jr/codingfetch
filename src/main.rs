#[cfg(feature = "pretty_output")]
#[macro_use] extern crate prettytable;

use clap::Arg;
mod output;

fn main() {
    let matches = clap::App::new("codingfetch")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Laylong <https://github.com/C-Bowen-Jr>")
        .about("\nA neofetch like tool for programmers.")
        .arg(Arg::with_name("test"))
            .help_short("t")
            .help("Test command")
        .get_matches();

    output::main(matches);
}