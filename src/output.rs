use clap::ArgMatches;
use tabled::{row, Table, Tabled, builder::Builder, settings::Style};
use std::collections::HashMap;
use regex::Regex;

use std::io::Read;

#[derive(Tabled)]
struct VersionChart {
    language: String,
    version: String,
}

impl VersionChart {
    fn new(language: String, version: String) -> Self {
        Self {
            language,
            version,
        }
    }
}

#[derive(Tabled)]
struct Logo {
    ascii: String,
}

impl Logo {
    fn new(ascii: String) -> Self {
        Self {
            ascii,
        }
    }
}

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
        let mut language_chart:Vec<VersionChart> = vec![];
        let reg_find_version = Regex::new(r"(\d+\.\d+.\d+)").unwrap();
        
        let python = std::process::Command::new("python")
            .arg("-V")
            .output();
        match python {
            Ok(python_version) => {
                let python_version_string = std::str::from_utf8(&python_version.stdout).expect("");
                let Some(captured_version) = reg_find_version.captures(&python_version_string) else { return };
                language_chart.push(VersionChart::new("Python".to_string(),captured_version[1].to_string()));
            },
            Err(_) => (),
        };

        let python3 = std::process::Command::new("python3")
            .arg("-V")
            .output();
        match python3 {
            Ok(python3_version) => {
                let python3_version_string = std::str::from_utf8(&python3_version.stdout).expect("");
                let Some(captured_version) = reg_find_version.captures(&python3_version_string) else { return };
                language_chart.push(VersionChart::new("Python3".to_string(),captured_version[1].to_string()));
            },
            Err(_) => (),
        };
        
        let rust = std::process::Command::new("rustc")
            .arg("-V")
            .output();
        match rust {
            Ok(rust_version) => {
                let rust_version_string = std::str::from_utf8(&rust_version.stdout).expect("");
                let Some(captured_version) = reg_find_version.captures(&rust_version_string) else { return };
                language_chart.push(VersionChart::new("Rust".to_string(),captured_version[1].to_string()));
            },
            Err(_) => (),
        };
        
        /*let mut lang_builder = Builder::new();
        lang_builder.set_header(["Language", "Version"]);
        for (lang_name, lang_version) in &languages {
            let Some(caps) = reg_find_version.captures(&lang_version) else { return };
            lang_builder.push_record([lang_name, &caps[1].to_string()]);
        };
        let lang_table = lang_builder.build()
            .with(Style::sharp())
            .to_string();

        let mut temp_versions = vec![
            VersionChart::new("Python".to_string(),"3.11.0".to_string()),
            VersionChart::new("Javascript".to_string(),"0.1.0".to_string()),
            VersionChart::new("Ada".to_string(),"99.0.9".to_string()),
        ];

        temp_versions.push(VersionChart::new("C++".to_string(),"2.21.1".to_string()));*/

        let temp_logo = [
            Logo::new("    **    ".to_string()),
            Logo::new("   *--*   ".to_string()),
            Logo::new("  *----*  ".to_string()),
            Logo::new("  *----*  ".to_string()),
            Logo::new("    ||    ".to_string()),
        ];

        let right_side = Table::new(&language_chart).with(Style::modern()).to_string();
        let left_side = Table::new(&temp_logo).with(Style::blank()).to_string();

        let combined = row![left_side.to_string(),right_side.to_string()];
        println!("{}", &combined);
        
        //read_file_buffer("./ascii_art.bin");
    }
}