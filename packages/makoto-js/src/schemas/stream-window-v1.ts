/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * JSON Schema exported as TypeScript module.
 * Source: stream-window-v1.json
 */

const schema = {
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://makoto.dev/schemas/stream-window/v1",
  "title": "Makoto Stream Window Predicate",
  "description": "JSON Schema for the makoto.dev/stream-window/v1 predicate type. This predicate captures integrity attestations for bounded windows of streaming data using Merkle trees and hash chaining.",
  "type": "object",
  "required": [
    "stream",
    "window",
    "integrity"
  ],
  "additionalProperties": false,
  "properties": {
    "stream": {
      "$ref": "#/$defs/streamDescriptor"
    },
    "window": {
      "$ref": "#/$defs/windowDescriptor"
    },
    "integrity": {
      "$ref": "#/$defs/integrityDescriptor"
    },
    "aggregates": {
      "$ref": "#/$defs/aggregatesDescriptor"
    },
    "collector": {
      "$ref": "#/$defs/collectorDescriptor"
    },
    "metadata": {
      "$ref": "#/$defs/metadataDescriptor"
    },
    "verification": {
      "$ref": "#/$defs/verificationDescriptor"
    }
  },
  "$defs": {
    "iso8601Duration": {
      "type": "string",
      "pattern": "^P(?!$)(\\d+Y)?(\\d+M)?(\\d+W)?(\\d+D)?(T(?=\\d)(\\d+H)?(\\d+M)?(\\d+(\\.\\d+)?S)?)?$",
      "description": "ISO 8601 duration format (e.g., PT1M for 1 minute, PT5S for 5 seconds)",
      "examples": [
        "PT1M",
        "PT5S",
        "PT1H30M",
        "P1D"
      ]
    },
    "iso8601Timestamp": {
      "type": "string",
      "format": "date-time",
      "description": "ISO 8601 timestamp in RFC 3339 format",
      "examples": [
        "2025-12-20T10:00:00Z",
        "2025-12-20T10:00:00.001Z"
      ]
    },
    "hashAlgorithm": {
      "type": "string",
      "enum": [
        "sha256",
        "sha384",
        "sha512",
        "sha3-256",
        "sha3-384",
        "sha3-512",
        "blake2b",
        "blake3"
      ],
      "description": "Cryptographic hash algorithm identifier"
    },
    "hashDigest": {
      "type": "string",
      "minLength": 32,
      "description": "Hexadecimal-encoded hash digest"
    },
    "windowId": {
      "type": "string",
      "pattern": "^stream:[a-zA-Z0-9_-]+:window_[0-9]{8}_[0-9]{6}$",
      "description": "Unique window identifier in format stream:<stream_id>:window_<YYYYMMDD>_<HHMMSS>",
      "examples": [
        "stream:iot_sensors:window_20251220_100000"
      ]
    },
    "streamDescriptor": {
      "type": "object",
      "description": "Identifies the data stream being attested",
      "required": [
        "id"
      ],
      "additionalProperties": false,
      "properties": {
        "id": {
          "type": "string",
          "minLength": 1,
          "description": "Unique identifier for the stream"
        },
        "source": {
          "type": "string",
          "format": "uri",
          "description": "URI of the stream source (e.g., mqtt://, kafka://, https://)"
        },
        "topic": {
          "type": "string",
          "description": "Topic pattern or name for the stream"
        },
        "partitions": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "minItems": 1,
          "uniqueItems": true,
          "description": "List of partition identifiers included in this window"
        }
      }
    },
    "windowDescriptor": {
      "type": "object",
      "description": "Defines the windowing parameters for stream aggregation",
      "required": [
        "type",
        "duration"
      ],
      "additionalProperties": false,
      "properties": {
        "type": {
          "type": "string",
          "enum": [
            "tumbling",
            "sliding",
            "session"
          ],
          "description": "Window type: tumbling (fixed non-overlapping), sliding (overlapping), or session (activity-based gaps)"
        },
        "duration": {
          "$ref": "#/$defs/iso8601Duration",
          "description": "Window duration (for tumbling/sliding) or session gap timeout"
        },
        "slide": {
          "$ref": "#/$defs/iso8601Duration",
          "description": "Slide interval for sliding windows (required when type is 'sliding')"
        },
        "alignment": {
          "type": "string",
          "enum": [
            "wall-clock",
            "event-time"
          ],
          "default": "event-time",
          "description": "Time alignment strategy: wall-clock (processing time) or event-time (data timestamps)"
        },
        "watermark": {
          "$ref": "#/$defs/iso8601Timestamp",
          "description": "Watermark timestamp indicating progress of event-time processing"
        },
        "allowedLateness": {
          "$ref": "#/$defs/iso8601Duration",
          "description": "Maximum allowed lateness for late-arriving records"
        }
      },
      "if": {
        "properties": {
          "type": {
            "const": "sliding"
          }
        }
      },
      "then": {
        "required": [
          "slide"
        ]
      }
    },
    "integrityDescriptor": {
      "type": "object",
      "description": "Cryptographic integrity information for the window",
      "required": [
        "merkleTree"
      ],
      "additionalProperties": false,
      "properties": {
        "merkleTree": {
          "$ref": "#/$defs/merkleTreeDescriptor"
        },
        "chain": {
          "$ref": "#/$defs/chainDescriptor"
        }
      }
    },
    "merkleTreeDescriptor": {
      "type": "object",
      "description": "Merkle tree parameters enabling efficient integrity verification of window records",
      "required": [
        "algorithm",
        "leafCount",
        "root"
      ],
      "additionalProperties": false,
      "properties": {
        "algorithm": {
          "$ref": "#/$defs/hashAlgorithm",
          "description": "Hash algorithm used for internal tree nodes"
        },
        "leafHashAlgorithm": {
          "$ref": "#/$defs/hashAlgorithm",
          "description": "Hash algorithm used for leaf nodes (defaults to 'algorithm' if not specified)"
        },
        "leafCount": {
          "type": "integer",
          "minimum": 1,
          "description": "Number of leaf nodes (records) in the Merkle tree"
        },
        "treeHeight": {
          "type": "integer",
          "minimum": 1,
          "description": "Height of the Merkle tree (ceil(log2(leafCount)) + 1)"
        },
        "root": {
          "$ref": "#/$defs/hashDigest",
          "description": "Root hash of the Merkle tree - this is the signed integrity value"
        }
      }
    },
    "chainDescriptor": {
      "type": "object",
      "description": "Hash chain linking this window to previous windows for tamper-evident sequencing",
      "additionalProperties": false,
      "properties": {
        "previousWindowId": {
          "type": "string",
          "description": "Identifier of the immediately preceding window"
        },
        "previousMerkleRoot": {
          "$ref": "#/$defs/hashDigest",
          "description": "Merkle root of the previous window (enables chain verification)"
        },
        "chainLength": {
          "type": "integer",
          "minimum": 1,
          "description": "Position in the chain (1 = genesis window)"
        },
        "genesisWindowId": {
          "type": "string",
          "description": "Identifier of the first window in this chain"
        }
      },
      "dependentRequired": {
        "previousWindowId": [
          "previousMerkleRoot"
        ],
        "previousMerkleRoot": [
          "previousWindowId"
        ]
      }
    },
    "aggregatesDescriptor": {
      "type": "object",
      "description": "Aggregate values computed over the window for quick verification and analysis",
      "additionalProperties": false,
      "properties": {
        "checksum": {
          "type": "string",
          "description": "Simple checksum of all records (for quick integrity check)"
        },
        "statistics": {
          "type": "object",
          "description": "Statistical aggregates over the window data",
          "properties": {
            "minTimestamp": {
              "$ref": "#/$defs/iso8601Timestamp",
              "description": "Earliest record timestamp in the window"
            },
            "maxTimestamp": {
              "$ref": "#/$defs/iso8601Timestamp",
              "description": "Latest record timestamp in the window"
            },
            "avgIntervalMs": {
              "type": "number",
              "minimum": 0,
              "description": "Average interval between records in milliseconds"
            }
          },
          "additionalProperties": true
        }
      }
    },
    "collectorDescriptor": {
      "type": "object",
      "description": "Information about the system that collected and attested this window",
      "required": [
        "id"
      ],
      "additionalProperties": false,
      "properties": {
        "id": {
          "type": "string",
          "format": "uri",
          "description": "Unique identifier URI for the collector"
        },
        "version": {
          "type": "object",
          "description": "Version information for collector software components",
          "additionalProperties": {
            "type": "string"
          }
        },
        "location": {
          "type": "string",
          "description": "Physical or logical location identifier of the collector"
        }
      }
    },
    "metadataDescriptor": {
      "type": "object",
      "description": "Operational metadata about window processing",
      "additionalProperties": true,
      "properties": {
        "processingLatency": {
          "$ref": "#/$defs/iso8601Duration",
          "description": "Time taken to process and close the window"
        },
        "lateRecords": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of records that arrived after the watermark but within allowed lateness"
        },
        "droppedRecords": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of records dropped (arrived after allowed lateness)"
        },
        "backpressureEvents": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of backpressure events during window processing"
        }
      }
    },
    "verificationDescriptor": {
      "type": "object",
      "description": "Information for verifying individual records within the window",
      "additionalProperties": false,
      "properties": {
        "merkleProofAvailable": {
          "type": "boolean",
          "description": "Whether Merkle proofs are available for individual record verification"
        },
        "proofEndpoint": {
          "type": "string",
          "format": "uri",
          "description": "Endpoint URL for retrieving Merkle proofs"
        }
      }
    }
  }
} as const;

export default schema;
