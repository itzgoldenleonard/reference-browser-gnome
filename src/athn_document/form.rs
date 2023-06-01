#[derive(PartialEq, Debug)]
pub enum FormField {
    Submit(ID, SubmitField),
    String(ID, StringField),
}

// Helper structs
#[derive(PartialEq, Debug)]
pub struct ID {
    id: String,
}

impl ID {
    pub fn new(input: &str) -> Result<ID, &str> {
        // If any character in input is not alphabetic and not an underscore then input is an invalid ID
        if input
            .chars()
            .find(|&c| !c.is_ascii_alphabetic() && Some(c) != char::from_u32(0x5f))
            .is_some()
        {
            return Err("Found form field with invalid ID");
        };
        Ok(ID {
            id: input.to_string(),
        })
    }

    pub fn id_cloned(&self) -> String {
        self.id.clone()
    }

    pub fn id(self) -> String {
        self.id
    }
}

#[derive(PartialEq, Debug)]
pub struct ConditionalProperty {
    pub inverse: bool, // inverse == true is a conditional not
    pub target: ID,
}

#[derive(PartialEq, Debug)]
pub struct GlobalProperties<T> {
    pub optional: bool,
    pub label: Option<String>,
    pub default: Option<T>,
    pub conditional: Option<ConditionalProperty>,
}

// Field type structs
#[derive(PartialEq, Debug)]
pub struct SubmitField {
    pub dest: String, // Like with Link this isnt parsed as a URL yet because it can be relative
    pub label: Option<String>,
    pub redirect: bool,
}

#[derive(PartialEq, Debug)]
pub struct StringField {
    pub global: GlobalProperties<String>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub multiline: bool,
    pub secret: bool,
    pub allowed_variants: Option<Vec<String>>,
}

impl FormField {
    pub fn parse(input: &str) -> Result<FormField, &str> {
        use FormField::*;

        // Turn the &str into a ID, type and properties variables that I can work with
        let (id, input) = input.split_once(":").ok_or("Form field with no ID found")?;
        let id = ID::new(id)?;
        let (field_type, input) = input.split_once(" \\").unwrap_or((input, ""));
        let properties: Vec<(&str, &str)> = input
            .split(" \\")
            .map(|property| property.split_once(" ").unwrap_or((property, "")))
            .collect();

        // Create an appropriate FormField based on the type found
        match field_type {
            "submit" => Ok(Submit(
                id,
                SubmitField {
                    dest: properties
                        .iter()
                        .find(|e| e.0 == "dest")
                        .ok_or("Submit type form field without destination found")?
                        .1
                        .to_string(),

                    label: properties
                        .iter()
                        .find(|e| e.0 == "label" || e.0 == "l")
                        .map(|e| e.1.to_string()),

                    redirect: properties.iter().find(|e| e.0 == "redirect").is_some(),
                },
            )),
            "string" => Ok(String(
                id,
                StringField {
                    global: GlobalProperties::<std::string::String>::parse(&properties, |s| {
                        s.to_string()
                    })?,

                    min: find_number_property(
                        &properties,
                        "min",
                        "String type form field with anvalid min property value found",
                    )?,

                    max: find_number_property(
                        &properties,
                        "max",
                        "String type form field with anvalid max property value found",
                    )?,

                    multiline: properties.iter().find(|e| e.0 == "multiline").is_some(), 
                    // TODO: Maybe it would be a good idea to have a function for this
                    secret: properties.iter().find(|e| e.0 == "secret").is_some(),

                    allowed_variants: match properties.iter().find(|e| e.0 == "e") {
                        Some(_) => Some(
                            properties
                                .iter()
                                .filter(|e| e.0 == "e")
                                .map(|e| e.1.to_string())
                                .collect(),
                        ),
                        None => None,
                    },
                },
            )),
            _ => Err("Form field with invalid type found"),
        }
    }
}

