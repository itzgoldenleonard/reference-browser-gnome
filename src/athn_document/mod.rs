use url::Url;

#[derive(PartialEq, Debug)]
pub struct Document {
    pub metadata: Metadata,
    pub main: Vec<MainLine>,
    pub header: Option<Vec<HeaderLine>>,
    pub footer: Option<Vec<FooterLine>>,
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
    }
}

#[derive(Default)]
pub struct DocumentBuilder {
    pub metadata: MetadataBuilder,
    main: Vec<MainLine>,
    header: Option<Vec<HeaderLine>>,
    footer: Option<Vec<FooterLine>>,
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

impl Metadata {
    pub fn builder() -> MetadataBuilder {
        MetadataBuilder::default()
    }
}

#[derive(Default)]
pub struct MetadataBuilder {
    title: String,
    subtitle: Option<String>,
    author: Option<Vec<String>>,
    license: Option<Vec<String>>,
    language: Option<Vec<String>>,
    cache: Option<u32>,
}

#[derive(PartialEq, Debug)]
// A single line in the main section
pub enum MainLine {
    TextLine(String),
    LinkLine((Url, Option<String>)), // The LinkLine doesnt use a Url type for its url component
                                     // because relative URLs are allowed, and we dont know the
                                     // base URL yet, the URL will have to be parsed later
    PreformattedLine(bool, String),
    SeparatorLine,
    UListLine(Level, String),
    OListLine(Level, String, String),
    DropdownLine(String, String),
    HeadingLine(Level, String),
    QuoteLine(String),
}

#[derive(PartialEq, Debug)]
pub enum HeaderLine {
    LinkLine((Url, Option<String>)),
}

#[derive(PartialEq, Debug)]
pub enum FooterLine {
    LinkLine((Url, Option<String>)),
    TextLine(String),
}

#[derive(PartialEq, Debug)]
pub enum Level {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub enum Section {
    Meta,
    Main,
    Form,
    Header,
    Footer,
}

impl MetadataBuilder {
    pub fn new() -> MetadataBuilder {
        MetadataBuilder::default()
    }

    pub fn title(mut self, title: String) -> MetadataBuilder {
        self.title = title;
        self
    }

    pub fn subtitle(mut self, subtitle: String) -> MetadataBuilder {
        self.subtitle = Some(subtitle);
        self
    }

    pub fn add_author_unfailing(mut self, author: String) -> MetadataBuilder {
        // Add an author if there's room for one
        if self.author.clone().unwrap_or_default().len() == 16 {
            return self;
        };
        self.author = Some([self.author.unwrap_or(vec![]), vec![author]].concat());
        self
    }

    pub fn add_license_unfailing(mut self, license: String) -> MetadataBuilder {
        // Add a license if there's room for one
        if self.license.clone().unwrap_or_default().len() == 4 {
            return self;
        };
        self.license = Some([self.license.unwrap_or(vec![]), vec![license]].concat());
        self
    }

    pub fn add_language_unfailing(mut self, language: String) -> MetadataBuilder {
        // Add a language if there's room for one
        if self.language.clone().unwrap_or_default().len() == 256 {
            return self;
        };
        self.language = Some([self.language.unwrap_or(vec![]), vec![language]].concat());
        self
    }

    pub fn cache(mut self, cache: u32) -> MetadataBuilder {
        self.cache = Some(cache);
        self
    }

    pub fn build(self) -> Metadata {
        Metadata {
            title: self.title,
            subtitle: self.subtitle,
            author: self.author,
            license: self.license,
            language: self.language,
            cache: self.cache,
        }
    }

    fn parse(self, line: &str) -> Result<MetadataBuilder, &str> {
        // Parses a metadata line and returns a builder with the corresponding changes applied to
        // it. Panics if the input line is shorter than 3 bytes.
        match line.split_at(3) {
            ("TI ", val) => Ok(self.title(val.to_string())),
            ("ST ", val) => Ok(self.subtitle(val.to_string())),
            ("AU ", val) => Ok(self.add_author_unfailing(val.to_string())),
            ("LI ", val) => Ok(self.add_license_unfailing(val.to_string())),
            ("LA ", val) => Ok(self.add_language_unfailing(val.to_string())),
            ("CH ", val) => Ok(self.cache(val.parse().unwrap())), // TODO: Get rid of this unwrap
            (_, _) => Err("Invalid Metadata tag line encountered"),
        }
    }
}

impl DocumentBuilder {
    pub fn new() -> DocumentBuilder {
        DocumentBuilder::default()
    }

