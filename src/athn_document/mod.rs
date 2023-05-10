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
        for line in input.lines() {
            if line.starts_with("TI ") {
                return Ok(AthnDocument::new(line.to_string()));
            }
        } 
        Err("No title found")
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
}
