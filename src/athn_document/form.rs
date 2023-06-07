use email_address::EmailAddress;
use std::num::NonZeroU32;
use std::time::SystemTime;

#[derive(PartialEq, Debug)]
pub enum FormField {
    Submit(ID, SubmitField),
    String(ID, StringField),
    Integer(ID, IntField),
    Float(ID, FloatField),
    Boolean(ID, BoolField),
    File(ID, FileField),
    List(ID, ListField),
    Date(ID, DateField),
    Email(ID, EmailField),
    Phone(ID, TelField),
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
    pub destination: String, // Like with Link this isnt parsed as a URL yet because it can be relative
    pub label: Option<String>,
    pub redirect: bool,
}

#[derive(PartialEq, Debug)]
pub struct StringField {
    pub global: GlobalProperties<String>,
    pub min: Option<NonZeroU32>,
    pub max: Option<NonZeroU32>,
    pub multiline: bool,
    pub secret: bool,
    pub variant: Option<Vec<String>>,
}

#[derive(PartialEq, Debug)]
pub struct IntField {
    pub global: GlobalProperties<i64>,
    pub min: Option<i64>,
    pub max: Option<i64>,
    pub step: Option<i64>,
    pub positive: bool,
}

#[derive(PartialEq, Debug)]
pub struct FloatField {
    pub global: GlobalProperties<f64>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
    pub positive: bool,
}

#[derive(PartialEq, Debug)]
pub struct BoolField {
    pub global: GlobalProperties<bool>,
}

#[derive(PartialEq, Debug)]
pub struct FileField {
    pub global: GlobalProperties<()>, // The file field cant have a default value
    pub max: Option<NonZeroU32>,
    pub allowed_types: Option<Vec<String>>,
}

#[derive(PartialEq, Debug)]
pub struct ListField {
    pub global: GlobalProperties<NonZeroU32>,
    pub min: Option<NonZeroU32>,
    pub max: Option<NonZeroU32>,
    pub children: Option<Vec<ID>>,
}

#[derive(PartialEq, Debug)]
pub struct DateField {
    pub global: GlobalProperties<SystemTime>,
    pub min: Option<SystemTime>,
    pub max: Option<SystemTime>,
    pub time: bool,
    pub date: bool,
}

#[derive(PartialEq, Debug)]
pub struct EmailField {
    pub global: GlobalProperties<EmailAddress>,
}

#[derive(PartialEq, Debug)]
pub struct TelField {
    // The phone number isnt verified because I couldnt find a standard to verify against
    // https://crates.io/crates/phonenumber
    // https://docs.rs/phonenumber/0.3.2+8.13.9/phonenumber/#reexports
    // https://www.twilio.com/docs/glossary/what-e164
    // https://www.twilio.com/blog/international-phone-number-input-html-javascript
    pub global: GlobalProperties<String>,
    pub country: Option<String>,
}

