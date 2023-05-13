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

#[derive(PartialEq, Debug)]
pub struct AthnMetadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<Vec<String>>,
    pub license: Option<Vec<String>>,
    pub language: Option<Vec<String>>,
    pub cache: Option<u32>,
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
        let mut metadata = AthnMetadata::new("Default title".to_string());

        for line in input.lines() {
            match line.split_at(3) {
                ("TI ", val) => metadata.title = val.to_string(),
                ("ST ", val) => metadata.subtitle = Some(val.to_string()),
                ("AU ", val) => {
                    metadata.author =
                        Some([metadata.author.unwrap_or(vec![]), vec![val.to_string()]].concat())
                }
                ("LI ", val) => {
                    metadata.license =
                        Some([metadata.license.unwrap_or(vec![]), vec![val.to_string()]].concat())
                }
                ("LA ", val) => {
                    metadata.language =
                        Some([metadata.language.unwrap_or(vec![]), vec![val.to_string()]].concat())
                }
                ("CH ", val) => match val.parse() {
                    Err(_) => return Err("Invalid value for cache duration metadata tag"),
                    Ok(val) => metadata.cache = Some(val),
                }
                (_, val) => println!("Hit catchall with value: {}", val),
            }
        }

        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    mod metadata_from_str_tests {
        use super::super::*;

        #[test]
        fn find_title() {
            let expected = "Test".to_string();

            let meta_section = "TI Test\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().title, expected);
        }

        #[test]
        fn find_subtitle() {
            let expected = Some("This is a subtitle".to_string());

            let meta_section = "TI Test\nST This is a subtitle";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().subtitle, expected);
        }

        #[test]
        fn find_0_authors() {
            let expected = None;

            let meta_section = "TI Test\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().author, expected);
        }

        #[test]
        fn find_1_author() {
            let expected = Some(vec!["Some author".to_string()]);

            let meta_section = "TI Test\nAU Some author\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().author, expected);
        }

        #[test]
        fn find_multiple_authors() {
            let expected = Some(vec![
                "Some author".to_string(),
                "Another author".to_string(),
                "3rd author".to_string(),
            ]);

            let meta_section = "TI Test\nAU Some author\nAU Another author\nAU 3rd author\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().author, expected);
        }

        #[test]
        fn find_0_licenses() {
            let expected = None;

            let meta_section = "TI Test\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().license, expected);
        }

        #[test]
        fn find_1_license() {
            let expected = Some(vec!["GPL-3.0-or-later".to_string()]);

            let meta_section = "TI Test\nAU Some author\nLI GPL-3.0-or-later";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().license, expected);
        }

        #[test]
        fn find_multiple_licenses() {
            let expected = Some(vec![
                "GPL-3.0-or-later".to_string(),
                "CC-BY-SA-4.0".to_string(),
            ]);

            let meta_section = "TI Test\nLI GPL-3.0-or-later\nLI CC-BY-SA-4.0\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().license, expected);
        }

        #[test]
        fn find_multiple_languages() {
            let expected = Some(vec!["en".to_string(), "de".to_string()]);

            let meta_section = "TI Test\nLA en\nLA de\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().language, expected);
        }

        #[test]
        fn find_no_cache_duration() {
            let expected = None;

            let meta_section = "TI Test\n";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().cache, expected);
        }

        #[test]
        fn find_valid_cache_duration() {
            let expected = Some(100);

            let meta_section = "TI Test\nAU Some author\nLI GPL-3.0-or-later\nCH 100";

            let result = AthnMetadata::from_str(meta_section);

            assert_eq!(result.unwrap().cache, expected);
        }

        #[test]
        fn find_invalid_cache_duration() {
            let meta_section = "TI Test\nAU Some author\nLI GPL-3.0-or-later\nCH 1o0";

            let result = AthnMetadata::from_str(meta_section);

            assert!(result.is_err());
        }
    }
}
