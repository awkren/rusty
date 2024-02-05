pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[derive(Default)]
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
        if file_name.ends_with(".rs") {
            return Self {
                name: String::from("Rust"),
                hl_opts: HighlightingOptions {
                    numbers: true,
                    strings: true,
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    primary_keywords: vec![
                        "as".to_string(),
                        "break".to_string(),
                        "const".to_string(),
                        "continue".to_string(),
                        "crate".to_string(),
                        "else".to_string(),
                        "enum".to_string(),
                        "extern".to_string(),
                        "false".to_string(),
                        "fn".to_string(),
                        "for".to_string(),
                        "if".to_string(),
                        "impl".to_string(),
                        "in".to_string(),
                        "let".to_string(),
                        "loop".to_string(),
                        "match".to_string(),
                        "mod".to_string(),
                        "move".to_string(),
                        "mut".to_string(),
                        "pub".to_string(),
                        "ref".to_string(),
                        "return".to_string(),
                        "self".to_string(),
                        "Self".to_string(),
                        "static".to_string(),
                        "struct".to_string(),
                        "super".to_string(),
                        "trait".to_string(),
                        "true".to_string(),
                        "type".to_string(),
                        "unsafe".to_string(),
                        "use".to_string(),
                        "where".to_string(),
                        "while".to_string(),
                        "dyn".to_string(),
                        "abstract".to_string(),
                        "become".to_string(),
                        "box".to_string(),
                        "do".to_string(),
                        "final".to_string(),
                        "macro".to_string(),
                        "override".to_string(),
                        "priv".to_string(),
                        "typeof".to_string(),
                        "unsized".to_string(),
                        "virtual".to_string(),
                        "yield".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "try".to_string(),
                    ],
                    secondary_keywords: vec![
                        "bool".to_string(),
                        "char".to_string(),
                        "i8".to_string(),
                        "i16".to_string(),
                        "i32".to_string(),
                        "i64".to_string(),
                        "isize".to_string(),
                        "u8".to_string(),
                        "u16".to_string(),
                        "u32".to_string(),
                        "u64".to_string(),
                        "usize".to_string(),
                        "f32".to_string(),
                        "f64".to_string(),
                    ],
                },
            };
        } else if file_name.ends_with(".js") {
            return Self {
                name: String::from("JavaScript"),
                hl_opts: HighlightingOptions {
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    numbers: true,
                    strings: true,
                    primary_keywords: vec![
                        "if".to_string(),
                        "else".to_string(),
                        "for".to_string(),
                        "while".to_string(),
                        "do".to_string(),
                        "switch".to_string(),
                        "case".to_string(),
                        "break".to_string(),
                        "continue".to_string(),
                        "return".to_string(),
                        "function".to_string(),
                        "var".to_string(),
                        "let".to_string(),
                        "const".to_string(),
                        "try".to_string(),
                        "catch".to_string(),
                        "finally".to_string(),
                        "throw".to_string(),
                        "class".to_string(),
                        "extends".to_string(),
                        "super".to_string(),
                        "import".to_string(),
                        "export".to_string(),
                        "default".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "instanceof".to_string(),
                        "typeof".to_string(),
                        "delete".to_string(),
                        "in".to_string(),
                        "with".to_string(),
                        "new".to_string(),
                        "this".to_string(),
                        "true".to_string(),
                        "false".to_string(),
                        "Infinity".to_string(),
                        "void".to_string(),
                        "debugger".to_string(),
                        "arguments".to_string(),
                    ],
                    secondary_keywords: vec![
                        "boolean".to_string(),
                        "Number".to_string(),
                        "NaN".to_string(),
                        "String".to_string(),
                        "undefined".to_string(),
                        "null".to_string(),
                        "Object".to_string(),
                        "Symbol".to_string(),
                        "BigInt".to_string(),
                        "Array".to_string(),
                        "Function".to_string(),
                        "Map".to_string(),
                        "Set".to_string(),
                        "Promise".to_string(),
                        "RegExp".to_string(),
                        "Error".to_string(),
                        "Date".to_string(),
                    ],
                },
            };
        } else if file_name.ends_with(".ts") {
            return Self {
                name: String::from("TypeScript"),
                hl_opts: HighlightingOptions {
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    numbers: true,
                    strings: true,
                    primary_keywords: vec![
                        "if".to_string(),
                        "else".to_string(),
                        "for".to_string(),
                        "while".to_string(),
                        "do".to_string(),
                        "switch".to_string(),
                        "case".to_string(),
                        "break".to_string(),
                        "continue".to_string(),
                        "return".to_string(),
                        "function".to_string(),
                        "var".to_string(),
                        "let".to_string(),
                        "const".to_string(),
                        "try".to_string(),
                        "catch".to_string(),
                        "finally".to_string(),
                        "throw".to_string(),
                        "class".to_string(),
                        "extends".to_string(),
                        "super".to_string(),
                        "import".to_string(),
                        "export".to_string(),
                        "default".to_string(),
                        "async".to_string(),
                        "await".to_string(),
                        "instanceof".to_string(),
                        "typeof".to_string(),
                        "interface".to_string(),
                        "type".to_string(),
                        "enum".to_string(),
                        "implements".to_string(),
                        "namespace".to_string(),
                        "declare".to_string(),
                        "module".to_string(),
                        "readonly".to_string(),
                        "constructor".to_string(),
                        "get".to_string(),
                        "set".to_string(),
                        "readonly".to_string(),
                        "abstract".to_string(),
                        "as".to_string(),
                        "is".to_string(),
                        "keyof".to_string(),
                        "readonly".to_string(),
                        "delete".to_string(),
                        "in".to_string(),
                        "with".to_string(),
                        "new".to_string(),
                        "this".to_string(),
                        "true".to_string(),
                        "false".to_string(),
                        "null".to_string(),
                        "undefined".to_string(),
                        "NaN".to_string(),
                        "Infinity".to_string(),
                        "void".to_string(),
                        "debugger".to_string(),
                        "arguments".to_string(),
                    ],
                    secondary_keywords: vec![
                        "boolean".to_string(),
                        "Number".to_string(),
                        "NaN".to_string(),
                        "String".to_string(),
                        "undefined".to_string(),
                        "null".to_string(),
                        "Object".to_string(),
                        "Symbol".to_string(),
                        "BigInt".to_string(),
                        "Array".to_string(),
                        "Function".to_string(),
                        "Map".to_string(),
                        "Set".to_string(),
                        "Promise".to_string(),
                        "RegExp".to_string(),
                        "Error".to_string(),
                        "Date".to_string(),
                    ],
                },
            };
        } else if file_name.ends_with(".html") {
            return Self {
                name: String::from("HTML"),
                hl_opts: HighlightingOptions {
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    numbers: true,
                    strings: true,
                    primary_keywords: vec![
                        "html".to_string(),
                        "DOCTYPE".to_string(),
                        "html".to_string(),
                        "head".to_string(),
                        "body".to_string(),
                        "div".to_string(),
                        "span".to_string(),
                        "p".to_string(),
                        "a".to_string(),
                        "img".to_string(),
                        "link".to_string(),
                        "script".to_string(),
                        "style".to_string(),
                        "h1".to_string(),
                        "h2".to_string(),
                        "h3".to_string(),
                        "h4".to_string(),
                        "h5".to_string(),
                        "h6".to_string(),
                        "ul".to_string(),
                        "ol".to_string(),
                        "li".to_string(),
                        "table".to_string(),
                        "tr".to_string(),
                        "th".to_string(),
                        "td".to_string(),
                        "form".to_string(),
                        "input".to_string(),
                        "select".to_string(),
                        "option".to_string(),
                        "button".to_string(),
                        "label".to_string(),
                        "textarea".to_string(),
                    ],
                    secondary_keywords: vec![
                        "src".to_string(),
                        "class".to_string(),
                        "id".to_string(),
                        "style".to_string(),
                        "src".to_string(),
                        "href".to_string(),
                        "alt".to_string(),
                        "width".to_string(),
                        "height".to_string(),
                        " type".to_string(),
                        "value".to_string(),
                        "placeholder".to_string(),
                        "checked".to_string(),
                        "disabled".to_string(),
                        "readonly".to_string(),
                        "selected".to_string(),
                        "colspan".to_string(),
                        "rowspan".to_string(),
                        "for".to_string(),
                        "name".to_string(),
                        "action".to_string(),
                        "method".to_string(),
                        "enctype".to_string(),
                        "autocomplete".to_string(),
                        "target".to_string(),
                        "rel".to_string(),
                        "media".to_string(),
                        "charset".to_string(),
                        "http-equiv".to_string(),
                        "content".to_string(),
                        "placeholder".to_string(),
                        "rows".to_string(),
                        "cols".to_string(),
                    ],
                },
            };
        }
        Self::default()

        /* This is how we add more languages - Remove this comment when I wont add more languages
        file_name.ends_with("file extensions here") {
            return Self {
                name: String::from("Language"),
                hl_opts: HighlightingOptions {
                    characters: true,
                    comments: true,
                    multiline_comments: true,
                    numbers: true,
                    strings: true,
                    primary_keywords: vec![],
                    secondary_keywords: vec![],
            }
        }*/
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
