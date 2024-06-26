use tabled::{row, Table, Tabled, settings::{Settings,Style,Disable,object::Rows, object::Columns, themes::Colorization, Color}};
use regex::Regex;
use confy;
use serde::{Serialize,Deserialize};

#[path = "logos.rs"] mod logos;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSettings {
    gh_user: String,
}

impl ::std::default::Default for ConfigSettings {
    fn default() -> Self {
        Self {
            gh_user: "".to_string(),
        }
    }
}

impl ConfigSettings {
    pub fn new(username: String) -> Self {
        Self {
            gh_user: username,
        }
    }

    pub fn get_config() -> Result<ConfigSettings, confy::ConfyError> {
        let cfg: ConfigSettings = confy::load("codingfetch", None)?;
        Ok(cfg)
    }

    pub fn set_config(new_config: ConfigSettings) -> Result<(), confy::ConfyError> {
        confy::store("codingfetch", None, new_config)?;
        Ok(())
    }
}

pub fn main() {
    if false {
        println!(env!("CARGO_PKG_VERSION"));
    }
    else {
        let mut language_chart:Vec<VersionChart> = vec![];
        let reg_find_version = Regex::new(r"(\d+\.\d+.\d+)").unwrap();
        let _table_config = Settings::default()
            .with(Style::blank());
        
        // GITHUB
        if let Ok(config_settings) = ConfigSettings::get_config() {
            language_chart.push(VersionChart::new("github.com".to_string(),config_settings.gh_user));
        } else {
            language_chart.push(VersionChart::new("github.com".to_string(),"/username".to_string()));
        }

        // IDE
        let vscode = std::process::Command::new("code")
            .arg("--version")
            .output();
        match vscode {
            Ok(vscode_version) => {
                let vscode_version_string = std::str::from_utf8(&vscode_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&vscode_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("VS Code".to_string(), found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let codium = std::process::Command::new("codium")
            .arg("--version")
            .output();
        match codium {
            Ok(codium_version) => {
                let codium_version_string = std::str::from_utf8(&codium_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&codium_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Codium".to_string(), found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let kate = std::process::Command::new("kate")
            .arg("--version")
            .output();
        match kate {
            Ok(kate_version) => {
                let kate_version_string = std::str::from_utf8(&kate_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&kate_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Kate".to_string(), found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let vim = std::process::Command::new("vim")
            .arg("--version")
            .output();
        match vim {
            Ok(vim_version) => {
                let vim_version_string = std::str::from_utf8(&vim_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&vim_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Vim".to_string(), found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let xcode = std::process::Command::new("xcodebuild")
            .arg("-version")
            .output();
        match xcode {
            Ok(xcode_version) => {
                let xcode_version_string = std::str::from_utf8(&xcode_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&xcode_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Xcode".to_string(), found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        // PROGRAMMING LANGUAGES
        let ada = std::process::Command::new("ada")
            .arg("--version")
            .output();
        match ada {
            Ok(ada_version) => {
                let ada_version_string = std::str::from_utf8(&ada_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&ada_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Ada".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let fortran = std::process::Command::new("gfortran")
            .arg("--version")
            .output();
        match fortran {
            Ok(fortran_version) => {
                let fortran_version_string = std::str::from_utf8(&fortran_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&fortran_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Fortran".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let haskell = std::process::Command::new("ghc")
            .arg("--version")
            .output();
        match haskell {
            Ok(haskell_version) => {
                let haskell_version_string = std::str::from_utf8(&haskell_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&haskell_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Haskell".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let kotlin = std::process::Command::new("kotlin")
            .arg("--version")
            .output();
        match kotlin {
            Ok(kotlin_version) => {
                let kotlin_version_string = std::str::from_utf8(&kotlin_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&kotlin_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Kotlin".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

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

        let ruby = std::process::Command::new("ruby")
            .arg("-v")
            .output();
        match ruby {
            Ok(ruby_version) => {
                let ruby_version_string = std::str::from_utf8(&ruby_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&ruby_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Ruby".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let swift = std::process::Command::new("switf")
            .arg("--version")
            .output();
        match swift {
            Ok(swift_version) => {
                let swift_version_string = std::str::from_utf8(&swift_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&swift_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Swift".to_string(),found_version[1].to_string())),
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

        let golang = std::process::Command::new("go")
            .arg("version")
            .output();
        match golang {
            Ok(go_version) => {
                let go_version_string = std::str::from_utf8(&go_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&go_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Go".to_string(),found_version[1].to_string())),
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

        let lua = std::process::Command::new("lua")
            .arg("-v")
            .output();
        match lua {
            Ok(lua_version) => {
                let lua_version_string = std::str::from_utf8(&lua_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&lua_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Lua".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let perl = std::process::Command::new("perl")
            .arg("-v")
            .output();
        match perl {
            Ok(perl_version) => {
                let perl_version_string = std::str::from_utf8(&perl_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&perl_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Perl".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        let zig = std::process::Command::new("zig")
            .arg("version")
            .output();
        match zig {
            Ok(zig_version) => {
                let zig_version_string = std::str::from_utf8(&zig_version.stdout).expect("");
                let captured_version = reg_find_version.captures(&zig_version_string);
                match captured_version {
                    Some(found_version) => language_chart.push(VersionChart::new("Zig".to_string(),found_version[1].to_string())),
                    None => (),
                }
            },
            Err(_) => (),
        };

        // 40 char width
        let mut temp_logo: Vec<String> = vec![];
        let logo_lines = logos::get_logo("anything");
        for each in &logo_lines {
            temp_logo.push(each.to_string());
        }
        

        let right_side = Table::new(&language_chart)
            .with(Disable::row(Rows::first()))
            .with(Style::psql())
            .with(Colorization::exact([Color::BOLD | Color::FG_BRIGHT_GREEN], Columns::first()))
            .to_string();
        let left_side = Table::new(&temp_logo)
            .with(Disable::row(Rows::first()))
            .with(Style::blank())
            .to_string();

        let combined = row![left_side.to_string(),right_side.to_string()]
            .with(Style::blank())
            .to_string();
        println!("\n\n{}\n\n", &combined);
    }
}