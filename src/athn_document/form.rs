#[derive(PartialEq, Debug)]
pub enum FormField {
    Submit(ID, SubmitField),
    String(ID, StringField),
    Integer(ID, IntField),
    Float(ID, FloatField),
    Boolean(ID, BoolField),
    File(ID, FileField),
    List(ID, ListField),
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
    pub max: Option<u64>,
    pub allowed_types: Option<Vec<String>>,
}

#[derive(PartialEq, Debug)]
pub struct ListField {
    pub global: GlobalProperties<u32>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub children: Option<Vec<ID>>,
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

        let boolean_property = |name: &str| properties.iter().find(|e| e.0 == name).is_some();

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

                    redirect: boolean_property("redirect"),
                },
            )),
            "string" => Ok(String(
                id,
                StringField {
                    global: GlobalProperties::<std::string::String>::parse(&properties, |s| {
                        Ok(s.to_string())
                    })?,

                    min: find_number_property(
                        &properties,
                        "min",
                        "String type form field with invalid min property value found",
                    )?,

                    max: find_number_property(
                        &properties,
                        "max",
                        "String type form field with invalid max property value found",
                    )?,

                    multiline: boolean_property("multiline"),
                    secret: boolean_property("secret"),

                    allowed_variants: match boolean_property("e") {
                        true => Some(
                            properties
                                .iter()
                                .filter(|e| e.0 == "e")
                                .map(|e| e.1.to_string())
                                .collect(),
                        ),
                        false => None,
                    },
                },
            )),
            "int" => Ok(Integer(
                id,
                IntField {
                    global: GlobalProperties::<i64>::parse(&properties, |s| {
                        Ok(s.parse().map_err(|_| {
                            "Integer type form field with invalid default property found"
                        })?)
                    })?,

                    min: find_number_property(
                        &properties,
                        "min",
                        "Integer type form field with invalid min property value found",
                    )?,

                    max: find_number_property(
                        &properties,
                        "max",
                        "Integer type form field with invalid max property value found",
                    )?,

                    step: find_number_property(
                        &properties,
                        "step",
                        "Integer type form field with invalid step property value found",
                    )?,

                    positive: boolean_property("positive"),
                },
            )),
            "float" => Ok(Float(
                id,
                FloatField {
                    global: GlobalProperties::<f64>::parse(&properties, |s| {
                        Ok(s.parse().map_err(|_| {
                            "Float type form field with invalid default property found"
                        })?)
                    })?,

                    min: find_number_property(
                        &properties,
                        "min",
                        "Float type form field with invalid min property value found",
                    )?,

                    max: find_number_property(
                        &properties,
                        "max",
                        "Float type form field with invalid max property value found",
                    )?,

                    step: find_number_property(
                        &properties,
                        "step",
                        "Float type form field with invalid step property value found",
                    )?,

                    positive: boolean_property("positive"),
                },
            )),
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

                    max: find_number_property(
                        &properties,
                        "max",
                        "File type form field with invalid max property value found",
                    )?,

                    allowed_types: match boolean_property("type") {
                        true => Some(
                            properties
                                .iter()
                                .filter(|e| e.0 == "type")
                                .map(|e| e.1.to_string())
                                .collect(),
                        ),
                        false => None,
                    },
                },
            )),
            "list" => Ok(List(
                id,
                ListField {
                    global: GlobalProperties::<u32>::parse(&properties, |s| {
                        Ok(s.parse().map_err(|_| {
                            "List type form field with invalid default property found"
                        })?)
                    })?,

                    min: find_number_property(
                        &properties,
                        "min",
                        "List type form field with invalid min property value found",
                    )?,

                    max: find_number_property(
                        &properties,
                        "max",
                        "List type form field with invalid max property value found",
                    )?,

                    children: match boolean_property("child") {
                        true => {
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
                        }
                        false => None,
                    },
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
                max: Some(500000),
                allowed_types: None,
            },
        );

        let line = "test:file \\l This is a test \\max 500000";

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
            "file:file \\l This is a test \\type image/jpg \\type image/png \\type image/webp";

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

        let line = "Test:bool \\?";

        let form = FormField::parse(line).unwrap();

        assert_eq!(form, expected);
    }

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
        let line = "Test:float \\max 100 \\d 1o";

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

        let line = "float:float \\l This is a test \\positive \\min 10 \\max 5000000 \\d 2000000.0 \\step 0.5";

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
        let line = "Test:int \\max 100 \\d 1o";

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

        let line = "int:int \\l This is a test \\positive \\min 10 \\max 500 \\d 200 \\step 5";

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
