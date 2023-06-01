#[derive(PartialEq, Debug)]
pub enum FormField {
    Submit(SubmitField),
    String(StringField),
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
    pub id: ID,
}

#[derive(PartialEq, Debug)]
pub struct GlobalProperties<T> {
    pub id: ID,
    pub optional: bool,
    pub label: String,
    pub default: T,
    pub conditional: ConditionalProperty,
}

// Field type structs
#[derive(PartialEq, Debug)]
pub struct SubmitField {
    pub id: ID,
    pub dest: String, // Like with Link this isnt parsed as a URL yet because it can be relative
    pub label: Option<String>,
    pub redirect: bool,
}

#[derive(PartialEq, Debug)]
pub struct StringField {
    pub global: GlobalProperties<String>,
    pub min: u32,
    pub max: u32,
    pub multiline: bool,
    pub secret: bool,
    pub allowed_variants: Option<Vec<String>>,
}

impl FormField {
    pub fn parse(input: &str) -> Result<FormField, &str> {
        use FormField::*;

        let (id, input) = input.split_once(":").ok_or("Form field with no ID found")?;
        let (field_type, input) = input.split_once(" \\").unwrap_or((input, ""));
        let properties: Vec<(&str, &str)> = input
            .split(" \\")
            .map(|property| property.split_once(" ").unwrap_or((property, "")))
            .collect();

        match field_type {
            "submit" => Ok(Submit(SubmitField {
                id: ID::new(id)?,

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
            })),
            _ => Err("Form field with invalid type found"),
        }
    }
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
    fn basic_string_field() {}

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
        let expected = FormField::Submit(SubmitField {
            id: ID::new("Send").unwrap(),
            dest: "/destination".to_string(),
            label: None,
            redirect: false,
        });

        let line = "Send:submit \\dest /destination";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }

    #[test]
    fn submit_field_with_redirect() {
        let expected = FormField::Submit(SubmitField {
            id: ID::new("Send").unwrap(),
            dest: "/destination".to_string(),
            label: None,
            redirect: true,
        });

        let line = "Send:submit \\dest /destination \\redirect";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }

    #[test]
    fn submit_field_with_redirect_and_label() {
        let expected = FormField::Submit(SubmitField {
            id: ID::new("Send").unwrap(),
            dest: "/destination".to_string(),
            label: Some("Click here to submit".to_string()),
            redirect: true,
        });

        let line = "Send:submit \\dest /destination \\redirect \\label Click here to submit";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }
}
