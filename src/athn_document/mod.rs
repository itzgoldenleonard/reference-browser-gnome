pub struct AthnDocument {
    pub metadata: AthnMetadata,
}

impl AthnDocument {
    pub fn new(title: String) -> AthnDocument {
        AthnDocument {
            metadata: AthnMetadata::new(title),
        }
    }

    pub fn from_str(input: &str) -> Result<AthnDocument, &str> {
        let metadata_section = match input.find("--- Meta\n") {
            None => return Err("No metadata section found"),
            Some(start_index) => match input.find("---\n") {
                None => return Err("Meta section found but not ended"),
                Some(end_index) => input.get(start_index + 9..end_index).unwrap(),
            },
        };

        let metadata_parsed = AthnMetadata::from_str(metadata_section).unwrap();

        Ok(AthnDocument {
            metadata: metadata_parsed,
        })
    }
}

pub struct AthnMetadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<Vec<String>>,
    pub license: Option<Vec<String>>,
    pub language: Option<Vec<String>>,
    pub cache: Option<i32>,
}

impl AthnMetadata {
    fn new(title: String) -> AthnMetadata {
        AthnMetadata {
            title: title,
            subtitle: None,
            author: None,
            license: None,
            language: None,
            cache: None,
        }
    }

    fn from_str(input: &str) -> Result<AthnMetadata, &str> {
        let lines = input.lines();
        let title = match lines.filter(|x| x.starts_with("TI ")).last() {
            None => return Err("Required metadata tag 'Title' not found"),
            Some(t) => t.split_at(3).1,
        };
        Ok(AthnMetadata::new(title.into()))
    }
}
