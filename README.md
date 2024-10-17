# BeagleEditor Toolchain
A CLI app to manage BeagleEditor
## Installation
### Cargo
First, make sure you have [Rust](https://www.rust-lang.org/learn/get-started) installed

Install [Visual Studio C++ build tools from Microsoft website](https://aka.ms/vs/17/release/vs_BuildTools.exe) If you have Windows, otherwise install `gcc` and `g++` with your system package manager

It's good to check for a new update if you have Rust installed previously
```
rustup update
```
Once installed, run this command to install BeTL
```
cargo install betl
```
Sometimes, installation may fail. Run this command if installation failed:
```
cargo install --locked betl
```
### Executable files
Install from [GitHub releases](https://github.com/beaglesoftware/betl/releases)
## Features
| Feature | Description |
| --- | --- |
| Explain an error/warning/info | Description not avaliable |
| Installs BeagleEditor | Installs BeagleEditor for you (Just for Windows) |
