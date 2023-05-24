pub struct Document {
    pub metadata: Metadata,
    pub main_lines: Vec<MainLine>,
}

#[derive(PartialEq, Debug)]
pub struct Metadata {
    pub title: String,
    pub subtitle: Option<String>,
    pub author: Option<Vec<String>>,
    pub license: Option<Vec<String>>,
    pub language: Option<Vec<String>>,
    pub cache: Option<u32>,
}

#[derive(PartialEq, Debug)]
// A single line in the main section
pub enum MainLine {
    TextLine(String),
    HeadingLine(HeadingLevel, String),
}

#[derive(PartialEq, Debug)]
pub enum HeadingLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl Document {
    pub fn new(title: String) -> Document {
        Document {
            metadata: Metadata::new(title),
            main_lines: vec![],
        }
    }

    pub fn from_str(input: &str) -> Result<Document, &str> {
        // Find the metadata section and store it in a variable
        let metadata_section = match input.find("--- Meta\n") {
            None => return Err("No metadata section found"),
            Some(start_index) => match input.find("---\n") {
                None => return Err("Meta section found but not ended"),
                Some(end_index) => input.get(start_index + 9..end_index).unwrap(),
            },
        };
        // Parse the metadata section
        let metadata_parsed = Metadata::from_str(metadata_section)?;

        // Find the main section and store it
        let main_section = match input.find("---\n") {
            None => return Err("No main section found"),
            Some(i) => input.get(i + 4..).unwrap(),
        };
        // Iterate over each line of the main section and convert it to an AthnMainLine with the
        // enum's from_str function
        let main_lines = main_section
            .lines()
            .map(|line| MainLine::from_str(line))
            .collect();

        Ok(Document {
            metadata: metadata_parsed,
            main_lines,
        })
    }
}

impl Metadata {
    fn new(title: String) -> Metadata {
        Metadata {
            title,
            subtitle: None,
            author: None,
            license: None,
            language: None,
            cache: None,
        }
    }

    fn from_str(input: &str) -> Result<Metadata, &str> {
        let mut metadata = Metadata::new("Default title".to_string());

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
                },
                (_, val) => println!("Hit catchall with value: {}", val),
            }
        }

        Ok(metadata)
    }
}

impl MainLine {
    fn from_str(input: &str) -> MainLine {
        use MainLine::*;

        match input.split_at(2) {
            ("# ", val) => HeadingLine(HeadingLevel::One, val.to_string()),
            ("##", val) => match val.find(" ") {
                Some(0) => HeadingLine(HeadingLevel::Two, val.get(1..).unwrap().to_string()),
                Some(1) => HeadingLine(HeadingLevel::Three, val.get(2..).unwrap().to_string()),
                Some(2) => HeadingLine(HeadingLevel::Four, val.get(3..).unwrap().to_string()),
                Some(3) => HeadingLine(HeadingLevel::Five, val.get(4..).unwrap().to_string()),
                Some(4) => HeadingLine(HeadingLevel::Six, val.get(5..).unwrap().to_string()),
                _ => TextLine(input.to_string()),
            }
            (_, _) => TextLine(input.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    mod document_from_str_tests {
        use super::super::*;

        #[test]
        fn find_single_text_line() {
            let expected = vec![MainLine::TextLine("Hello world!".to_string())];

            let input = "--- Meta\nTI Test\n---\nHello world!\n";

            let result = Document::from_str(input);

            assert_eq!(result.unwrap().main_lines, expected);
        }
    }

    mod main_line_from_str_tests {
        use super::super::*;

        #[test]
        fn find_basic_text_line() {
            let expected = MainLine::TextLine("Hello world!".to_string());

            let line = "Hello world!";

            let result = MainLine::from_str(line);

            assert_eq!(result, expected);
        }

        #[test]
        fn find_heading_line() {
            let expected = MainLine::HeadingLine(HeadingLevel::One, "Hello world!".to_string());

            let line = "# Hello world!";

            let result = MainLine::from_str(line);

            assert_eq!(result, expected);
        }

        #[test]
        fn find_heading6_line() {
            let expected = MainLine::HeadingLine(HeadingLevel::Six, "Hello world!".to_string());

            let line = "###### Hello world!";

            let result = MainLine::from_str(line);

            assert_eq!(result, expected);
        }

        #[test]
        fn find_text_line_that_looks_like_a_heading() {
            let expected = MainLine::TextLine("##########This is actually not a heading line".to_string());

            let line = "##########This is actually not a heading line";

            let result = MainLine::from_str(line);

            assert_eq!(result, expected);
        }
    }

    mod metadata_from_str_tests {
        use super::super::*;

        #[test]
        fn find_title() {
            let expected = "Test".to_string();

            let meta_section = "TI Test\n";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().title, expected);
        }

        #[test]
        fn find_subtitle() {
            let expected = Some("This is a subtitle".to_string());

            let meta_section = "TI Test\nST This is a subtitle";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().subtitle, expected);
        }

        #[test]
        fn find_0_authors() {
            let expected = None;

            let meta_section = "TI Test\n";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().author, expected);
        }

        #[test]
        fn find_1_author() {
            let expected = Some(vec!["Some author".to_string()]);

            let meta_section = "TI Test\nAU Some author\n";

            let result = Metadata::from_str(meta_section);

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

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().author, expected);
        }

        #[test]
        fn find_0_licenses() {
            let expected = None;

            let meta_section = "TI Test\n";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().license, expected);
        }

        #[test]
        fn find_1_license() {
            let expected = Some(vec!["GPL-3.0-or-later".to_string()]);

            let meta_section = "TI Test\nAU Some author\nLI GPL-3.0-or-later";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().license, expected);
        }

        #[test]
        fn find_multiple_licenses() {
            let expected = Some(vec![
                "GPL-3.0-or-later".to_string(),
                "CC-BY-SA-4.0".to_string(),
            ]);

            let meta_section = "TI Test\nLI GPL-3.0-or-later\nLI CC-BY-SA-4.0\n";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().license, expected);
        }

        #[test]
        fn find_multiple_languages() {
            let expected = Some(vec!["en".to_string(), "de".to_string()]);

            let meta_section = "TI Test\nLA en\nLA de\n";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().language, expected);
        }

        #[test]
        fn find_no_cache_duration() {
            let expected = None;

            let meta_section = "TI Test\n";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().cache, expected);
        }

        #[test]
        fn find_valid_cache_duration() {
            let expected = Some(100);

            let meta_section = "TI Test\nAU Some author\nLI GPL-3.0-or-later\nCH 100";

            let result = Metadata::from_str(meta_section);

            assert_eq!(result.unwrap().cache, expected);
        }

        #[test]
        fn find_invalid_cache_duration() {
            let meta_section = "TI Test\nAU Some author\nLI GPL-3.0-or-later\nCH 1o0";

            let result = Metadata::from_str(meta_section);

            assert!(result.is_err());
        }
    }
}
