pub mod form;
use form::*;
pub mod line_types;
use line_types::*;

#[derive(PartialEq, Debug)]
pub struct Document {
    pub metadata: Metadata,
    pub main: Vec<MainLine>,
    pub header: Option<Vec<Link>>,
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
    header: Option<Vec<Link>>,
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

#[derive(Default)]
pub struct ParserState {
    current_section: Section,
    form_count: u32,
}

#[derive(Default)]
pub enum Section {
    #[default]
    Meta,
    Main,
    Form,
    Header,
    Footer,
}

pub fn parse(
    mut line: std::str::Lines,
    mut builder: DocumentBuilder,
    state: ParserState,
) -> Result<DocumentBuilder, &str> {
    match line.next() {
        // Reached the end of the document, we're done
        None => Ok(builder),
        Some(current_line) => {
            // Ignore empty lines
            if current_line.is_empty() {
                return parse(line, builder, state);
            };

            // Change the section if a section line is encountered
            if current_line.starts_with("+++") {
                use Section::*;
                match current_line {
                    "+++ Header" => {
                        return parse(
                            line,
                            builder,
                            ParserState {
                                current_section: Header,
                                ..state
                            },
                        )
                    }
                    "+++ Footer" => {
                        return parse(
                            line,
                            builder,
                            ParserState {
                                current_section: Footer,
                                ..state
                            },
                        )
                    }
                    "+++ Form" => {
                        return parse(
                            line,
                            builder,
                            ParserState {
                                current_section: Form,
                                form_count: state.form_count + 1,
                            },
                        )
                    }
                    _ => {
                        return parse(
                            line,
                            builder,
                            ParserState {
                                current_section: Main,
                                ..state
                            },
                        )
                    }
                }
            }

            // Differentiate behavior based on the section
            match state.current_section {
                Section::Meta => {
                    // split_at will panic if the line is shorter than 3 bytes, it's not valid
                    // anyways if it is, so we can just Err if that happens
                    if current_line.len() < 3 {
                        return Err("Invalid Metadata tag line encountered (too short)");
                    };
                    builder.metadata = builder.metadata.parse(current_line)?;
                    parse(line, builder, state)
                }

                Section::Main => parse(
                    line,
                    builder.add_main_line(MainLine::parse(current_line)?),
                    state,
                ),
                Section::Form => match current_line.split_once("???") {
                    Some((_, val)) => parse(
                        line,
                        builder.add_main_line(MainLine::FormFieldLine(
                            state.form_count,
                            FormField::parse(val)?,
                        )),
                        state,
                    ),
                    None => parse(
                        line,
                        builder.add_main_line(MainLine::parse(current_line)?),
                        state,
                    ),
                },
                Section::Header => match current_line.split_once("@@@") {
                    None => Err("Invalid header line encountered"),
                    Some((_, val)) => parse(
                        line,
                        builder.add_header_line(val.into()),
                        state,
                    ),
                },
                Section::Footer => match current_line.split_once("@@@") {
                    Some((_, val)) => parse(
                        line,
                        builder.add_footer_line(FooterLine::LinkLine(val.into())),
                        state,
                    ),
                    None => parse(
                        line,
                        builder.add_footer_line(FooterLine::TextLine(current_line.to_string())),
                        state,
                    ),
                },
            }
        }
    }
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
            ("TM ", val) => Ok(self.title(val.to_string())),
            ("SM ", val) => Ok(self.subtitle(val.to_string())),
            ("AM ", val) => Ok(self.add_author_unfailing(val.to_string())),
            ("RM ", val) => Ok(self.add_license_unfailing(val.to_string())),
            ("LM ", val) => Ok(self.add_language_unfailing(val.to_string())),
            ("CM ", val) => Ok(self.cache(val.parse().map_err(|_| "Invalid cache tag value")?)),
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

    pub fn add_header_line(mut self, line: Link) -> DocumentBuilder {
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

#[cfg(test)]
mod tests;
