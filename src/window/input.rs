use crate::athn_document::form::ID;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Input {
    #[serde(flatten)]
    pub id: ID,
    #[serde(flatten)]
    pub value: InputTypes,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "lowercase")]
pub enum InputTypes {
    Int(Option<i64>),
    Float(Option<f64>),
    String(Option<String>),
    Bool(Option<bool>),
    #[serde(with = "humantime_serde")]
    Date(Option<std::time::SystemTime>),
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

    /*
    #[test]
    fn serialize_serde() {
        let _vector = vec![
            Input {
                id: ID::new("age").unwrap(),
                value: InputTypes::Int(Some(0)),
            },
            Input {
                id: ID::new("other_int").unwrap(),
                value: InputTypes::Int(None),
            },
        ];

        let _input = Input {
            id: ID::new("age").unwrap(),
            value: InputTypes::Int(Some(0)),
            //value: InputTypes::Int(None),
        };

        let serialized = serde_json::to_string_pretty(&_vector).unwrap();
        println!("{}", serialized);

        assert!(serialized.is_empty());
    }
    */

    #[test]
    fn equal_to_id() {
        let id = ID::new("test_id").unwrap();

        let input = Input {
            id: ID::new("not_test_id").unwrap(),
            value: InputTypes::Int(None),
        };

        assert_ne!(input, id);
    }

    #[test]
    fn not_equal_to_id() {
        let id = ID::new("test_id").unwrap();

        let input = Input {
            id: ID::new("test_id").unwrap(),
            value: InputTypes::Int(None),
        };

        assert_eq!(input, id);
    }
}
