use clap::ArgMatches;
use std::collections::HashMap;

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
        
        for (lang_name, lang_version) in &Languages {
            println!("{lang_name}: [{lang_version}]");
        };
        let rust_logo = "
        :  -#:.%%.:#-  :            
        - .@@*@@@@%%@@@@*@@. -         
     . -@@@@@@@%%%..@%%@@@@@@@- .      
    .@@@@@@*-.   -%#-   .-*@@@@@@.     
  =**@@@%-                  -%@@@**=   
  :@@@@@@@@@@@@@@@@@@@@@%#+.  =@@@@:   
:@@@@@@@@@@@@@@@@@@@@@@@@@@@=  +@@@@@: 
-*@#.-@+:@@@@@@#-----=%@@@@@@ -@::%@*- 
:#@@@##*: %@@@@@#-----=%@@@@@= -##*@@@#:
-*@@@=    %@@@@@@@@@@@@@@@@*      =@@@*-
-*@@@=    %@@@@@%****%@@@@@@#    +#@@@*-
:#@@@#    %@@@@@*     -@@@@@@=  =@@@@@#:
-*@@@@@@@@@@@@@@@@@=  %@@@@@@@@@@@@@*- 
:@@@@@@@@@@@@@@@@@@=  :@@@@@@@@@@@@@@: 
  :@@@@%--==:::::::.    ::==--%@@@@:   
  =**@@@@*+@-            =@+#@@@@**=   
    .@@@@=-%%:.        .:@#:=@@@@.     
     . -@@@@@@@@%####%@@@@@@@@- .      
        - .@@*@@@@@@@@@@*@@. -         
           :  -#:.%%.:#-  :            

        ";
        println!("{}",&rust_logo);
    }
}