    pub fn add_main_line(mut self, line: MainLine) -> DocumentBuilder {
        self.main.push(line);
        self
    }

    pub fn add_header_line(mut self, line: HeaderLine) -> DocumentBuilder {
        match self.header.as_mut() {
            None => self.header = Some(vec![line]),
            Some(v) => v.push(line),
        }
        self
    }

    pub fn add_footer_line(mut self, line: FooterLine) -> DocumentBuilder {
        match self.footer.as_mut() {
            None => self.footer = Some(vec![line]),
            Some(v) => v.push(line),
        }
        self
    }

    pub fn build(self) -> Document {
        Document {
            metadata: self.metadata.build(),
            main: self.main,
            header: self.header,
            footer: self.footer,
        }
    }
}

pub fn parse(
    mut line: std::str::Lines,
    mut builder: DocumentBuilder,
    current_section: Section,
) -> Result<DocumentBuilder, &str> {
    match line.next() {
        // Reached the end of the document, we're done
        None => Ok(builder),
        Some(current_line) => {
            // Ignore empty lines
            if current_line.is_empty() {
                return parse(line, builder, current_section);
            };

            // Change the section if a section line is encountered
            if current_line.get(0..3).unwrap_or_default() == "+++" {
                use Section::*;
                match current_line {
                    "+++ Meta" => return parse(line, builder, Meta),
                    "+++ Header" => return parse(line, builder, Header),
                    "+++ Footer" => return parse(line, builder, Footer),
                    "+++ Form" => return parse(line, builder, Form),
                    _ => return parse(line, builder, Main),
                }
            }

            // Differentiate behavior based on the section
            match current_section {
                Section::Meta => {
                    // split_at will panic if the line is shorter than 3 bytes, it's not valid
                    // anyways if it is, so we can just Err if that happens
                    if current_line.len() < 3 {
                        return Err("Invalid Metadata tag line encountered (too short)");
                    };
                    builder.metadata = builder.metadata.parse(current_line)?;
                    parse(line, builder, current_section)
                }

                Section::Main | Section::Form => {
                    // split_at will panic if the line is shorter than 3 bytes, if it is we know
                    // that the line is a text line
                    if current_line.len() < 3 {
                        return parse(
                            line,
                            builder.add_main_line(MainLine::TextLine(current_line.to_string())),
                            current_section,
                        );
                    };
                    // Call the MainLine parser on it
                    parse(
                        line,
                        builder.add_main_line(MainLine::parse(current_line)?),
                        current_section,
                    )
                }
                Section::Header => match current_line.split_once("=> ") {
                    None => Err("Invalid header line encountered"),
                    Some((_, val)) => parse(
                        line,
                        builder
                            .add_header_line(HeaderLine::LinkLine(parse_link_line(val).unwrap())),
                        current_section,
                    ),
                },
                Section::Footer => match current_line.split_once("=> ") {
                    Some((_, val)) => parse(
                        line,
                        builder
                            .add_footer_line(FooterLine::LinkLine(parse_link_line(val).unwrap())),
                        current_section,
                    ),
                    None => parse(line, builder.add_footer_line(FooterLine::TextLine(current_line.to_string())), current_section)
                },
            }
        }
    }
}

impl MainLine {
    fn parse(input: &str) -> Result<MainLine, &str> {
        // Parses a string slice of a main line and returns the correct object.
        // Panics if the input line is shorter than 3 bytes.
        use Level::*;
        use MainLine::*;
        match input.split_at(3) {
            ("=> ", val) => Ok(LinkLine(parse_link_line(val).unwrap())), // TODO: Get rid of that unwrap
            // Preformatted lines
            ("```", val) => Ok(PreformattedLine(false, val.to_string())),
            ("'''", val) => Ok(PreformattedLine(true, val.to_string())),
            ("---", _) => Ok(SeparatorLine),
            // Unordered lists
            ("1* ", val) => Ok(UListLine(One, val.to_string())),
            ("2* ", val) => Ok(UListLine(Two, val.to_string())),
            ("3* ", val) => Ok(UListLine(Three, val.to_string())),
            ("4* ", val) => Ok(UListLine(Four, val.to_string())),
            ("5* ", val) => Ok(UListLine(Five, val.to_string())),
            ("6* ", val) => Ok(UListLine(Six, val.to_string())),
            // Ordered lists (need the split_by_separator function)
            ("1- ", val) => {
                let (bullet, content) = val
                    .split_once(" ")
                    .ok_or("Invalid ordered list line found")?;
                Ok(OListLine(One, bullet.to_string(), content.to_string()))
            }
            ("2- ", val) => {
                let (bullet, content) = val
                    .split_once(" ")
                    .ok_or("Invalid ordered list line found")?;
                Ok(OListLine(Two, bullet.to_string(), content.to_string()))
            }
            ("3- ", val) => {
                let (bullet, content) = val
                    .split_once(" ")
                    .ok_or("Invalid ordered list line found")?;
                Ok(OListLine(Three, bullet.to_string(), content.to_string()))
            }
            ("4- ", val) => {
                let (bullet, content) = val
                    .split_once(" ")
                    .ok_or("Invalid ordered list line found")?;
                Ok(OListLine(Four, bullet.to_string(), content.to_string()))
            }
            ("5- ", val) => {
                let (bullet, content) = val
                    .split_once(" ")
                    .ok_or("Invalid ordered list line found")?;
                Ok(OListLine(Five, bullet.to_string(), content.to_string()))
            }
            ("6- ", val) => {
                let (bullet, content) = val
                    .split_once(" ")
                    .ok_or("Invalid ordered list line found")?;
                Ok(OListLine(Six, bullet.to_string(), content.to_string()))
            }
            ("\\/ ", val) => {
                let (label, content) = val
                    .split_once(" | ")
                    .ok_or("Dropdown line without ' | ' delimiter found")?;
                Ok(DropdownLine(label.to_string(), content.to_string()))
            }
            // Headings
            ("#1 ", val) => Ok(HeadingLine(One, val.to_string())),
            ("#2 ", val) => Ok(HeadingLine(Two, val.to_string())),
            ("#3 ", val) => Ok(HeadingLine(Three, val.to_string())),
            ("#4 ", val) => Ok(HeadingLine(Four, val.to_string())),
            ("#5 ", val) => Ok(HeadingLine(Five, val.to_string())),
            ("#6 ", val) => Ok(HeadingLine(Six, val.to_string())),
            (">> ", val) => Ok(QuoteLine(val.to_string())),
            (_, _) => Ok(TextLine(input.to_string())),
        }
    }
}

fn parse_link_line(input: &str) -> Result<(Url, Option<String>), url::ParseError> {
    // Takes the content of a link line and parses it into a LinkLine object
    Ok(match input.split_once(" ") {
        Some((url, label)) => (Url::parse(url)?, Some(label.to_string())),
        None => (Url::parse(input)?, None),
    })
}

#[cfg(test)]
mod tests {
    mod parse_tests {
        use super::super::*;

