pub use clap::{Args, Parser, Subcommand};
use std::{
    fs::{create_dir, read_to_string, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use crate::config::Config;

/// Creates a new folder including all necesary items for physics laboratory
/// analysis using FerriLab.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new directory with all necesary files for a laboratory analysis.
    New(BasicArgs),
    /// Customize the default template files. 
    Change {
        /// Path to the new template file. Must be a .tex, .typ or .ipynb file.
        temp_name: Option<PathBuf>,
        /// Reset the templates to the default ones.
        #[arg(short, long)]
        reset: bool,
    },
    /// Self updates the CLI to the last release version.
    Update
}

#[derive(Args)]
pub struct BasicArgs {
    /// Directory name
    pub folder_name: PathBuf,

    /// Path to the data for analaysing
    #[arg(short, long)]
    pub data: Option<PathBuf>,

    /// By default is a typst file, use it to create a latex file instead.
    #[arg(short, long)]
    pub latex: bool,
}

fn create_file(folder: &Path, path: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .open(folder.join(path))
}

pub fn create_main(folder: &Path, latex: bool, config: &Config) {
    if latex {
        let mut file = create_file(folder, "main.tex").expect("Failed to create the main.tex");

        write!(file, "{}", config.latex).unwrap_or(());
    } else {
        let mut file = create_file(folder, "main.typ").expect("Failed to create the main.typ");

        write!(
            &mut file,
            "#import \"template.typ\": *\n #show: project.with()"
        )
        .unwrap_or(());

        let mut temp_file =
            create_file(folder, "template.typ").expect("Failed to create the template.typ.");

        write!(temp_file, "{}", config.typst).unwrap_or(());
    }
}

pub fn create_folder(folder: &Path) {
    create_dir(folder).expect("Failed to create the new directory.");
}

pub fn create_jupyter(folder: &Path, config: &Config) {
    let mut file =
        create_file(folder, "analysis.ipynb").expect("Failed to create the analysis.ipynb.");

    write!(file, "{}", config.jupyter).expect("Failed to create the analysis.ipynb.");
}

pub fn add_data(folder: &Path, data: &Path) {
    let mut file = create_file(folder, "data.txt").expect("Failed to create the data.txt.");

    write!(file, "{}", read_to_string(data).unwrap()).expect("Failed to create the data.txt.");
}
