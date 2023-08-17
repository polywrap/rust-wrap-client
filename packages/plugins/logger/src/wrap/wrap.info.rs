/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "logger-plugin".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "enumTypes": [
    {
      "constants": [
        "DEBUG",
        "INFO",
        "WARN",
        "ERROR"
      ],
      "kind": 8,
      "type": "LogLevel"
    }
  ],
  "moduleType": {
    "kind": 128,
    "methods": [
      {
        "arguments": [
          {
            "enum": {
              "kind": 16384,
              "name": "level",
              "required": true,
              "type": "LogLevel"
            },
            "kind": 34,
            "name": "level",
            "required": true,
            "type": "LogLevel"
          },
          {
            "kind": 34,
            "name": "message",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "message",
              "required": true,
              "type": "String"
            },
            "type": "String"
          }
        ],
        "kind": 64,
        "name": "log",
        "required": true,
        "return": {
          "kind": 34,
          "name": "log",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "log",
            "required": true,
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      }
    ],
    "type": "Module"
  },
  "version": "0.1"
})).unwrap()
  }
}