        #[test]
        fn basic_example() {
            let expected = Document::builder().build();

            let content = "\n+++ Meta\nTI Test\nST Subtitle test\nAU Author 1\nAU Author 2\nLI CC0-1.0\nLA en\nCH 0\n+++ Header\n=> /index.athn Homepage\n=> /about.athn About\n+++\n\n()\nLittle text line makes the test fail\n=> https://example.com/ Link line with label, the next one will be without\n=> https://localhost/\n```Preformatted line\n'''Textual preformatted line\n---\n1* Unordered list\n2* Subitem\n6* Subsubsubsubsubitem\n1- 1. Ordered list\n1- 2. With multiple lines\n2- a) And subitems\n\\/ Dropdown | This is a dropdown line\n#1 Heading 1\n#2 Heading 2\n#4 Heading 4\n>> I never said that  - Albert Einstein\n+++ Footer\nThis is just a boring old footer\n=> /privacy.athn Privacy policy";

            let document = parse(content.lines(), Document::builder(), Section::Main).unwrap();

            assert_eq!(document.build(), expected);
        }
    }

    mod metadatabuilder_tests {
        use super::super::*;

        #[test]
        fn build_test() {
            let expected_title = String::new();

            let metadata_obj = Metadata::builder().build();

            assert_eq!(expected_title, metadata_obj.title);
        }