impl<U> GlobalProperties<U> {
    pub fn parse<T>(
        input: &Vec<(&str, &str)>,
        converter: fn(&str) -> T,
    ) -> Result<GlobalProperties<T>, &'static str> {
        Ok(GlobalProperties {
            optional: input
                .iter()
                .find(|e| e.0 == "optional" || e.0 == "?")
                .is_some(),

            label: input
                .iter()
                .find(|e| e.0 == "label" || e.0 == "l")
                .map(|e| e.1.to_string()),

            default: input
                .iter()
                .find(|e| e.0 == "default" || e.0 == "d")
                .map(|e| converter(e.1)),

            conditional: input
                .iter()
                .find(|e| e.0 == "c" || e.0 == "!c")
                .map(|e| {
                    Ok::<ConditionalProperty, &str>(ConditionalProperty {
                        inverse: e.0.starts_with("!"),
                        target: ID::new(e.1).map_err(|_| "Invalid ID")?,
                    })
                })
                .transpose()?,
        })
    }
}

fn find_number_property<'a, F: std::str::FromStr>(
    input: &Vec<(&str, &str)>,
    property_name: &str,
    err: &'a str,
) -> Result<Option<F>, &'a str> {
    input
        .iter()
        .find(|e| e.0 == property_name)
        .map(|e| e.1.parse())
        .transpose()
        .map_err(|_| err)
}

mod tests {
    use super::*;

    #[test]
    fn create_valid_id() {
        let valid_id = ID::new("valid_ID").unwrap();

        assert_eq!(valid_id.id(), "valid_ID".to_string());
    }

    #[test]
    fn create_invalid_id() {
        assert!(ID::new("1nv4lid_ID").is_err());
    }

    #[test]
    fn basic_string_field() {
        let expected = FormField::String(
            ID::new("string").unwrap(),
            StringField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: None,
                    conditional: None,
                },
                min: None,
                max: None,
                multiline: false,
                secret: false,
                allowed_variants: None,
            },
        );

        let line = "string:string \\l This is a test";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn advanced_string_field() {
        let expected = FormField::String(
            ID::new("string").unwrap(),
            StringField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: Some("Fill me out".to_string()),
                    conditional: Some(ConditionalProperty {
                        inverse: true,
                        target: ID::new("other_id").unwrap(),
                    }),
                },
                min: Some(10),
                max: Some(500),
                multiline: true,
                secret: false,
                allowed_variants: None,
            },
        );

        let line =
            "string:string \\l This is a test \\multiline \\min 10 \\max 500 \\d Fill me out \\!c other_id";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn enum_string_field() {
        let expected = FormField::String(
            ID::new("string").unwrap(),
            StringField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: None,
                    conditional: None,
                },
                min: None,
                max: None,
                multiline: false,
                secret: false,
                allowed_variants: Some(vec![
                    "Variant 1".to_string(),
                    "Variant 2".to_string(),
                    "Variant 3".to_string(),
                ]),
            },
        );

        let line = "string:string \\l This is a test \\e Variant 1 \\e Variant 2 \\e Variant 3";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn label_shorthand() {
        let line1 = "Send:submit \\dest /destination \\label Test";
        let line2 = "Send:submit \\dest /destination \\l Test";

        let form1 = FormField::parse(line1).unwrap();
        let form2 = FormField::parse(line2).unwrap();

        assert_eq!(form1, form2);
    }

    #[test]
    fn submit_field_with_dest() {
        let expected = FormField::Submit(
            ID::new("Send").unwrap(),
            SubmitField {
                dest: "/destination".to_string(),
                label: None,
                redirect: false,
            },
        );

        let line = "Send:submit \\dest /destination";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }

    #[test]
    fn submit_field_with_redirect() {
        let expected = FormField::Submit(
            ID::new("Send").unwrap(),
            SubmitField {
                dest: "/destination".to_string(),
                label: None,
                redirect: true,
            },
        );

        let line = "Send:submit \\dest /destination \\redirect";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }

    #[test]
    fn submit_field_with_redirect_and_label() {
        let expected = FormField::Submit(
            ID::new("Send").unwrap(),
            SubmitField {
                dest: "/destination".to_string(),
                label: Some("Click here to submit".to_string()),
                redirect: true,
            },
        );

        let line = "Send:submit \\dest /destination \\redirect \\label Click here to submit";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }
}
