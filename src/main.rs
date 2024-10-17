use clap::{Parser, Subcommand};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::get;
use std::fs::{create_dir, remove_dir_all};
use reqwest::header::CONTENT_LENGTH;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Parser)]
#[command(name = "BeagleEditor Toolchain")]
#[command(about = "Tools for BeagleEditor, all in one place!", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Explain {
        #[arg(value_name = "EXPLAINID")]
        name: String,
    },
    Install,
}

struct ExplainInfo {
    id: String,
    infotype: String,
    message: String,
    info: String,
}

fn main() {
    // BEE100: Known error but with Python/TypeScript exception message
    // BEE120: Error when writing the file
    // BEE140: Error when reading the file
    // BEE160: Error when reading an extension
    // BEW100: Warning
    // BEI100: Information
    let cli = Cli::parse();
    match &cli.command {
        Commands::Explain { name } => {
            let mut expinfo  = ExplainInfo {
                id: "INVALID".to_string(),
                infotype: "Unknown".to_string(),
                message: "Unknown message".to_string(),
                info: "It look like explain ID is invalid. Please pass a valid explain ID".to_string(),
            };

            if name == "BEE100" {
                expinfo = ExplainInfo {
                    id: "BEE100".to_string(),
                    infotype: "Error".to_string(),
                    message: "Exception message provided by Python/TypeScript".to_string(),
                    info: "It is a known error, but the message is provided by the programming language".to_string(),
                };
            }
            if name == "BEE120" {
                expinfo = ExplainInfo {
                    id: "BEE120".to_string(),
                    infotype: "Error".to_string(),
                    message: "Error when writing the file".to_string(),
                    info: "It can be occurred when writing the file. Example: No space left on device to save the file".to_string(),
                }
            }
            if name == "BEE140" {
                expinfo = ExplainInfo {
                    id: "BEE140".to_string(),
                    infotype: "Error".to_string(),
                    message: "Error when reading the file".to_string(),
                    info: "It can be occurred when reading the file. Example: File not exists"
                        .to_string(),
                }
            }
            if name == "BEE160" {
                expinfo = ExplainInfo {
                    id: "BEE160".to_string(),
                    infotype: "Error".to_string(),
                    message: "Error when reading an extension".to_string(),
                    info: "It can be occurred when reading an extension. Example: Syntax error"
                        .to_string(),
                }
            }
            if name == "BEW100" {
                expinfo = ExplainInfo {
                    id: "BEW100".to_string(),
                    infotype: "Warning".to_string(),
                    message: "Exception message provided by Python/TypeScript".to_string(),
                    info: "It is a known error, but the message is provided by the programming language".to_string(),
                }
            }
            if name == "BEI100" {
                expinfo = ExplainInfo {
                    id: "BEI100".to_string(),
                    infotype: "Information".to_string(),
                    message: "Information message provided by BeagleEditor".to_string(),
                    info: "It is an information provided by BeagleEditor".to_string(),
                }
            }

            let expid = &expinfo.id;
            let expinfotype = &expinfo.infotype;
            let expmsg = &expinfo.message;
            let expinfo = &expinfo.info;
            println!(
                "BeagleEditor Explaination\nID: {}\nMessage Type: {}\nMessage: {}\nInformation: {}",
                expid.purple(),
                expinfotype.red(),
                expmsg.blue(),
                expinfo.green(),
            );
        }
        Commands::Install => {
            let save_path = "tmpexec/beagleeditor.exe";
            let dlurl = "https://github.com/beaglesoftware/editor/releases/download/v2024.4.0.1/BeagleEditor-2024.4.0.1-Installer.exe";

            let response = get(dlurl).expect("Failed to download file");

            let total_size = response
                .headers()
                .get(CONTENT_LENGTH)
                .and_then(|val| val.to_str().ok()) // This returns an Option<&str>
                .and_then(|s| s.parse::<u64>().ok()) // Parse to u64, returning Option<u64>
                .unwrap_or(0); // Default to 0 if parsing fails

            let pb = ProgressBar::new(total_size);
            pb.set_style(ProgressStyle::default_bar()
                    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
                    .expect("Invalid progress bar template")
                    .progress_chars("#>-"));

            let path = Path::new(save_path);

            if !(Path::new("tmpexec/").exists()) {
                println!("Temporary directory does not exists. Creating...");
                create_dir("tmpexec/");
            }

            let mut file = File::create(&path).expect("Failed to create the file");

            let bytes = response.bytes().expect("Failed to read bytes");

            let chunk_size = 4096;
            let mut downloaded: u64 = 0;

            for chunk in bytes.chunks(chunk_size) {
                file.write_all(chunk).expect("Failed to write to file");
                downloaded += chunk.len() as u64;
                pb.set_position(downloaded);
            }

            pb.finish_with_message("Download complete!");

            println!("Now let's install BeagleEditor....");
            println!("Installing BeagleEditor....");
            let mut child = Command::new("./tmpexec/beagleeditor.exe")
                .spawn()
                .expect("Failed to execute installer");

            match child.wait() {
                Ok(status) => {
                    if status.success() {
                        println!("BeagleEditor installed successfully!");
                    } else {
                        println!("Process exited with failure. Exit status: {}", status);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to wait on child process: {}", e);
                }
            }

            println!("Cleaning up by deleting temporary executable...");
            match remove_dir_all("tmpexec/") {
                Ok(_) => println!("Temporary directory and its contents deleted successfully!"),
                Err(e) => eprintln!("Error deleting directory: {}", e),
            }
            println!("Installed BeagleEditor successfully!");
        }
    }
}