        #[test]
        fn set_title() {
            let expected = String::from("Hello world!");

            let metadata_obj = Metadata::builder()
                .title("Hello world!".to_string())
                .build();

            assert_eq!(metadata_obj.title, expected);
        }

        #[test]
        fn set_subtitle() {
            let expected = Some(String::from("Hello world!"));

            let metadata_obj = Metadata::builder()
                .subtitle("Hello world!".to_string())
                .build();

            assert_eq!(metadata_obj.subtitle, expected);
        }

        #[test]
        fn set_single_author() {
            let expected = Some(vec!["Author 1".to_string()]);

            let metadata_obj = Metadata::builder()
                .add_author_unfailing("Author 1".to_string())
                .build();

            assert_eq!(metadata_obj.author, expected);
        }

        #[test]
        fn set_multiple_author() {
            let expected = Some(vec!["Author 1".to_string(), "Author 2".to_string()]);

            let metadata_obj = Metadata::builder()
                .add_author_unfailing("Author 1".to_string())
                .add_author_unfailing("Author 2".to_string())
                .build();

            assert_eq!(metadata_obj.author, expected);
        }

        #[test]
        fn set_too_many_authors() {
            let expected = Some(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
                "12".to_string(),
                "13".to_string(),
                "14".to_string(),
                "15".to_string(),
                "16".to_string(),
            ]);

            let metadata_obj = Metadata::builder()
                .add_author_unfailing("1".to_string())
                .add_author_unfailing("2".to_string())
                .add_author_unfailing("3".to_string())
                .add_author_unfailing("4".to_string())
                .add_author_unfailing("5".to_string())
                .add_author_unfailing("6".to_string())
                .add_author_unfailing("7".to_string())
                .add_author_unfailing("8".to_string())
                .add_author_unfailing("9".to_string())
                .add_author_unfailing("10".to_string())
                .add_author_unfailing("11".to_string())
                .add_author_unfailing("12".to_string())
                .add_author_unfailing("13".to_string())
                .add_author_unfailing("14".to_string())
                .add_author_unfailing("15".to_string())
                .add_author_unfailing("16".to_string())
                .add_author_unfailing("17".to_string())
                .add_author_unfailing("18".to_string())
                .build();

            assert_eq!(metadata_obj.author, expected);
        }

        #[test]
        fn set_license() {
            let expected = Some(vec!["CC0-1.0".to_string()]);

            let metadata_obj = Metadata::builder()
                .add_license_unfailing("CC0-1.0".to_string())
                .build();

            assert_eq!(metadata_obj.license, expected);
        }

        #[test]
        fn set_multiple_licenses() {
            let expected = Some(vec!["CC0-1.0".to_string(), "Unlicense".to_string()]);

            let metadata_obj = Metadata::builder()
                .add_license_unfailing("CC0-1.0".to_string())
                .add_license_unfailing("Unlicense".to_string())
                .build();

            assert_eq!(metadata_obj.license, expected);
        }

        #[test]
        fn set_too_many_licenses() {
            let expected = Some(vec![
                "CC0-1.0".to_string(),
                "CC0-1.0".to_string(),
                "CC0-1.0".to_string(),
                "CC0-1.0".to_string(),
            ]);

            let metadata_obj = Metadata::builder()
                .add_license_unfailing("CC0-1.0".to_string())
                .add_license_unfailing("CC0-1.0".to_string())
                .add_license_unfailing("CC0-1.0".to_string())
                .add_license_unfailing("CC0-1.0".to_string())
                .add_license_unfailing("CC0-1.0".to_string())
                .add_license_unfailing("Unlicense".to_string())
                .build();

            assert_eq!(metadata_obj.license, expected);
        }

