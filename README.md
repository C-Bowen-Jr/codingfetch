# codingfetch

Codingfetch is a neofetch like Rust program. It's inspired as an alternative to [langfetch](https://github.com/aderpas/langfetch). This will display what languages you have install and their versions. In progress is what default language logo get's populated, however a config setting will allow you to change to a prefered.

## Preview
![screenshot of version 0.2.5](codingfetch_v_0-2-5.png "Example output of V.0.2.5")

## Install

 First build from source.
 ```
 cargo build --release
 ```

 Then run the make file.
 ```
 make install
 ```

 Make also has an uninstall if you need it.
 ```
 make uninstall
 ```

 ### Note:
Current makefile only works for Linux. Windows and and Mac are planned, 
but just not implemented yet.

## Usage

Call through command line by typing:
```
codingfetch
```

Set the github username line by adding the gh-user arguement followed by just your username. Ie
```
codingfetch --gh-user C-Bowen-Jr
```

If used through cargo run, keep in mind that to pass arguments to the project instead of to cargo, use a blank arg first.
```
cargo run -- --gh-user You
```

TODO: add alternative setting for gitlab accounts

## Configuration File

The configuration file is created by the crate confy. The current save path is determined by confy itself, and will be dependent on your OS.

## Currently Checks

The checked list is not exhaustive, so if you don't see an IDE, language, or compiler/interpreter, then this is a perfect contribution opertunity.

### IDEs
- Codium
- Kate
- Vim
- VSCode
- Xcode

### Languages
- Ada
- C/C++
 - gcc
- Fortran
- Git
- GoLang
- Haskell
- Java
- Javascript
 - NodeJS
- Kotlin
- Lua
- Perl
- Python
 - python
 - python3
- Ruby
- Rust
- Swift
- Zig