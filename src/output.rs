use clap::ArgMatches;
use tabled::{row, Table, Tabled, settings::{Settings,Style,Disable,object::Rows, object::Columns, themes::Colorization, Color}};
//use std::collections::HashMap;
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

pub fn main() {
    if false {
        println!(env!("CARGO_PKG_VERSION"));
    }
    else {
        let mut language_chart:Vec<VersionChart> = vec![];
        let reg_find_version = Regex::new(r"(\d+\.\d+.\d+)").unwrap();
        let table_config = Settings::default()
            .with(Style::blank());
        
        language_chart.push(VersionChart::new("github.com".to_string(),"/username".to_string()));

        let python = std::process::Command::new("python")
            .arg("-V")
            .output();
        match python {
            Ok(python_version) => {
                let python_version_string = std::str::from_utf8(&python_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&python_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Python".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let python3 = std::process::Command::new("python3")
            .arg("-V")
            .output();
        match python3 {
            Ok(python3_version) => {
                let python3_version_string = std::str::from_utf8(&python3_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&python3_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Python3".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };
        
        let rust = std::process::Command::new("rustc")
            .arg("-V")
            .output();
        match rust {
            Ok(rust_version) => {
                let rust_version_string = std::str::from_utf8(&rust_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&rust_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Rust".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };
        
        let ccpp = std::process::Command::new("gcc")
            .arg("--version")
            .output();
        match ccpp {
            Ok(ccpp_version) => {
                let ccpp_version_string = std::str::from_utf8(&ccpp_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&ccpp_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("C/C++".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let git = std::process::Command::new("git")
            .arg("--version")
            .output();
        match git {
            Ok(git_version) => {
                let git_version_string = std::str::from_utf8(&git_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&git_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Git".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let java = std::process::Command::new("javac")
            .arg("--version")
            .output();
        match java {
            Ok(java_version) => {
                let java_version_string = std::str::from_utf8(&java_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&java_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Java".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let nodejs = std::process::Command::new("nodejs")
            .arg("--version")
            .output();
        match nodejs {
            Ok(nodejs_version) => {
                let nodejs_version_string = std::str::from_utf8(&nodejs_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&nodejs_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("NodeJS".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        // 40 char width
        let temp_logo = [
            Logo::new("           ,;*?%S######S%?*;,           ".to_string()),
            Logo::new("        :*%##@@##########@@##%*:        ".to_string()),
            Logo::new("     ,+%#@@##################@@#%+,     ".to_string()),
            Logo::new("    ;S@@####@@#####@@#####@@####@@S;    ".to_string()),
            Logo::new("  ,?@@###+:;+?S##SSSSSSS##%+;:;###@@?,  ".to_string()),
            Logo::new(" ,%@####S,    ,:,,    ,,:,     S####@%, ".to_string()),
            Logo::new(" %@######:                    ,######@? ".to_string()),
            Logo::new("+@######+,                     ;######@+".to_string()),
            Logo::new("S#####@*                        +@#####S".to_string()),
            Logo::new("######@;                        :#######".to_string()),
            Logo::new("######@+                        ;@######".to_string()),
            Logo::new("S######S,                       %@#####S".to_string()),
            Logo::new("+@#####@%:                    ,?@#####@+".to_string()),
            Logo::new(" %@##SS#@#?;,              ,;*S@#####@? ".to_string()),
            Logo::new(" ,%@#%+:+#@@#S%*,      ,*%S##@######@%, ".to_string()),
            Logo::new("  ,?#@@%,,?S##S;        :#@#######@@?,  ".to_string()),
            Logo::new("    ;S@@%:  ,,          ,S######@@S;    ".to_string()),
            Logo::new("     ,+%#@S%%%%,        ,S###@@#%+,     ".to_string()),
            Logo::new("        :*%#@@#,        ,#@##%*:        ".to_string()),
            Logo::new("           ,;*+,         +*;,           ".to_string()),
        ];

        let right_side = Table::new(&language_chart)
            .with(Disable::row(Rows::first()))
            .with(Style::markdown())
            // TODO, this adds weird line break effect
            .with(Colorization::exact([Color::FG_GREEN], Columns::first()))
            //.with(tabled::settings::Modify::new(Columns::first()).with(Color::FG_GREEN))
            .to_string();
        let left_side = Table::new(&temp_logo)
            .with(Disable::row(Rows::first()))
            .with(Style::blank())
            .to_string();

        let combined = row![left_side.to_string(),right_side.to_string()]
            .with(Style::blank())
            .to_string();
        println!("\n\n{}\n\n", &combined);
        
        //read_file_buffer("./ascii_art.bin");
    }
}