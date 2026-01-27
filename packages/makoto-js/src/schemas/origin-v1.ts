/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * JSON Schema exported as TypeScript module.
 * Source: origin-v1.json
 */

const schema = {
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://makoto.dev/schemas/origin/v1",
  "title": "Makoto Origin Attestation",
  "description": "JSON Schema for Makoto Data Origin Attestation (origin/v1). Documents the provenance of data at its point of collection or creation.",
  "type": "object",
  "required": [
    "_type",
    "subject",
    "predicateType",
    "predicate"
  ],
  "properties": {
    "_type": {
      "type": "string",
      "const": "https://in-toto.io/Statement/v1",
      "description": "in-toto Statement type identifier"
    },
    "subject": {
      "type": "array",
      "minItems": 1,
      "description": "The dataset(s) this attestation describes",
      "items": {
        "$ref": "#/$defs/subject"
      }
    },
    "predicateType": {
      "type": "string",
      "const": "https://makoto.dev/origin/v1",
      "description": "Makoto origin predicate type identifier"
    },
    "predicate": {
      "$ref": "#/$defs/originPredicate"
    }
  },
  "$defs": {
    "subject": {
      "type": "object",
      "required": [
        "name",
        "digest"
      ],
      "properties": {
        "name": {
          "type": "string",
          "description": "Identifier for the dataset, typically in format 'dataset:<name>'",
          "examples": [
            "dataset:customer_transactions_2025q4"
          ]
        },
        "digest": {
          "type": "object",
          "description": "Cryptographic digests identifying the dataset contents",
          "required": [
            "sha256"
          ],
          "properties": {
            "sha256": {
              "type": "string",
              "pattern": "^[a-f0-9]{64}$",
              "description": "SHA-256 hash of the dataset contents"
            },
            "sha512": {
              "type": "string",
              "pattern": "^[a-f0-9]{128}$",
              "description": "SHA-512 hash of the dataset contents"
            },
            "recordCount": {
              "type": "string",
              "pattern": "^[0-9]+$",
              "description": "Number of records in the dataset (as string for large numbers)"
            },
            "merkleRoot": {
              "type": "string",
              "pattern": "^[a-f0-9]{64}$",
              "description": "Root hash of Merkle tree over dataset records"
            }
          },
          "additionalProperties": {
            "type": "string",
            "description": "Additional digest algorithms"
          }
        }
      }
    },
    "originPredicate": {
      "type": "object",
      "required": [
        "origin",
        "collector"
      ],
      "description": "The origin predicate containing provenance information",
      "properties": {
        "origin": {
          "$ref": "#/$defs/origin"
        },
        "collector": {
          "$ref": "#/$defs/collector"
        },
        "schema": {
          "$ref": "#/$defs/schema"
        },
        "metadata": {
          "$ref": "#/$defs/metadata"
        },
        "dtaCompliance": {
          "$ref": "#/$defs/dtaCompliance"
        }
      }
    },
    "origin": {
      "type": "object",
      "required": [
        "source",
        "sourceType",
        "collectionMethod",
        "collectionTimestamp"
      ],
      "description": "Information about where and how the data was collected",
      "properties": {
        "source": {
          "type": "string",
          "format": "uri",
          "description": "URI identifying the data source (API endpoint, database, file location, etc.)",
          "examples": [
            "https://api.partner-bank.com/v2/transactions"
          ]
        },
        "sourceType": {
          "type": "string",
          "enum": [
            "api",
            "database",
            "file",
            "stream",
            "manual",
            "sensor",
            "other"
          ],
          "description": "Category of the data source"
        },
        "collectionMethod": {
          "type": "string",
          "enum": [
            "pull",
            "push",
            "scheduled-pull",
            "event-driven",
            "batch-upload",
            "streaming",
            "manual"
          ],
          "description": "How the data was collected from the source"
        },
        "collectionTimestamp": {
          "type": "string",
          "format": "date-time",
          "description": "ISO 8601 timestamp when data collection occurred"
        },
        "geography": {
          "type": "string",
          "description": "Geographic region where data was collected (e.g., 'US-WEST-2', 'EU', 'APAC')",
          "examples": [
            "US-WEST-2",
            "EU-CENTRAL-1",
            "US"
          ]
        },
        "consent": {
          "$ref": "#/$defs/consent"
        }
      }
    },
    "consent": {
      "type": "object",
      "required": [
        "type"
      ],
      "description": "Consent and legal basis for data collection",
      "properties": {
        "type": {
          "type": "string",
          "enum": [
            "contractual",
            "consent",
            "legitimate-interest",
            "legal-obligation",
            "public-interest",
            "vital-interest"
          ],
          "description": "Legal basis for data collection (aligned with GDPR Article 6)"
        },
        "reference": {
          "type": "string",
          "format": "uri",
          "description": "URI to consent documentation, DPA, or legal agreement"
        },
        "obtained": {
          "type": "string",
          "format": "date-time",
          "description": "When consent/agreement was obtained"
        },
        "expires": {
          "type": "string",
          "format": "date-time",
          "description": "When consent/agreement expires (if applicable)"
        }
      }
    },
    "collector": {
      "type": "object",
      "required": [
        "id"
      ],
      "description": "Information about the system that collected the data",
      "properties": {
        "id": {
          "type": "string",
          "format": "uri",
          "description": "Unique identifier for the collector instance",
          "examples": [
            "https://expanso.io/collectors/prod-west-collector-01"
          ]
        },
        "version": {
          "type": "object",
          "description": "Version information for collector software components",
          "additionalProperties": {
            "type": "string"
          },
          "examples": [
            {
              "expanso-cli": "1.4.2",
              "collector-plugin": "2.1.0"
            }
          ]
        },
        "environment": {
          "type": "string",
          "enum": [
            "production",
            "staging",
            "development",
            "test"
          ],
          "description": "Deployment environment of the collector"
        },
        "platform": {
          "type": "string",
          "description": "Platform or runtime environment (e.g., 'expanso', 'kubernetes', 'aws-lambda')"
        }
      }
    },
    "schema": {
      "type": "object",
      "required": [
        "format"
      ],
      "description": "Schema information for the collected data",
      "properties": {
        "format": {
          "type": "string",
          "description": "Data format (e.g., 'json-lines', 'csv', 'parquet', 'avro')",
          "examples": [
            "json-lines",
            "csv",
            "parquet",
            "avro",
            "protobuf"
          ]
        },
        "schemaRef": {
          "type": "string",
          "format": "uri",
          "description": "URI to the schema definition"
        },
        "schemaDigest": {
          "type": "object",
          "description": "Digest of the schema for integrity verification",
          "properties": {
            "sha256": {
              "type": "string",
              "pattern": "^[a-f0-9]{64}$"
            }
          }
        },
        "schemaVersion": {
          "type": "string",
          "description": "Version of the schema"
        }
      }
    },
    "metadata": {
      "type": "object",
      "description": "Collection statistics and metrics",
      "properties": {
        "collectionDuration": {
          "type": "string",
          "pattern": "^P(?!$)(\\d+Y)?(\\d+M)?(\\d+W)?(\\d+D)?(T(?=\\d)(\\d+H)?(\\d+M)?(\\d+S)?)?$",
          "description": "ISO 8601 duration of the collection process",
          "examples": [
            "PT45M",
            "PT1H30M",
            "P1D"
          ]
        },
        "bytesCollected": {
          "type": "integer",
          "minimum": 0,
          "description": "Total bytes of data collected"
        },
        "recordsCollected": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of records successfully collected"
        },
        "recordsDropped": {
          "type": "integer",
          "minimum": 0,
          "description": "Number of records dropped during collection"
        },
        "errorRate": {
          "type": "number",
          "minimum": 0,
          "maximum": 1,
          "description": "Fraction of records that encountered errors (0.0 to 1.0)"
        },
        "startTime": {
          "type": "string",
          "format": "date-time",
          "description": "When collection started"
        },
        "endTime": {
          "type": "string",
          "format": "date-time",
          "description": "When collection completed"
        }
      }
    },
    "dtaCompliance": {
      "type": "object",
      "description": "D&TA Data Provenance Standards v1.0.0 compliance information",
      "properties": {
        "standardsVersion": {
          "type": "string",
          "description": "Version of D&TA standards being followed",
          "examples": [
            "1.0.0"
          ]
        },
        "sourceStandard": {
          "type": "object",
          "description": "D&TA Source Standard fields",
          "properties": {
            "datasetTitle": {
              "type": "string",
              "description": "Human-readable title for the dataset"
            },
            "datasetIssuer": {
              "type": "string",
              "description": "Organization that issued/provided the data"
            },
            "description": {
              "type": "string",
              "description": "Description of the dataset contents and purpose"
            }
          }
        },
        "provenanceStandard": {
          "type": "object",
          "description": "D&TA Provenance Standard fields",
          "properties": {
            "dataOriginGeography": {
              "type": "string",
              "description": "Geographic origin of the data"
            },
            "method": {
              "type": "string",
              "description": "Description of collection method"
            },
            "dataFormat": {
              "type": "string",
              "description": "Format of the data"
            }
          }
        },
        "useStandard": {
          "type": "object",
          "description": "D&TA Use Standard fields",
          "properties": {
            "confidentialityClassification": {
              "type": "string",
              "enum": [
                "public",
                "internal",
                "confidential",
                "restricted"
              ],
              "description": "Data confidentiality level"
            },
            "intendedDataUse": {
              "type": "string",
              "description": "Intended use for this data"
            },
            "license": {
              "type": "string",
              "description": "License or usage terms"
            }
          }
        }
      }
    }
  }
} as const;

export default schema;
