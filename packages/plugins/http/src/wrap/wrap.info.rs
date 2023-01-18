/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};
use serde_json::{json, from_value};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "Http".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "enumTypes": [
    {
      "constants": [
        "TEXT",
        "BINARY"
      ],
      "kind": 8,
      "type": "ResponseType"
    }
  ],
  "moduleType": {
    "kind": 128,
    "methods": [
      {
        "arguments": [
          {
            "kind": 34,
            "name": "url",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "url",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "request",
            "object": {
              "kind": 8192,
              "name": "request",
              "type": "Request"
            },
            "type": "Request"
          }
        ],
        "kind": 64,
        "name": "get",
        "required": true,
        "return": {
          "kind": 34,
          "name": "get",
          "object": {
            "kind": 8192,
            "name": "get",
            "type": "Response"
          },
          "type": "Response"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "url",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "url",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "request",
            "object": {
              "kind": 8192,
              "name": "request",
              "type": "Request"
            },
            "type": "Request"
          }
        ],
        "kind": 64,
        "name": "post",
        "required": true,
        "return": {
          "kind": 34,
          "name": "post",
          "object": {
            "kind": 8192,
            "name": "post",
            "type": "Response"
          },
          "type": "Response"
        },
        "type": "Method"
      }
    ],
    "type": "Module"
  },
  "objectTypes": [
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "name": "status",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "status",
            "required": true,
            "type": "Int"
          },
          "type": "Int"
        },
        {
          "kind": 34,
          "name": "statusText",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "statusText",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "map": {
            "key": {
              "kind": 4,
              "name": "headers",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "headers",
            "scalar": {
              "kind": 4,
              "name": "headers",
              "required": true,
              "type": "String"
            },
            "type": "Map<String, String>",
            "value": {
              "kind": 4,
              "name": "headers",
              "required": true,
              "type": "String"
            }
          },
          "name": "headers",
          "type": "Map<String, String>"
        },
        {
          "kind": 34,
          "name": "body",
          "scalar": {
            "kind": 4,
            "name": "body",
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "Response"
    },
    {
      "kind": 1,
      "properties": [
        {
          "kind": 34,
          "map": {
            "key": {
              "kind": 4,
              "name": "headers",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "headers",
            "scalar": {
              "kind": 4,
              "name": "headers",
              "required": true,
              "type": "String"
            },
            "type": "Map<String, String>",
            "value": {
              "kind": 4,
              "name": "headers",
              "required": true,
              "type": "String"
            }
          },
          "name": "headers",
          "type": "Map<String, String>"
        },
        {
          "kind": 34,
          "map": {
            "key": {
              "kind": 4,
              "name": "urlParams",
              "required": true,
              "type": "String"
            },
            "kind": 262146,
            "name": "urlParams",
            "scalar": {
              "kind": 4,
              "name": "urlParams",
              "required": true,
              "type": "String"
            },
            "type": "Map<String, String>",
            "value": {
              "kind": 4,
              "name": "urlParams",
              "required": true,
              "type": "String"
            }
          },
          "name": "urlParams",
          "type": "Map<String, String>"
        },
        {
          "enum": {
            "kind": 16384,
            "name": "responseType",
            "required": true,
            "type": "ResponseType"
          },
          "kind": 34,
          "name": "responseType",
          "required": true,
          "type": "ResponseType"
        },
        {
          "kind": 34,
          "name": "body",
          "scalar": {
            "kind": 4,
            "name": "body",
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "Request"
    }
  ],
  "version": "0.1"
})).unwrap()
  }
}
