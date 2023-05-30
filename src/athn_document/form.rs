#[derive(PartialEq, Debug)]
pub enum FormField {
    Submit(SubmitField),
}

#[derive(PartialEq, Debug)]
pub struct SubmitField {
    pub id: String,
    pub dest: String, // Like with Link this isnt parsed as a URL yet because it can be relative
    pub label: Option<String>,
    pub redirect: bool,
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
            "submit" => {
                Ok(Submit(SubmitField {
                    id: id.to_string(),

                    dest: properties
                        .iter()
                        .find(|e| e.0 == "dest")
                        .ok_or("Submit type form field without destination found")?
                        .1
                        .to_string(),

                    label: properties
                        .iter()
                        .find(|e| e.0 == "label")
                        .map(|e| e.1.to_string()),

                    redirect: properties.iter().find(|e| e.0 == "redirect").is_some(),
                }))
            }
            _ => Err("Form field with invalid type found"),
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn submit_field_with_dest() {
        let expected = FormField::Submit(SubmitField {
            id: "Send".to_string(),
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
            id: "Send".to_string(),
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
            id: "Send".to_string(),
            dest: "/destination".to_string(),
            label: Some("Click here to submit".to_string()),
            redirect: true,
        });

        let line = "Send:submit \\dest /destination \\redirect \\label Click here to submit";

        let document = FormField::parse(line).unwrap();

        assert_eq!(document, expected);
    }
}
