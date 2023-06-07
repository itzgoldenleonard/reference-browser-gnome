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
    FormFieldLine(u32, super::form::FormField),
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

impl TryFrom<u8> for Level {
    type Error = &'static str;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0x31 => Ok(Self::One),
            0x32 => Ok(Self::Two),
            0x33 => Ok(Self::Three),
            0x34 => Ok(Self::Four),
            0x35 => Ok(Self::Five),
            0x36 => Ok(Self::Six),
            _ => Err("Level unable to be constructed from given byte"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum AdmonitionType {
    Note,
    Warning,
    Danger,
}

impl TryFrom<u8> for AdmonitionType {
    type Error = &'static str;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            0x5f => Ok(Self::Note),
            0x2a => Ok(Self::Warning),
            0x21 => Ok(Self::Danger),
            _ => Err("AdmonitionType unable to be constructed from given byte"),
        }
    }
}

impl From<&str> for Link {
    fn from(input: &str) -> Self {
        // Takes the content of a link line and parses it into a Link object
        let (url, label) =
            split_delimited(input).map_or_else(|_| (input.into(), None), |v| (v.0, Some(v.1)));
        Self { url, label }
    }
}

fn split_delimited(input: &str) -> Result<(String, String), &str> {
    use tuple::Map;
    Ok(input
        .split_once(" | ")
        .ok_or("Incorrectly delimited line encountered")?
        .map(|e| e.into()))
}

impl MainLine {
    pub fn parse(input: &str) -> Result<MainLine, &str> {
        // Parses a string slice of a main line and returns the correct object.
        use MainLine::*;

        let text_line = || Ok(TextLine(input.into()));

        let lti: Vec<u8> = input.bytes().take(3).collect();
        if lti.len() < 3 {
            return text_line();
        };
        let content = input.get(3..).unwrap_or_default();

        // If it ends with space
        if lti[2] == 0x20 {
            // Try making the level object here and using if level.is_ok() in the match arms
            match lti[1] {
                0x2d if (0x31..=0x36).contains(&lti[0]) => {
                    Ok(UListLine(lti[0].try_into()?, content.into()))
                }
                0x2a if (0x31..=0x36).contains(&lti[0]) => Ok(OListLine(
                    lti[0].try_into()?,
                    split_delimited(content)?.0,
                    split_delimited(content)?.1,
                )),
                0x21 if lti[0] == 0x2a || lti[0] == 0x21 || lti[0] == 0x5f => {
                    Ok(AdmonitionLine(lti[0].try_into()?, content.into()))
                }
                0x23 if (0x31..=0x36).contains(&lti[0]) => {
                    Ok(HeadingLine(lti[0].try_into()?, content.into()))
                }
                _ => text_line(),
            }
        } else {
            if lti[0] != lti[1] || lti[1] != lti[2] {
                return text_line();
            };
            match lti[0] {
                0x40 => Ok(LinkLine(content.into())),
                0x3b => Ok(PreformattedLine(false, content.into())),
                0x27 => Ok(PreformattedLine(true, content.into())),
                0x3d => Ok(SeparatorLine),
                0x2e => Ok(DropdownLine(
                    split_delimited(content)?.0,
                    split_delimited(content)?.1,
                )),
                0x2f => Ok(QuoteLine(content.into())),
                _ => text_line(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dropdown_test() {
        let expected = MainLine::DropdownLine("Dropdown".to_string(), "Hidden content".to_string());

        let line = "...Dropdown | Hidden content";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn quote_test() {
        let expected = MainLine::QuoteLine("Quote".to_string());

        let line = "///Quote";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn text_preformatted_test() {
        let expected = MainLine::PreformattedLine(true, "Textual preformatted line".to_string());

        let line = "'''Textual preformatted line";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn preformatted_test() {
        let expected = MainLine::PreformattedLine(false, "".to_string());

        let line = ";;;";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn link_test() {
        let expected = MainLine::LinkLine(Link {
            url: "https://example.com/".to_string(),
            label: Some("Label".to_string()),
        });

        let line = "@@@https://example.com/ | Label";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn unlabeled_link_test() {
        let expected = MainLine::LinkLine(Link {
            url: "https://example.com/".to_string(),
            label: None,
        });

        let line = "@@@https://example.com/";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn separator_line() {
        let expected = MainLine::SeparatorLine;

        let line = "=== Something";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn admonition_test() {
        let expected = MainLine::AdmonitionLine(AdmonitionType::Warning, "Warning".to_string());

        let line = "*! Warning";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn olist_test() {
        let expected =
            MainLine::OListLine(Level::Three, "1.".to_string(), "Unordered list".to_string());

        let line = "3* 1. | Unordered list";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn ulist_test() {
        let expected = MainLine::UListLine(Level::Three, "Unordered list".to_string());

        let line = "3- Unordered list";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn heading_test() {
        let expected = MainLine::HeadingLine(Level::One, "Heading".to_string());

        let line = "1# Heading";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }

    #[test]
    fn short_text_line() {
        let expected = MainLine::TextLine("()".to_string());

        let line = "()";

        let parsed = MainLine::parse(line);

        assert_eq!(parsed.unwrap(), expected);
    }
}
