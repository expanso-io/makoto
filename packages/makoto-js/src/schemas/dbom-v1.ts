/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * JSON Schema exported as TypeScript module.
 * Source: dbom-v1.json
 */

const schema = {
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://makoto.dev/schemas/dbom-v1.json",
  "title": "Data Bill of Materials (DBOM)",
  "description": "A comprehensive manifest documenting the provenance, lineage, and compliance status of a dataset, including all source data and transformations applied.",
  "type": "object",
  "required": [
    "dbomVersion",
    "dbomId",
    "dataset",
    "sources"
  ],
  "properties": {
    "dbomVersion": {
      "type": "string",
      "description": "Version of the DBOM specification this document conforms to",
      "pattern": "^\\d+\\.\\d+\\.\\d+$",
      "examples": [
        "1.0.0"
      ]
    },
    "dbomId": {
      "type": "string",
      "description": "Unique identifier for this DBOM, typically a URN",
      "pattern": "^urn:dbom:.+$",
      "examples": [
        "urn:dbom:example.com:fraud-detection-training-v3"
      ]
    },
    "dataset": {
      "$ref": "#/$defs/dataset"
    },
    "sources": {
      "type": "array",
      "description": "List of source datasets that contribute to the final dataset",
      "minItems": 1,
      "items": {
        "$ref": "#/$defs/source"
      }
    },
    "transformations": {
      "type": "array",
      "description": "Ordered list of transformations applied to produce the final dataset",
      "items": {
        "$ref": "#/$defs/transformation"
      }
    },
    "lineageGraph": {
      "$ref": "#/$defs/lineageGraph"
    },
    "compliance": {
      "$ref": "#/$defs/compliance"
    },
    "verification": {
      "$ref": "#/$defs/verification"
    },
    "metadata": {
      "$ref": "#/$defs/metadata"
    }
  },
  "$defs": {
    "dataset": {
      "type": "object",
      "description": "Information about the final dataset described by this DBOM",
      "required": [
        "name",
        "version",
        "created",
        "digest",
        "makotoLevel"
      ],
      "properties": {
        "name": {
          "type": "string",
          "description": "Human-readable name of the dataset"
        },
        "version": {
          "type": "string",
          "description": "Version of the dataset",
          "pattern": "^\\d+\\.\\d+\\.\\d+$"
        },
        "description": {
          "type": "string",
          "description": "Human-readable description of the dataset's purpose and contents"
        },
        "created": {
          "type": "string",
          "format": "date-time",
          "description": "ISO 8601 timestamp when the dataset was created"
        },
        "creator": {
          "$ref": "#/$defs/creator"
        },
        "digest": {
          "$ref": "#/$defs/digest"
        },
        "makotoLevel": {
          "$ref": "#/$defs/makotoLevel"
        }
      }
    },
    "creator": {
      "type": "object",
      "description": "Entity responsible for creating the dataset",
      "properties": {
        "organization": {
          "type": "string",
          "description": "Name of the organization"
        },
        "contact": {
          "type": "string",
          "description": "Contact email or URI for the responsible party"
        }
      }
    },
    "digest": {
      "type": "object",
      "description": "Cryptographic digest and metadata about the dataset contents",
      "required": [
        "sha256"
      ],
      "properties": {
        "sha256": {
          "type": "string",
          "description": "SHA-256 hash of the dataset contents",
          "pattern": "^[a-fA-F0-9]{64}$|^[a-zA-Z0-9_]+$"
        },
        "sha512": {
          "type": "string",
          "description": "Optional SHA-512 hash of the dataset contents",
          "pattern": "^[a-fA-F0-9]{128}$"
        },
        "recordCount": {
          "type": [
            "string",
            "integer"
          ],
          "description": "Number of records in the dataset"
        },
        "format": {
          "type": "string",
          "description": "File format of the dataset",
          "examples": [
            "parquet",
            "csv",
            "json",
            "avro",
            "orc"
          ]
        },
        "sizeBytes": {
          "type": "integer",
          "description": "Size of the dataset in bytes",
          "minimum": 0
        }
      }
    },
    "makotoLevel": {
      "type": "string",
      "description": "Makoto attestation level achieved",
      "enum": [
        "L1",
        "L2",
        "L3"
      ]
    },
    "source": {
      "type": "object",
      "description": "A source dataset that contributes to the final dataset",
      "required": [
        "name",
        "attestationType",
        "makotoLevel"
      ],
      "properties": {
        "name": {
          "type": "string",
          "description": "Identifier for this source dataset"
        },
        "description": {
          "type": "string",
          "description": "Human-readable description of the source"
        },
        "attestationRef": {
          "type": "string",
          "format": "uri",
          "description": "URI reference to the attestation document for this source"
        },
        "attestationType": {
          "type": "string",
          "format": "uri",
          "description": "URI identifying the attestation predicate type",
          "examples": [
            "https://makoto.dev/origin/v1",
            "https://makoto.dev/transform/v1"
          ]
        },
        "makotoLevel": {
          "$ref": "#/$defs/makotoLevel"
        },
        "geography": {
          "type": "string",
          "description": "Geographic region where the data was collected or processed"
        },
        "consent": {
          "$ref": "#/$defs/consent"
        },
        "license": {
          "$ref": "#/$defs/license"
        },
        "contribution": {
          "$ref": "#/$defs/contribution"
        }
      }
    },
    "consent": {
      "type": "object",
      "description": "Information about consent for data usage",
      "properties": {
        "type": {
          "type": "string",
          "description": "Type of consent obtained",
          "enum": [
            "explicit",
            "contractual",
            "legitimate-interest",
            "public-task"
          ]
        },
        "reference": {
          "type": "string",
          "format": "uri",
          "description": "URI reference to the consent documentation"
        }
      }
    },
    "license": {
      "type": "object",
      "description": "License information for the source data",
      "properties": {
        "type": {
          "type": "string",
          "description": "Type of license",
          "examples": [
            "public-domain",
            "open-source",
            "commercial",
            "proprietary"
          ]
        },
        "identifier": {
          "type": "string",
          "description": "SPDX license identifier or similar standard identifier",
          "examples": [
            "CC0-1.0",
            "MIT",
            "Apache-2.0",
            "CC-BY-4.0"
          ]
        },
        "reference": {
          "type": "string",
          "format": "uri",
          "description": "URI reference to the license documentation"
        }
      }
    },
    "contribution": {
      "type": "object",
      "description": "Information about how much this source contributes to the final dataset",
      "properties": {
        "recordCount": {
          "type": "integer",
          "description": "Number of records contributed by this source",
          "minimum": 0
        },
        "recordPercentage": {
          "type": "number",
          "description": "Percentage of final dataset records from this source",
          "minimum": 0,
          "maximum": 100
        }
      }
    },
    "transformation": {
      "type": "object",
      "description": "A transformation step in the data pipeline",
      "required": [
        "order",
        "name",
        "attestationType",
        "makotoLevel",
        "inputs",
        "outputs"
      ],
      "properties": {
        "order": {
          "type": "integer",
          "description": "Sequence number of this transformation in the pipeline",
          "minimum": 1
        },
        "name": {
          "type": "string",
          "description": "Human-readable name for the transformation"
        },
        "description": {
          "type": "string",
          "description": "Detailed description of what the transformation does"
        },
        "attestationRef": {
          "type": "string",
          "format": "uri",
          "description": "URI reference to the attestation document for this transformation"
        },
        "attestationType": {
          "type": "string",
          "format": "uri",
          "description": "URI identifying the attestation predicate type"
        },
        "makotoLevel": {
          "$ref": "#/$defs/makotoLevel"
        },
        "inputs": {
          "type": "array",
          "description": "Names of input datasets consumed by this transformation",
          "items": {
            "type": "string"
          },
          "minItems": 1
        },
        "outputs": {
          "type": "array",
          "description": "Names of output datasets produced by this transformation",
          "items": {
            "type": "string"
          },
          "minItems": 1
        },
        "transformType": {
          "type": "string",
          "format": "uri",
          "description": "URI identifying the type of transformation",
          "examples": [
            "https://makoto.dev/transforms/anonymization",
            "https://makoto.dev/transforms/join",
            "https://makoto.dev/transforms/filter",
            "https://makoto.dev/transforms/aggregate",
            "https://makoto.dev/transforms/feature-engineering"
          ]
        }
      }
    },
    "lineageGraph": {
      "type": "object",
      "description": "Visual representation of the data lineage",
      "properties": {
        "format": {
          "type": "string",
          "description": "Format of the graph content",
          "enum": [
            "graphviz-dot",
            "mermaid",
            "json-ld",
            "cytoscape"
          ]
        },
        "content": {
          "type": "string",
          "description": "The graph content in the specified format"
        },
        "url": {
          "type": "string",
          "format": "uri",
          "description": "URL to an external lineage graph representation"
        }
      }
    },
    "compliance": {
      "type": "object",
      "description": "Compliance and regulatory status of the dataset",
      "properties": {
        "overallMakotoLevel": {
          "$ref": "#/$defs/makotoLevel"
        },
        "levelJustification": {
          "type": "string",
          "description": "Explanation of how the overall Makoto level was determined"
        },
        "privacyAssessment": {
          "$ref": "#/$defs/privacyAssessment"
        },
        "regulatoryCompliance": {
          "type": "array",
          "description": "List of regulatory compliance statuses",
          "items": {
            "$ref": "#/$defs/regulatoryStatus"
          }
        },
        "dtaCompliance": {
          "$ref": "#/$defs/dtaCompliance"
        }
      }
    },
    "privacyAssessment": {
      "type": "object",
      "description": "Privacy and anonymization assessment",
      "properties": {
        "piiRemoved": {
          "type": "boolean",
          "description": "Whether personally identifiable information has been removed"
        },
        "anonymizationVerified": {
          "type": "boolean",
          "description": "Whether anonymization has been verified"
        },
        "kAnonymity": {
          "type": "integer",
          "description": "k-anonymity level achieved",
          "minimum": 1
        },
        "lDiversity": {
          "type": "integer",
          "description": "l-diversity level achieved",
          "minimum": 1
        },
        "tCloseness": {
          "type": "number",
          "description": "t-closeness threshold achieved",
          "minimum": 0,
          "maximum": 1
        },
        "differentialPrivacy": {
          "type": "object",
          "description": "Differential privacy parameters if applicable",
          "properties": {
            "epsilon": {
              "type": "number",
              "minimum": 0
            },
            "delta": {
              "type": "number",
              "minimum": 0,
              "maximum": 1
            }
          }
        }
      }
    },
    "regulatoryStatus": {
      "type": "object",
      "description": "Compliance status for a specific regulation",
      "required": [
        "regulation",
        "status"
      ],
      "properties": {
        "regulation": {
          "type": "string",
          "description": "Name or identifier of the regulation",
          "examples": [
            "GDPR Article 35",
            "EU AI Act Article 10",
            "CCPA",
            "HIPAA"
          ]
        },
        "status": {
          "type": "string",
          "description": "Compliance status",
          "enum": [
            "compliant",
            "non-compliant",
            "partial",
            "not-applicable",
            "pending-review"
          ]
        },
        "notes": {
          "type": "string",
          "description": "Additional notes about the compliance status"
        },
        "assessmentDate": {
          "type": "string",
          "format": "date",
          "description": "Date of the compliance assessment"
        },
        "assessor": {
          "type": "string",
          "description": "Person or entity that performed the assessment"
        }
      }
    },
    "dtaCompliance": {
      "type": "object",
      "description": "D&TA Data Provenance Standards compliance",
      "properties": {
        "standardsVersion": {
          "type": "string",
          "description": "Version of D&TA standards this DBOM conforms to"
        },
        "allFieldsPresent": {
          "type": "boolean",
          "description": "Whether all required D&TA fields are present"
        }
      }
    },
    "verification": {
      "type": "object",
      "description": "Results of attestation chain verification",
      "properties": {
        "chainVerified": {
          "type": "boolean",
          "description": "Whether the complete attestation chain has been verified"
        },
        "allSignaturesValid": {
          "type": "boolean",
          "description": "Whether all cryptographic signatures are valid"
        },
        "attestationCount": {
          "type": "integer",
          "description": "Number of attestations in the chain",
          "minimum": 0
        },
        "verificationTimestamp": {
          "type": "string",
          "format": "date-time",
          "description": "When the verification was performed"
        },
        "verifier": {
          "type": "object",
          "description": "Tool that performed the verification",
          "properties": {
            "tool": {
              "type": "string",
              "description": "Name of the verification tool"
            },
            "version": {
              "type": "string",
              "description": "Version of the verification tool"
            }
          }
        },
        "errors": {
          "type": "array",
          "description": "List of verification errors if any",
          "items": {
            "type": "object",
            "properties": {
              "code": {
                "type": "string"
              },
              "message": {
                "type": "string"
              },
              "attestationRef": {
                "type": "string",
                "format": "uri"
              }
            }
          }
        }
      }
    },
    "metadata": {
      "type": "object",
      "description": "Metadata about the DBOM document itself",
      "properties": {
        "generator": {
          "type": "object",
          "description": "Tool that generated this DBOM",
          "properties": {
            "tool": {
              "type": "string",
              "description": "Name of the generator tool"
            },
            "version": {
              "type": "string",
              "description": "Version of the generator tool"
            }
          }
        },
        "created": {
          "type": "string",
          "format": "date-time",
          "description": "When this DBOM was generated"
        },
        "validUntil": {
          "type": "string",
          "format": "date-time",
          "description": "Expiration date for this DBOM"
        },
        "accessControl": {
          "type": "object",
          "description": "Access control settings for this DBOM",
          "properties": {
            "visibility": {
              "type": "string",
              "description": "Visibility level",
              "enum": [
                "public",
                "internal",
                "confidential",
                "restricted"
              ]
            },
            "allowedConsumers": {
              "type": "array",
              "description": "List of entities allowed to access this DBOM",
              "items": {
                "type": "string"
              }
            }
          }
        },
        "tags": {
          "type": "array",
          "description": "Optional tags for categorization",
          "items": {
            "type": "string"
          }
        }
      }
    }
  }
} as const;

export default schema;
