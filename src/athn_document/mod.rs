pub struct AthnDocument {
    pub metadata: AthnMetadata,
}

impl AthnDocument {
    pub fn new(title: String) -> AthnDocument {
        AthnDocument {
            metadata: AthnMetadata::new(title),
        }
    }
}

pub struct AthnMetadata {
    title: String,
    subtitle: Option<String>,
    author: Option<Vec<String>>,
    license: Option<Vec<String>>,
    language: Option<Vec<String>>,
    cache: Option<i32>,
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