fn input_property<'a, T>(
    properties: &Vec<(&str, &str)>,
    name: &str,
    converter: fn(&str) -> Result<T, &'a str>,
) -> Result<Option<T>, &'a str> {
    properties
        .iter()
        .find(|e| e.0 == name)
        .map(|e| Ok(converter(e.1)?))
        .transpose()
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

        // Property finding functions
        let property = |name: &str| properties.iter().find(|e| e.0 == name);
        let string_property = |name: &str| property(name).map(|e| e.1.to_string());
        let boolean_property = |name: &str| property(name).is_some();
        let uint_property = |name: &str| {
            Ok::<_, &str>(
                property(name)
                    .map(|e| e.1.parse::<NonZeroU32>())
                    .transpose()
                    .map_err(|_| "invalid integer property found")?,
            )
        };
        let list_property = |name: &str| {
            if boolean_property(name) {
                return Some(
                    properties
                        .iter()
                        .filter(|e| e.0 == name)
                        .map(|e| e.1.to_string())
                        .collect::<Vec<std::string::String>>(),
                );
            } else {
                return None;
            };
        };

        // Create an appropriate FormField based on the type found
        match field_type {
            "submit" => Ok(Submit(
                id,
                SubmitField {
                    destination: string_property("destination")
                        .ok_or("Submit type form field without destination found")?,
                    label: string_property("label"),
                    redirect: boolean_property("redirect"),
                },
            )),
            "string" => Ok(String(
                id,
                StringField {
                    global: GlobalProperties::<std::string::String>::parse(&properties, |s| {
                        Ok(s.to_string())
                    })?,
                    min: uint_property("min")?,
                    max: uint_property("max")?,
                    multiline: boolean_property("multiline"),
                    secret: boolean_property("secret"),
                    variant: list_property("variant"),
                },
            )),
            "int" => {
                let converter = |s: &str| -> Result<i64, &str> {
                    Ok(s.parse().map_err(|_| {
                        "Integer type form field with invalid input type property found"
                    })?)
                };

                Ok(Integer(
                    id,
                    IntField {
                        global: GlobalProperties::<i64>::parse(&properties, converter)?,
                        min: input_property(&properties, "min", converter)?,
                        max: input_property(&properties, "max", converter)?,
                        step: input_property(&properties, "step", converter)?,
                        positive: boolean_property("positive"),
                    },
                ))
            }
            "float" => {
                let converter = |s: &str| -> Result<f64, &str> {
                    Ok(s.parse().map_err(|_| {
                        "Float type form field with invalid input type property found"
                    })?)
                };
                Ok(Float(
                    id,
                    FloatField {
                        global: GlobalProperties::<f64>::parse(&properties, converter)?,
                        min: input_property(&properties, "min", converter)?,
                        max: input_property(&properties, "max", converter)?,
                        step: input_property(&properties, "step", converter)?,
                        positive: boolean_property("positive"),
                    },
                ))
            }
            "bool" => Ok(Boolean(
                id,
                BoolField {
                    global: GlobalProperties::<bool>::parse(&properties, |s| {
                        Ok(s.parse().map_err(|_| {
                            "Bool type form field with invalid default property found"
                        })?)
                    })?,
                },
            )),
            "file" => Ok(File(
                id,
                FileField {
                    global: GlobalProperties::<()>::parse(&properties, |_| {
                        Err("File type form field with default property found")
                    })?,
                    max: uint_property("max")?,
                    allowed_types: list_property("type"),
                },
            )),
            "list" => {
                let converter = |s: &str| -> Result<NonZeroU32, &str> {
                    Ok(s.parse().map_err(|_| {
                        "List type form field with invalid input type property found"
                    })?)
                };

                Ok(List(
                    id,
                    ListField {
                        global: GlobalProperties::<NonZeroU32>::parse(&properties, converter)?,
                        min: input_property(&properties, "min", converter)?,
                        max: input_property(&properties, "max", converter)?,
                        children: if boolean_property("child") {
                            let ids_result = properties
                                .iter()
                                .filter(|e| e.0 == "child")
                                .map(|e| ID::new(e.1));

                            if ids_result.clone().find(|e| e.is_err()).is_some() {
                                return Err("List field with invalid child ID found");
                            };
                            // This unwrap is safe because I just checked that all elements in the
                            // vector are Ok(ID)
                            Some(ids_result.map(|e| e.unwrap()).collect())
                        } else {
                            None
                        },
                    },
                ))
            }
            "date" => {
                let converter = |s: &str| -> Result<SystemTime, &str> {
                    match s {
                        "now" => Ok(SystemTime::now()),
                        val => val
                            .parse::<humantime::Timestamp>()
                            .map(|ts| ts.into())
                            .map_err(|_| {
                                "Date type form field with invalid input type property found"
                            }),
                    }
                };

                Ok(Date(
                    id,
                    DateField {
                        global: GlobalProperties::<SystemTime>::parse(&properties, converter)?,
                        min: input_property(&properties, "min", converter)?,
                        max: input_property(&properties, "max", converter)?,
                        date: boolean_property("date"),
                        time: boolean_property("time"),
                    },
                ))
            }
            "email" => Ok(Email(
                id,
                EmailField {
                    global: GlobalProperties::<EmailAddress>::parse(&properties, |s| {
                        Ok(s.parse().map_err(|_| {
                            "Email type form field with invalid default property found"
                        })?)
                    })?,
                },
            )),
            "tel" => Ok(Phone(
                id,
                TelField {
                    global: GlobalProperties::<std::string::String>::parse(&properties, |s| {
                        Ok(s.to_string())
                    })?,
                    country: string_property("country"),
                },
            )),
            _ => Err("Form field with invalid type found"),
        }
    }
}