        #[test]
        fn set_lang() {
            let expected = Some(vec!["en".to_string()]);

            let metadata_obj = Metadata::builder()
                .add_language_unfailing("en".to_string())
                .build();

            assert_eq!(metadata_obj.language, expected);
        }

        #[test]
        fn set_multiple_langs() {
            let expected = Some(vec!["en_US".to_string(), "en_GB".to_string()]);

            let metadata_obj = Metadata::builder()
                .add_language_unfailing("en_US".to_string())
                .add_language_unfailing("en_GB".to_string())
                .build();

            assert_eq!(metadata_obj.language, expected);
        }

        #[test]
        fn set_cache() {
            let expected = Some(100);

            let metadata_obj = Metadata::builder().cache(100).build();

            assert_eq!(metadata_obj.cache, expected);
        }
    }

    mod documentbuilder_tests {
        use super::super::*;

        #[test]
        fn build_test() {
            let expected_obj = Document {
                metadata: Metadata::builder().build(),
                main: vec![],
                header: None,
                footer: None,
            };

            let document_obj = Document::builder().build();

            assert_eq!(document_obj, expected_obj);
        }

        #[test]
        fn single_main_line() {
            use MainLine::*;

            let expected = vec![SeparatorLine];

            let document_obj = Document::builder().add_main_line(SeparatorLine).build();

            assert_eq!(document_obj.main, expected);
        }

        #[test]
        fn multiple_main_lines() {
            use MainLine::*;

            let expected = vec![
                SeparatorLine,
                HeadingLine(Level::One, "Line 2".to_string()),
                TextLine("Line 3".to_string()),
            ];

            let document_obj = Document::builder()
                .add_main_line(SeparatorLine)
                .add_main_line(HeadingLine(Level::One, "Line 2".to_string()))
                .add_main_line(TextLine("Line 3".to_string()))
                .build();

            assert_eq!(document_obj.main, expected);
        }

        #[test]
        fn single_header_line() {
            use HeaderLine::*;

            let expected = Some(vec![LinkLine((
                Url::parse("https://localhost:3000/").unwrap(),
                None,
            ))]);

            let document_obj = Document::builder()
                .add_header_line(LinkLine((
                    Url::parse("https://localhost:3000/").unwrap(),
                    None,
                )))
                .build();

            assert_eq!(document_obj.header, expected);
        }

        #[test]
        fn multiple_header_lines() {
            use HeaderLine::*;

            let expected = Some(vec![
                LinkLine((Url::parse("https://localhost:3000/").unwrap(), None)),
                LinkLine((
                    Url::parse("https://localhost:3000/index.athn").unwrap(),
                    Some("index".to_string()),
                )),
            ]);

            let document_obj = Document::builder()
                .add_header_line(LinkLine((
                    Url::parse("https://localhost:3000/").unwrap(),
                    None,
                )))
                .add_header_line(LinkLine((
                    Url::parse("https://localhost:3000/index.athn").unwrap(),
                    Some("index".to_string()),
                )))
                .build();

            assert_eq!(document_obj.header, expected);
        }

        #[test]
        fn single_footer_line() {
            use FooterLine::*;

            let expected = Some(vec![LinkLine((
                Url::parse("https://localhost:3000/").unwrap(),
                None,
            ))]);

            let document_obj = Document::builder()
                .add_footer_line(LinkLine((
                    Url::parse("https://localhost:3000/").unwrap(),
                    None,
                )))
                .build();

            assert_eq!(document_obj.footer, expected);
        }

        #[test]
        fn multiple_footer_lines() {
            use FooterLine::*;

            let expected = Some(vec![
                LinkLine((Url::parse("https://localhost:3000/").unwrap(), None)),
                LinkLine((
                    Url::parse("https://localhost:3000/index.athn").unwrap(),
                    Some("index".to_string()),
                )),
            ]);

            let document_obj = Document::builder()
                .add_footer_line(LinkLine((
                    Url::parse("https://localhost:3000/").unwrap(),
                    None,
                )))
                .add_footer_line(LinkLine((
                    Url::parse("https://localhost:3000/index.athn").unwrap(),
                    Some("index".to_string()),
                )))
                .build();

            assert_eq!(document_obj.footer, expected);
        }
    }
}
