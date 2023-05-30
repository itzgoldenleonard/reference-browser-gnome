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
    AdmonitionLine(AdmonitionType, String),
    HeadingLine(Level, String),
    QuoteLine(String),
    FormFieldLine(i32, super::form::FormField),
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
pub struct Link {
    // The Link doesnt use a Url type for its url component because relative URLs are allowed, and we dont know the base URL yet, the URL will have to be parsed later when we know its base
    pub url: String,
    pub label: Option<String>,
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

#[derive(PartialEq, Debug)]
pub enum AdmonitionType {
    Note,
    Warning,
    Danger,
}

impl Link {
    pub fn parse(input: &str) -> Link {
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

impl MainLine {
    pub fn parse(input: &str) -> Result<MainLine, &str> {
        // Parses a string slice of a main line and returns the correct object.
        use AdmonitionType::*;
        use Level::*;
        use MainLine::*;

        if input.len() < 3 {
            return Ok(TextLine(input.to_string()));
        };

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
            // Admonitions
            ("_! ", val) => Ok(AdmonitionLine(Note, val.to_string())),
            ("*! ", val) => Ok(AdmonitionLine(Warning, val.to_string())),
            ("!! ", val) => Ok(AdmonitionLine(Danger, val.to_string())),
            // Headings
            ("1# ", val) => Ok(HeadingLine(One, val.to_string())),
            ("2# ", val) => Ok(HeadingLine(Two, val.to_string())),
            ("3# ", val) => Ok(HeadingLine(Three, val.to_string())),
            ("4# ", val) => Ok(HeadingLine(Four, val.to_string())),
            ("5# ", val) => Ok(HeadingLine(Five, val.to_string())),
            ("6# ", val) => Ok(HeadingLine(Six, val.to_string())),
            (">> ", val) => Ok(QuoteLine(val.to_string())),
            (_, _) => Ok(TextLine(input.to_string())),
        }
    }
}
