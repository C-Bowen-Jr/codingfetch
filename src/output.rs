use clap::ArgMatches;
use tabled::{builder::Builder, settings::Style};
use std::collections::HashMap;

use std::io::Read;

fn read_file_buffer(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    const BUFFER_LEN: usize = 1024;
    let mut buffer = [0u8; BUFFER_LEN];
    let mut file = std::fs::File::open(filepath).unwrap();

    loop {
        let read_count = file.read(&mut buffer).unwrap();
        println!("{}", std::str::from_utf8(&buffer[..read_count]).unwrap());

        if read_count != BUFFER_LEN {
            break;
        }
    }
    Ok(())
}

pub fn main(matches: ArgMatches) {
    if matches.is_present("test") {
        println!("Version: TODO");
    }
    else {
        let mut Languages = HashMap::new();
        
        let python = std::process::Command::new("python")
            .arg("-V")
            .output();
        match python {
            Ok(python_version) => {
                let python_version_string = std::str::from_utf8(&python_version.stdout).expect("");
                Languages.entry("Python".to_string()).or_insert(python_version_string.to_string());
            },
            Err(_) => (),
        };

        let python3 = std::process::Command::new("python3")
            .arg("-V")
            .output();
        match python3 {
            Ok(python3_version) => {
                let python3_version_string = std::str::from_utf8(&python3_version.stdout).expect("");
                Languages.entry("Python 3".to_string()).or_insert(python3_version_string.to_string());
            },
            Err(_) => (),
        };
        
        let rust = std::process::Command::new("rustc")
            .arg("-V")
            .output();
        match rust {
            Ok(rust_version) => {
                let rust_version_string = std::str::from_utf8(&rust_version.stdout).expect("");
                Languages.entry("Rust".to_string()).or_insert(rust_version_string.to_string());
            },
            Err(_) => (),
        };
        
        let mut lang_builder = Builder::new();
        for (lang_name, lang_version) in &Languages {
            //println!("{lang_name}: [{lang_version}]");
            lang_builder.push_record([lang_name, lang_version]);
        };
        let lang_table = lang_builder.build()
            .with(Style::ascii_rounded())
            .to_string();
        println!("{}", lang_table);
        
        //read_file_buffer("./ascii_art.bin");
    }
}