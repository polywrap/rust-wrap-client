/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.
use polywrap_plugin::*;

pub fn get_manifest() -> WrapManifest {
  WrapManifest {
    name: "ethereum-wallet".to_string(),
    type_: "plugin".to_string(),
    version: "0.1".to_string(),
    abi: from_value::<WrapManifestAbi>(json!({
  "envType": {
    "kind": 65536,
    "properties": [
      {
        "kind": 34,
        "name": "connection",
        "object": {
          "kind": 8192,
          "name": "connection",
          "type": "Connection"
        },
        "type": "Connection"
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
            "name": "method",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "method",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "params",
            "scalar": {
              "kind": 4,
              "name": "params",
              "type": "JSON"
            },
            "type": "JSON"
          },
          {
            "kind": 34,
            "name": "connection",
            "object": {
              "kind": 8192,
              "name": "connection",
              "type": "Connection"
            },
            "type": "Connection"
          }
        ],
        "comment": "Send a remote RPC request to the registered provider",
        "kind": 64,
        "name": "request",
        "required": true,
        "return": {
          "kind": 34,
          "name": "request",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "request",
            "required": true,
            "type": "JSON"
          },
          "type": "JSON"
        },
        "type": "Method"
      },
      {
        "arguments": [
          {
            "kind": 34,
            "name": "txHash",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "txHash",
              "required": true,
              "type": "String"
            },
            "type": "String"
          },
          {
            "kind": 34,
            "name": "confirmations",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "confirmations",
              "required": true,
              "type": "UInt32"
            },
            "type": "UInt32"
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
          },
          {
            "kind": 34,
            "name": "connection",
            "object": {
              "kind": 8192,
              "name": "connection",
              "type": "Connection"
            },
            "type": "Connection"
          }
        ],
        "comment": "Wait for a transaction to be confirmed",
        "kind": 64,
        "name": "waitForTransaction",
        "required": true,
        "return": {
          "kind": 34,
          "name": "waitForTransaction",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "waitForTransaction",
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
            "name": "connection",
            "object": {
              "kind": 8192,
              "name": "connection",
              "type": "Connection"
            },
            "type": "Connection"
          }
        ],
        "comment": "Get the ethereum address of the signer. Return null if signer is missing.",
        "kind": 64,
        "name": "signerAddress",
        "required": true,
        "return": {
          "kind": 34,
          "name": "signerAddress",
          "scalar": {
            "kind": 4,
            "name": "signerAddress",
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
            "name": "message",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "message",
              "required": true,
              "type": "Bytes"
            },
            "type": "Bytes"
          },
          {
            "kind": 34,
            "name": "connection",
            "object": {
              "kind": 8192,
              "name": "connection",
              "type": "Connection"
            },
            "type": "Connection"
          }
        ],
        "comment": "Sign a message and return the signature. Throws if signer is missing.",
        "kind": 64,
        "name": "signMessage",
        "required": true,
        "return": {
          "kind": 34,
          "name": "signMessage",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "signMessage",
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
            "name": "rlp",
            "required": true,
            "scalar": {
              "kind": 4,
              "name": "rlp",
              "required": true,
              "type": "Bytes"
            },
            "type": "Bytes"
          },
          {
            "kind": 34,
            "name": "connection",
            "object": {
              "kind": 8192,
              "name": "connection",
              "type": "Connection"
            },
            "type": "Connection"
          }
        ],
        "comment": "  Sign a serialized unsigned transaction and return the signature. Throws if signer is missing.\nThis method requires a wallet-based signer with a private key, and is not needed for most use cases.\nTypically, transactions are sent by `request` and signed by the wallet.",
        "kind": 64,
        "name": "signTransaction",
        "required": true,
        "return": {
          "kind": 34,
          "name": "signTransaction",
          "required": true,
          "scalar": {
            "kind": 4,
            "name": "signTransaction",
            "required": true,
            "type": "String"
          },
          "type": "String"
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
          "name": "node",
          "scalar": {
            "kind": 4,
            "name": "node",
            "type": "String"
          },
          "type": "String"
        },
        {
          "kind": 34,
          "name": "networkNameOrChainId",
          "scalar": {
            "kind": 4,
            "name": "networkNameOrChainId",
            "type": "String"
          },
          "type": "String"
        }
      ],
      "type": "Connection"
    }
  ],
  "version": "0.1"
})).unwrap()
  }
}
