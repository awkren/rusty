use serde::Deserialize;
use serde_yaml::from_reader;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Debug)]
pub struct FileType {
    pub name: String,
    pub hl_opts: HighlightingOptions,
}

#[derive(Deserialize, Default, Debug)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("Filetype not detected!"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl FileType {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }
    pub fn from(file_name: &str) -> Self {
        let file = File::open("highlighting_config.yaml").expect("Failed to open config file");
        let file_types: Vec<FileType> =
            serde_yaml::from_reader(file).expect("Failed to parse yaml.");

        for file_type in file_types {
            if (file_name.ends_with(".rs") && file_type.name == "Rust")
                || ((file_name.ends_with(".js") || file_name.ends_with(".jsx"))
                    && file_type.name == "JavaScript")
                || (file_name.ends_with(".py") && file_type.name == "Python")
                || (file_name.ends_with(".cs") && file_type.name == "C#")
                || (file_name.ends_with(".java") && file_type.name == "Java")
                || (file_name.ends_with(".sh") && file_type.name == "Bash")
                || (file_name.ends_with(".cpp") && file_type.name == "C++")
                || (file_name.ends_with(".css") && file_type.name == "CSS")
                || (file_name.ends_with(".html") && file_type.name == "HTML")
                || ((file_name.ends_with(".ts") || file_name.ends_with(".tsx"))
                    && file_type.name == "TypeScript")
            {
                return file_type;
            }
        }
        FileType::default()
    }
}

impl HighlightingOptions {
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    pub fn strings(&self) -> bool {
        self.strings
    }

    pub fn characters(&self) -> bool {
        self.characters
    }

    pub fn comments(&self) -> bool {
        self.comments
    }

    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }

    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }

    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }
}
