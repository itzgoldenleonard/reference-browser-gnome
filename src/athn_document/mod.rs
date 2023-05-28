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
    LinkLine(Link),
    PreformattedLine(bool, String),
    SeparatorLine,
    UListLine(Level, String),
    OListLine(Level, String, String),
    DropdownLine(String, String),
    HeadingLine(Level, String),
    QuoteLine(String),
}

#[derive(PartialEq, Debug)]
pub struct Link {
    // The Link doesnt use a Url type for its url component because relative URLs are allowed, and we dont know the base URL yet, the URL will have to be parsed later when we know its base
    pub url: String,
    pub label: Option<String>,
}

#[derive(PartialEq, Debug)]
pub enum HeaderLine {
    LinkLine(Link),
}

#[derive(PartialEq, Debug)]
pub enum FooterLine {
    LinkLine(Link),
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
            ("CH ", val) => Ok(self.cache(val.parse().map_err(|_| "Invalid cache tag value")?)),
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
                        builder.add_header_line(HeaderLine::LinkLine(Link::parse(val))),
                        current_section,
                    ),
                },
                Section::Footer => match current_line.split_once("=> ") {
                    Some((_, val)) => parse(
                        line,
                        builder.add_footer_line(FooterLine::LinkLine(Link::parse(val))),
                        current_section,
                    ),
                    None => parse(
                        line,
                        builder.add_footer_line(FooterLine::TextLine(current_line.to_string())),
                        current_section,
                    ),
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
            ("=> ", val) => Ok(LinkLine(Link::parse(val))),
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

impl Link {
    fn parse(input: &str) -> Link {
        // Takes the content of a link line and parses it into a Link object
        match input.split_once(" ") {
            Some((url, label)) => Link {
                url: url.to_string(),
                label: Some(label.to_string()),
            },
            None => Link {
                url: input.to_string(),
                label: None,
            },
        }
    }
}

#[cfg(test)]
mod tests;
