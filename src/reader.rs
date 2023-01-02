use std::io::{Error, ErrorKind};
use serde::Deserialize;
use serde_json::Value;

pub async fn try_read<T: for<'de> Deserialize<'de>>(file_path: &str) -> Result<T, Error> {
    let value = match tokio::fs::read_to_string(file_path).await {
        Ok(file_content) => try_deserialize(file_content)?,
        Err(error) => {
            return Err(error);
        }
    };

    Ok(value)
}

fn try_deserialize<T: for<'de> Deserialize<'de>>(file_content: String) -> Result<T, Error> {
    match serde_json::from_str::<Value>(&file_content) {
        Ok(value) => match serde_json::from_value::<T>(value) {
            Ok(value) => Ok(value),
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("failed to deserialize file's content: {}", error),
                ));
            }
        },
        Err(error) => {
            Err(Error::new(
                ErrorKind::InvalidData,
                format!("failed to deserialize file's content: {}", error),
            ))
        }
    }
}

#[cfg(test)]
#[test]
pub fn converts_string_into_expected_value() {
    #[derive(Deserialize)]
    struct Example {
        a: u32,
        b: String,
        c: bool,
    }

    let first_json = r#"{"a": 1, "b": "2", "c": true}"#;
    let second_json = r#"{"a": 3, "b": "4", "c": false}"#;

    let first = try_deserialize::<Example>(first_json.to_string()).unwrap();
    let second = try_deserialize::<Example>(second_json.to_string()).unwrap();

    assert_eq!(first.a, 1);
    assert_eq!(first.b, "2");
    assert_eq!(first.c, true);

    assert_eq!(second.a, 3);
    assert_eq!(second.b, "4");
    assert_eq!(second.c, false);
}