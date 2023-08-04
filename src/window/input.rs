use crate::athn_document::form::ID;

#[derive(Debug)]
struct Input {
    pub id: ID,
    pub value: InputTypes,
    pub optional: InputOptional,
}

#[derive(Debug)]
enum InputTypes {
    Int(i64),
}

#[derive(PartialEq, Debug)]
enum InputOptional {
    Required,
    Optional(bool),
}

impl Input {
    pub fn serialize(&self) -> String {
        let mut serialized = String::new();

        let id_serialized = format!("\"{}\": ", self.id.id_cloned());
        serialized.push_str(&id_serialized);
        serialized.push_str("{\n");

        use InputTypes::*;
        let type_serialized = match self.value {
            Int(..) => "\"type\": \"int\",\n",
        };
        serialized.push_str(type_serialized);

        use InputOptional::*;
        let optional_serialized = match self.optional {
            Required => "\"optional\": false,\n\"empty\": null,\n",
            Optional(false) => "\"optional\": true,\n\"empty\": false,\n",
            Optional(true) => "\"optional\": true,\n\"empty\": true,\n",
        };
        serialized.push_str(optional_serialized);

        let empty = if self.optional == Optional(true) {
            true
        } else {
            false
        };
        let value_serialized = if empty {
            "\"value\": null\n".to_string()
        } else {
            match self.value {
                Int(v) => format!("\"value\": {}\n", v.to_string()),
            }
        };
        serialized.push_str(&value_serialized);
        serialized.push_str("}");

        serialized
    }
}

impl PartialEq<ID> for Input {
    fn eq(&self, other: &ID) -> bool {
        self.id == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::athn_document::form::ID;

    #[test]
    fn empty_int_field() {
        let input = Input {
            id: ID::new("age").unwrap(),
            value: InputTypes::Int(0),
            optional: InputOptional::Optional(true),
        };

        let expected = r#""age": {
"type": "int",
"optional": true,
"empty": true,
"value": null
}"#;

        assert_eq!(input.serialize(), expected.to_string());
    }

    #[test]
    fn int_field() {
        let input = Input {
            id: ID::new("age").unwrap(),
            value: InputTypes::Int(18),
            optional: InputOptional::Required,
        };

        let expected = r#""age": {
"type": "int",
"optional": false,
"empty": null,
"value": 18
}"#;

        assert_eq!(input.serialize(), expected.to_string());
    }

    #[test]
    fn equal_to_id() {
        let id = ID::new("test_id").unwrap();

        let input = Input {
            id: ID::new("not_test_id").unwrap(),
            value: InputTypes::Int(0),
            optional: InputOptional::Optional(true),
        };

        assert_ne!(input, id);
    }

    #[test]
    fn not_equal_to_id() {
        let id = ID::new("test_id").unwrap();

        let input = Input {
            id: ID::new("test_id").unwrap(),
            value: InputTypes::Int(0),
            optional: InputOptional::Optional(true),
        };

        assert_eq!(input, id);
    }
}