impl<U> GlobalProperties<U> {
    pub fn parse<'a, T>(
        input: &Vec<(&str, &str)>,
        converter: fn(&str) -> Result<T, &'a str>,
    ) -> Result<GlobalProperties<T>, &'a str> {
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
                .map(|e| Ok::<T, &str>(converter(e.1)?))
                .transpose()?,

            conditional: input
                .iter()
                .find(|e| e.0 == "conditional" || e.0 == "!conditional")
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

mod tests {
    use super::*;

    #[test]
    fn basic_float_field() {
        let expected = FormField::Float(
            ID::new("Test").unwrap(),
            FloatField {
                global: GlobalProperties {
                    optional: false,
                    label: None,
                    default: None,
                    conditional: None,
                },
                min: None,
                max: Some(100.5),
                step: None,
                positive: false,
            },
        );

        let line = "Test:float \\max 100.5";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn invalid_float_field() {
        let line = "Test:float \\max 100 \\default 1o";

        let form = FormField::parse(line);

        assert!(form.is_err());
    }

    #[test]
    fn advanced_float_field() {
        let expected = FormField::Float(
            ID::new("float").unwrap(),
            FloatField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: Some(2000000.0),
                    conditional: None,
                },
                min: Some(10.0),
                max: Some(5000000.0),
                step: Some(0.5),
                positive: true,
            },
        );

        let line = "float:float \\label This is a test \\positive \\min 10 \\max 5000000 \\default 2000000.0 \\step 0.5";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn basic_int_field() {
        let expected = FormField::Integer(
            ID::new("Test").unwrap(),
            IntField {
                global: GlobalProperties {
                    optional: false,
                    label: None,
                    default: None,
                    conditional: None,
                },
                min: None,
                max: Some(100),
                step: None,
                positive: false,
            },
        );

        let line = "Test:int \\max 100";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn invalid_int_field() {
        let line = "Test:int \\max 100 \\default 1o";

        let form = FormField::parse(line);

        assert!(form.is_err());
    }

    #[test]
    fn advanced_int_field() {
        let expected = FormField::Integer(
            ID::new("int").unwrap(),
            IntField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: Some(200),
                    conditional: None,
                },
                min: Some(10),
                max: Some(500),
                step: Some(5),
                positive: true,
            },
        );

