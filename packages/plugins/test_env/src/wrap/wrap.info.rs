/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};
use serde_json::{json, from_value};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "TestEnv".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "envType": {
    "kind": 65536,
    "properties": [
      {
        "kind": 34,
        "name": "foo",
        "required": true,
        "scalar": {
          "kind": 4,
          "name": "foo",
          "required": true,
          "type": "String"
        },
        "type": "String"
      },
      {
        "kind": 34,
        "name": "bar",
        "scalar": {
          "kind": 4,
          "name": "bar",
          "type": "Bytes"
        },
        "type": "Bytes"
      }
    ],
    "type": "Env"
  },
  "moduleType": {
    "kind": 128,
    "methods": [
      {
        "arguments": [
          {
            "kind": 34,
            "name": "arg",
            "scalar": {
              "kind": 4,
              "name": "arg",
              "type": "String"
            },
            "type": "String"
          }
        ],
        "env": {
          "required": true
        },
        "kind": 64,
        "name": "requiredEnv",
        "required": true,
        "return": {
          "kind": 34,
          "name": "requiredEnv",
          "scalar": {
            "kind": 4,
            "name": "requiredEnv",
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "arg",
            "scalar": {
              "kind": 4,
              "name": "arg",
              "type": "String"
            },
            "type": "String"
          }
        ],
        "env": {
          "required": false
        },
        "kind": 64,
        "name": "optEnv",
        "required": true,
        "return": {
          "kind": 34,
          "name": "optEnv",
          "scalar": {
            "kind": 4,
            "name": "optEnv",
            "type": "Boolean"
          },
          "type": "Boolean"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "arg",
            "scalar": {
              "kind": 4,
              "name": "arg",
              "type": "String"
            },
            "type": "String"
          }
        ],
        "kind": 64,
        "name": "noEnv",
        "required": true,
        "return": {
          "kind": 34,
          "name": "noEnv",
          "scalar": {
            "kind": 4,
            "name": "noEnv",
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
