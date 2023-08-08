/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::JSON::{from_value, json};
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};

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
          "comment": "The body of the request. If present, the `formData` property will be ignored.",
          "kind": 34,
          "name": "body",
          "scalar": {
            "kind": 4,
            "name": "body",
            "type": "String"
          },
          "type": "String"
        },
        {
          "array": {
            "item": {
              "kind": 8192,
              "name": "formData",
              "required": true,
              "type": "FormDataEntry"
            },
            "kind": 18,
            "name": "formData",
            "object": {
              "kind": 8192,
              "name": "formData",
              "required": true,
              "type": "FormDataEntry"
            },
            "type": "[FormDataEntry]"
          },
          "comment": "  An alternative to the standard request body, 'formData' is expected to be in the 'multipart/form-data' format.\nIf present, the `body` property is not null, `formData` will be ignored.\nOtherwise, if formData is not null, the following header will be added to the request: 'Content-Type: multipart/form-data'.",
          "kind": 34,
          "name": "formData",
          "type": "[FormDataEntry]"
        },
        {
          "kind": 34,
          "name": "timeout",
          "scalar": {
            "kind": 4,
            "name": "timeout",
            "type": "UInt32"
          },
          "type": "UInt32"
        }
      ],
      "type": "Request"
    },
    {
      "kind": 1,
      "properties": [
        {
          "comment": "FormData entry key",
          "kind": 34,
          "name": "name",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "name",
            "required": true,
            "type": "String"
          },
          "type": "String"
        },
        {
          "comment": "If 'type' is defined, value is treated as a base64 byte string",
          "kind": 34,
          "name": "value",
          "scalar": {
            "kind": 4,
            "name": "value",
            "type": "String"
          },
          "type": "String"
        },
        {
          "comment": "File name to report to the server",
          "kind": 34,
          "name": "fileName",
          "scalar": {
            "kind": 4,
            "name": "fileName",
            "type": "String"
          },
          "type": "String"
        },
        {
          "comment": "MIME type (https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types). Defaults to empty string.",
          "kind": 34,
          "name": "type",
          "scalar": {
            "kind": 4,
            "name": "type",
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "FormDataEntry"
    }
  ],
  "version": "0.1"
})).unwrap()
  }
}
