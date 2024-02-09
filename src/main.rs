//! Welcome to CliFerrilab!
//!
//! This cli tool help you create and manage data along with the [FerriLab] library.
//! 
//! [FerriLab]: https://github.com/asaltanubes/FerriLab
//! 
//! ## Usage
//! 
//! Once you've installed the crate you only need to type "ferris" in your terminal
//! to make it work.

mod cli;
mod config;
use cli::*;
use config::Config;

fn main() {
    let mut config: Config = confy::load("ferris", None).unwrap();
    let args = Cli::parse();

    match args.command {
        Commands::Change { temp_name, reset } => {
            if !reset {
                config.change_template(&temp_name.unwrap());
            } else {
                confy::store("ferris", None, Config::default()).unwrap();
            }
        }
        Commands::New(folder) => {
            create_folder(&folder.folder_name);
            create_main(&folder.folder_name, folder.latex, &config);
            create_jupyter(&folder.folder_name, &config);
            if let Some(data) = folder.data {
                add_data(&folder.folder_name, &data)
            }
        }
        Commands::Update => {
            std::process::Command::new("cargo")
                .args(["install", "cli-ferrilab"])
                .spawn()
                .expect("Failed to update.");
        }
    }
}
