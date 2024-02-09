use serde::{Deserialize, Serialize};
use std::{path::Path, str::from_utf8, fs::read_to_string};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub latex: String,
    pub typst: String,
    pub jupyter: String,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            latex: from_utf8(include_bytes!("../templates/latex_template.tex"))
                .unwrap()
                .to_owned(),
            typst: from_utf8(include_bytes!("../templates/typst_template.typ"))
                .unwrap()
                .to_owned(),
            jupyter: from_utf8(include_bytes!("../templates/jupyter.ipynb"))
                .unwrap()
                .to_owned(),
        }
    }
}

impl Config {
    pub fn change_template(&mut self, name: &Path) {
        match name.extension().unwrap().to_str().unwrap() {
            "tex" => {
                self.latex = read_to_string(name).unwrap();
                confy::store("ferris", None, self).unwrap();
            }

            "typ" => {
                self.typst = read_to_string(name).unwrap();
                confy::store("ferris", None, self).unwrap();
            }

            "ipynb" => {
                self.jupyter = read_to_string(name).unwrap();
                confy::store("ferris", None, self).unwrap();
            }

            _ => {
                eprintln!("Wrong file extension, it must be a .tex, .typ or .ipynb file.")    
            }
        }
    }
}
