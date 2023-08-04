use crate::athn_document::form::ID;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct Input {
    #[serde(flatten)]
    pub id: ID,
    #[serde(flatten)]
    pub value: InputTypes,
    #[serde(flatten)]
    pub optional: InputOptional,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum InputTypes {
    Int(i64),
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum InputOptional {
    Required,
    Optional { empty: bool },
}

impl Serialize for InputOptional {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use InputOptional::*;
        let mut ss = serializer.serialize_struct("Optional", 2)?;
        match *self {
            Required => {
                ss.serialize_field("optional", &false)?;
                ss.serialize_field("empty", &())?;
                ss.end()
            }
            Optional { empty } => {
                ss.serialize_field("optional", &true)?;
                ss.serialize_field("empty", &empty)?;
                ss.end()
            }
        }
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
    fn serialize_serde() {
        let input = Input {
            id: ID::new("age").unwrap(),
            value: InputTypes::Int(0),
            //optional: InputOptional::Optional { empty: false },
            optional: InputOptional::Required,
        };

        let serialized = serde_json::to_string_pretty(&input).unwrap();
        println!("{}", serialized);

        assert!(serialized.is_empty());
    }

    /*
        #[test]
        fn empty_int_field() {
            let input = Input {
                id: ID::new("age").unwrap(),
                value: InputTypes::Int(0),
                optional: InputOptional::Optional { empty: true },
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
                optional: InputOptional::Required { empty: () },
            };

            let expected = r#""age": {
    "type": "int",
    "optional": false,
    "empty": null,
    "value": 18
    }"#;

            assert_eq!(input.serialize(), expected.to_string());
        }
    */

    #[test]
    fn equal_to_id() {
        let id = ID::new("test_id").unwrap();

        let input = Input {
            id: ID::new("not_test_id").unwrap(),
            value: InputTypes::Int(0),
            optional: InputOptional::Optional { empty: true },
        };

        assert_ne!(input, id);
    }

    #[test]
    fn not_equal_to_id() {
        let id = ID::new("test_id").unwrap();

        let input = Input {
            id: ID::new("test_id").unwrap(),
            value: InputTypes::Int(0),
            optional: InputOptional::Optional { empty: true },
        };

        assert_eq!(input, id);
    }
}
