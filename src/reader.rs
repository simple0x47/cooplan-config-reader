use std::io::{Error, ErrorKind};
use serde::Deserialize;

pub async fn try_read<T: Deserialize>(file_path: &str) -> Result<T, Error> {
    let value = match tokio::fs::read_to_string(file_path).await {
        Ok(value) => match serde_json::from_str::<Api>(&value) {
            Ok(value) => value,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("failed to deserialize api file's content: {}", error),
                ));
            }
        },
        Err(error) => {
            return Err(error);
        }
    };

    Ok(value)
}