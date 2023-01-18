/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use wrap_manifest_schemas::versions::{WrapManifest, WrapManifestAbi};
use serde_json::{json, from_value};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "HttpResolver".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "importedEnumTypes": [
    {
      "constants": [
        "TEXT",
        "BINARY"
      ],
      "kind": 520,
      "namespace": "Http",
      "nativeType": "ResponseType",
      "type": "Http_ResponseType",
      "uri": "ens/http.polywrap.eth"
    }
  ],
  "importedModuleTypes": [
    {
      "isInterface": false,
      "kind": 256,
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
                "type": "Http_Request"
              },
              "type": "Http_Request"
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
              "type": "Http_Response"
            },
            "type": "Http_Response"
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
                "type": "Http_Request"
              },
              "type": "Http_Request"
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
              "type": "Http_Response"
            },
            "type": "Http_Response"
          },
          "type": "Method"
        }
      ],
      "namespace": "Http",
      "nativeType": "Module",
      "type": "Http_Module",
      "uri": "ens/http.polywrap.eth"
    }
  ],
  "importedObjectTypes": [
    {
      "kind": 1025,
      "namespace": "Http",
      "nativeType": "Request",
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
            "type": "Http_ResponseType"
          },
          "kind": 34,
          "name": "responseType",
          "required": true,
          "type": "Http_ResponseType"
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
      "type": "Http_Request",
      "uri": "ens/http.polywrap.eth"
    },
    {
      "kind": 1025,
      "namespace": "Http",
      "nativeType": "Response",
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
      "type": "Http_Response",
      "uri": "ens/http.polywrap.eth"
    }
  ],
  "moduleType": {
    "imports": [
      {
        "type": "Http_Module"
      },
      {
        "type": "Http_Request"
      },
      {
        "type": "Http_ResponseType"
      },
      {
        "type": "Http_Response"
      }
    ],
    "kind": 128,
    "methods": [
      {
        "arguments": [
          {
            "kind": 34,
            "name": "authority",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "authority",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          }
        ],
        "kind": 64,
        "name": "tryResolveUri",
        "required": true,
        "return": {
          "kind": 34,
          "name": "tryResolveUri",
          "object": {
            "kind": 8192,
            "name": "tryResolveUri",
            "type": "MaybeUriOrManifest"
          },
          "type": "MaybeUriOrManifest"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "path",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "path",
              "required": true,
              "type": "String"
            },
            "type": "String"
          }
        ],
        "kind": 64,
        "name": "getFile",
        "required": true,
        "return": {
          "kind": 34,
          "name": "getFile",
          "scalar": {
            "kind": 4,
            "name": "getFile",
            "type": "Bytes"
          },
          "type": "Bytes"
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
          "name": "uri",
          "scalar": {
            "kind": 4,
            "name": "uri",
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "manifest",
          "scalar": {
            "kind": 4,
            "name": "manifest",
            "type": "Bytes"
          },
          "type": "Bytes"
        }
      ],
      "type": "MaybeUriOrManifest"
    }
  ],
  "version": "0.1"
})).unwrap()
  }
}
