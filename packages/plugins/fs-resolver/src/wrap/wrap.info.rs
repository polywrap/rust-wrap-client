/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_manifest::versions::{WrapManifest, WrapManifestAbi};
use serde_json::{json, from_value};

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "FsResolver".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "importedEnumTypes": [
    {
      "constants": [
        "ASCII",
        "UTF8",
        "UTF16LE",
        "UCS2",
        "BASE64",
        "BASE64URL",
        "LATIN1",
        "BINARY",
        "HEX"
      ],
      "kind": 520,
      "namespace": "FileSystem",
      "nativeType": "Encoding",
      "type": "FileSystem_Encoding",
      "uri": "ens/fs.polywrap.eth"
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
          "name": "readFile",
          "required": true,
          "return": {
            "kind": 34,
            "name": "readFile",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "readFile",
              "required": true,
              "type": "Bytes"
            },
            "type": "Bytes"
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
            },
            {
              "enum": {
                "kind": 16384,
                "name": "encoding",
                "type": "FileSystem_Encoding"
              },
              "kind": 34,
              "name": "encoding",
              "type": "FileSystem_Encoding"
            }
          ],
          "kind": 64,
          "name": "readFileAsString",
          "required": true,
          "return": {
            "kind": 34,
            "name": "readFileAsString",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "readFileAsString",
              "required": true,
              "type": "String"
            },
            "type": "String"
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
          "name": "exists",
          "required": true,
          "return": {
            "kind": 34,
            "name": "exists",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "exists",
              "required": true,
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
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            },
            {
              "kind": 34,
              "name": "data",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "data",
                "required": true,
                "type": "Bytes"
              },
              "type": "Bytes"
            }
          ],
          "kind": 64,
          "name": "writeFile",
          "required": true,
          "return": {
            "kind": 34,
            "name": "writeFile",
            "scalar": {
              "kind": 4,
              "name": "writeFile",
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
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            },
            {
              "kind": 34,
              "name": "recursive",
              "scalar": {
                "kind": 4,
                "name": "recursive",
                "type": "Boolean"
              },
              "type": "Boolean"
            }
          ],
          "kind": 64,
          "name": "mkdir",
          "required": true,
          "return": {
            "kind": 34,
            "name": "mkdir",
            "scalar": {
              "kind": 4,
              "name": "mkdir",
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
              "name": "path",
              "required": true,
              "scalar": {
                "kind": 4,
                "name": "path",
                "required": true,
                "type": "String"
              },
              "type": "String"
            },
            {
              "kind": 34,
              "name": "recursive",
              "scalar": {
                "kind": 4,
                "name": "recursive",
                "type": "Boolean"
              },
              "type": "Boolean"
            },
            {
              "kind": 34,
              "name": "force",
              "scalar": {
                "kind": 4,
                "name": "force",
                "type": "Boolean"
              },
              "type": "Boolean"
            }
          ],
          "kind": 64,
          "name": "rm",
          "required": true,
          "return": {
            "kind": 34,
            "name": "rm",
            "scalar": {
              "kind": 4,
              "name": "rm",
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
          "name": "rmdir",
          "required": true,
          "return": {
            "kind": 34,
            "name": "rmdir",
            "scalar": {
              "kind": 4,
              "name": "rmdir",
              "type": "Boolean"
            },
            "type": "Boolean"
          },
          "type": "Method"
        }
      ],
      "namespace": "FileSystem",
      "nativeType": "Module",
      "type": "FileSystem_Module",
      "uri": "ens/fs.polywrap.eth"
    }
  ],
  "moduleType": {
    "imports": [
      {
        "type": "FileSystem_Module"
      },
      {
        "type": "FileSystem_Encoding"
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
