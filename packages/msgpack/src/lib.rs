use polywrap_serde::{from_slice, to_vec};
use serde::{de::DeserializeOwned, Serialize};

pub use polywrap_serde::error::Error;
pub use polywrap_serde::Map;

pub fn encode<T: Serialize>(value: T) -> Result<Vec<u8>, polywrap_serde::error::Error> {
    to_vec(&value)
}

pub fn decode<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, polywrap_serde::error::Error> {
    from_slice(bytes)
}

#[cfg(test)]
mod tests {
    use polywrap_serde::Map;
    use serde::{Deserialize, Serialize};

    use crate::{decode, encode};

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct ValueTest {
        pub code: i32,
        pub success: bool,
        pub payload: Map<String, Vec<String>>,
    }

    #[test]
    fn vec_to_struct() {
        let encoded_struct: [u8; 53] = [
            131, 164, 99, 111, 100, 101, 204, 200, 167, 115, 117, 99, 99, 101, 115, 115, 195, 167,
            112, 97, 121, 108, 111, 97, 100, 199, 25, 1, 129, 168, 102, 101, 97, 116, 117, 114,
            101, 115, 146, 165, 115, 101, 114, 100, 101, 167, 109, 115, 103, 112, 97, 99, 107,
        ];

        let response = decode::<ValueTest>(&encoded_struct);
        let mut payload = Map::new();
        payload.insert(
            "features".to_string(),
            vec!["serde".to_string(), "msgpack".to_string()],
        );
        let expected = ValueTest {
            code: 200,
            success: true,
            payload,
        };
        assert_eq!(response.unwrap(), expected);
    }

    #[test]
    fn struct_to_vec() {
        let mut payload = Map::new();
        payload.insert(
            "features".to_string(),
            vec!["serde".to_string(), "msgpack".to_string()],
        );
        let value = ValueTest {
            code: 200,
            success: true,
            payload,
        };

        let expected: [u8; 53] = [
            131, 164, 99, 111, 100, 101, 204, 200, 167, 115, 117, 99, 99, 101, 115, 115, 195, 167,
            112, 97, 121, 108, 111, 97, 100, 199, 25, 1, 129, 168, 102, 101, 97, 116, 117, 114,
            101, 115, 146, 165, 115, 101, 114, 100, 101, 167, 109, 115, 103, 112, 97, 99, 107,
        ];
        assert_eq!(encode(value).unwrap(), expected);
    }
}