        let line =
            "int:int \\label This is a test \\positive \\min 10 \\max 500 \\default 200 \\step 5";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
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
                variant: None,
            },
        );

        let line = "string:string \\label This is a test";

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
                min: Some(10.try_into().unwrap()),
                max: Some(500.try_into().unwrap()),
                multiline: true,
                secret: false,
                variant: None,
            },
        );

        let line =
            "string:string \\label This is a test \\multiline \\min 10 \\max 500 \\default Fill me out \\!conditional other_id";

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
                variant: Some(vec![
                    "Variant 1".to_string(),
                    "Variant 2".to_string(),
                    "Variant 3".to_string(),
                ]),
            },
        );

        let line = "string:string \\label This is a test \\variant Variant 1 \\variant Variant 2 \\variant Variant 3";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn submit_field_with_dest() {
        let expected = FormField::Submit(
            ID::new("Send").unwrap(),
            SubmitField {
                destination: "/destination".to_string(),
                label: None,
                redirect: false,
            },
        );

        let line = "Send:submit \\destination /destination";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }

    #[test]
    fn submit_field_with_redirect() {
        let expected = FormField::Submit(
            ID::new("Send").unwrap(),
            SubmitField {
                destination: "/destination".to_string(),
                label: None,
                redirect: true,
            },
        );

        let line = "Send:submit \\destination /destination \\redirect";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }

    #[test]
    fn submit_field_with_redirect_and_label() {
        let expected = FormField::Submit(
            ID::new("Send").unwrap(),
            SubmitField {
                destination: "/destination".to_string(),
                label: Some("Click here to submit".to_string()),
                redirect: true,
            },
        );

        let line = "Send:submit \\destination /destination \\redirect \\label Click here to submit";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }

    #[test]
    fn default_tel_field() {
        let expected = FormField::Phone(
            ID::new("test").unwrap(),
            TelField {
                global: GlobalProperties {
                    optional: false,
                    label: None,
                    default: Some(String::from("+44 113 496 0000")),
                    conditional: None,
                },
                country: None,
            },
        );

        let line = "test:tel \\default +44 113 496 0000";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn basic_tel_field() {
        let expected = FormField::Phone(
            ID::new("test").unwrap(),
            TelField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: None,
                    conditional: None,
                },
                country: None,
            },
        );

        let line = "test:tel \\label This is a test";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn invalid_email_field() {
        let line = "Test:email \\default invalid.email";

        let form = FormField::parse(line);

        assert!(form.is_err());
    }

    #[test]
    fn default_email_field() {
        let expected = FormField::Email(
            ID::new("test").unwrap(),
            EmailField {
                global: GlobalProperties {
                    optional: false,
                    label: None,
                    default: Some("foo@example.com".parse().unwrap()),
                    conditional: None,
                },
            },
        );

        let line = "test:email \\default foo@example.com";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn basic_email_field() {
        let expected = FormField::Email(
            ID::new("test").unwrap(),
            EmailField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: None,
                    conditional: None,
                },
            },
        );

        let line = "test:email \\label This is a test";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn default_date_field() {
        let expected = FormField::Date(
            ID::new("test").unwrap(),
            DateField {
                global: GlobalProperties {
                    optional: true,
                    label: None,
                    default: Some(
                        "2023-04-10T12:00:00"
                            .parse::<humantime::Timestamp>()
                            .unwrap()
                            .into(),
                    ),
                    conditional: None,
                },
                min: None,
                max: None,
                time: false,
                date: false,
            },
        );

        let line = "test:date \\optional \\default 2023-04-10T12:00:00";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn now_date_field() {
        // This test is gonna fail because it takes ~15us to parse the line I dunno how to fix it
        let expected = FormField::Date(
            ID::new("test").unwrap(),
            DateField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: None,
                    conditional: None,
                },
                min: Some(SystemTime::now()),
                max: None,
                time: false,
                date: false,
            },
        );

        let line = "test:date \\min now \\label This is a test";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn basic_date_field() {
        let expected = FormField::Date(
            ID::new("test").unwrap(),
            DateField {
                global: GlobalProperties {
                    optional: false,
                    label: None,
                    default: None,
                    conditional: None,
                },
                min: Some(
                    "2023-06-01T00:00:00"
                        .parse::<humantime::Timestamp>()
                        .unwrap()
                        .into(),
                ),
                max: None,
                time: false,
                date: false,
            },
        );

        let line = "test:date \\min 2023-06-01T00:00:00";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn basic_file_field() {
        let expected = FormField::File(
            ID::new("test").unwrap(),
            FileField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: None,
                    conditional: None,
                },
                max: Some(500000.try_into().unwrap()),
                allowed_types: None,
            },
        );

        let line = "test:file \\label This is a test \\max 500000";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn enum_file_field() {
        let expected = FormField::File(
            ID::new("file").unwrap(),
            FileField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("This is a test".to_string()),
                    default: None,
                    conditional: None,
                },
                max: None,
                allowed_types: Some(vec![
                    "image/jpg".to_string(),
                    "image/png".to_string(),
                    "image/webp".to_string(),
                ]),
            },
        );

        let line =
            "file:file \\label This is a test \\type image/jpg \\type image/png \\type image/webp";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn boolean_field() {
        let expected = FormField::Boolean(
            ID::new("Test").unwrap(),
            BoolField {
                global: GlobalProperties {
                    optional: false,
                    label: Some("Boolean field".to_string()),
                    default: Some(true),
                    conditional: None,
                },
            },
        );

        let line = "Test:bool \\label Boolean field \\default true";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn optional_boolean_field() {
        let expected = FormField::Boolean(
            ID::new("Test").unwrap(),
            BoolField {
                global: GlobalProperties {
                    optional: true,
                    label: None,
                    default: None,
                    conditional: None,
                },
            },
        );

        let line = "Test:bool \\optional";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

    #[test]
    fn create_valid_id() {
        let valid_id = ID::new("valid_ID").unwrap();

        assert_eq!(valid_id.id(), "valid_ID".to_string());
    }

    #[test]
    fn create_invalid_id() {
        assert!(ID::new("1nv4lid_ID").is_err());
    }
}
