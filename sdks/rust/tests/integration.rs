//! Integration tests for the Makoto Rust SDK.

use chrono::Utc;
use makoto::hash::{sha256_hex, MerkleTree};
use makoto::signing::{MakotoSigner, SignedAttestation};
use makoto::types::common::{
    CollectionMethod, Environment, HashAlgorithm, IsolationLevel, MakotoLevel, SourceType,
    TimeAlignment,
};
use makoto::types::dbom::{DatasetInfo, DbomDigest, Source, Dbom};
use makoto::types::origin::{Collector, CollectionMetadata, DataSchema, Origin, OriginAttestation};
use makoto::types::stream_window::{
    ChainDescriptor, CollectorDescriptor, IntegrityDescriptor, MerkleTreeDescriptor,
    StreamDescriptor, StreamWindowAttestation, WindowDescriptor, WindowMetadata,
};
use makoto::types::transform::{
    CodeReference, ExecutionMetadata, Executor, InputReference, TransformAttestation,
    TransformDefinition, VerificationInfo,
};
use makoto::types::{Digest, Subject};
use makoto::verification::{
    detect_attestation_type, verify_attestation_json, verify_origin_structure,
    verify_stream_window_structure, verify_transform_structure, AttestationType,
};

#[test]
fn test_origin_attestation_full() {
    // Create a complete origin attestation
    let origin = Origin::new(
        "https://api.partner-bank.com/v2/transactions",
        SourceType::Api,
        CollectionMethod::ScheduledPull,
        Utc::now(),
    )
    .with_geography("US-WEST-2");

    let collector = Collector::new("https://expanso.io/collectors/prod-west-collector-01")
        .with_platform("expanso")
        .with_environment(Environment::Production);

    let schema = DataSchema::new("json-lines");

    let metadata = CollectionMetadata {
        records_collected: Some(150_000),
        bytes_collected: Some(45_000_000),
        collection_duration: Some("PT45M".to_string()),
        ..Default::default()
    };

    let data_hash = sha256_hex(b"sample dataset content");
    let digest = Digest::new(&data_hash).with_record_count(150_000);

    let attestation = OriginAttestation::builder()
        .subject(Subject::new(
            "dataset:customer_transactions_2025q4",
            digest,
        ))
        .origin(origin)
        .collector(collector)
        .schema(schema)
        .metadata(metadata)
        .build()
        .expect("Failed to build origin attestation");

    // Verify structure
    let result = verify_origin_structure(&attestation);
    assert!(result.valid, "Verification failed: {:?}", result.messages);
    assert_eq!(result.level, Some(MakotoLevel::L1));

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&attestation).unwrap();
    let parsed: OriginAttestation = serde_json::from_str(&json).unwrap();
    assert_eq!(attestation, parsed);

    // Verify JSON detection
    let detected = detect_attestation_type(&json).unwrap();
    assert_eq!(detected, AttestationType::Origin);
}

#[test]
fn test_transform_attestation_full() {
    let input_hash = sha256_hex(b"input dataset");
    let output_hash = sha256_hex(b"transformed output");

    let input = InputReference::new("dataset:raw_transactions", Digest::new(&input_hash))
        .with_makoto_level(MakotoLevel::L2)
        .with_attestation_ref("https://attestations.example.com/origin/12345");

    let transform = TransformDefinition::new(
        "https://makoto.dev/transforms/anonymization",
        "PII Anonymization",
    )
    .with_version("2.1.0")
    .with_description("Removes personally identifiable information")
    .with_code_ref(
        CodeReference::new("git+https://github.com/example/transforms.git")
            .with_commit("abc123def456")
            .with_path("transforms/anonymization.py"),
    );

    let executor = Executor::new("https://expanso.io/executors/prod-001")
        .with_platform("expanso")
        .with_environment("production")
        .with_isolation(IsolationLevel::Container);

    let metadata = ExecutionMetadata {
        invocation_id: Some("inv-12345-67890".to_string()),
        started_on: Some(Utc::now()),
        finished_on: Some(Utc::now()),
        duration_seconds: Some(45.5),
        records_input: Some(150_000),
        records_output: Some(150_000),
        records_modified: Some(150_000),
        ..Default::default()
    };

    let verification = VerificationInfo {
        input_hash_verified: Some(true),
        transform_deterministic: Some(true),
        output_reproducible: Some(true),
    };

    let attestation = TransformAttestation::builder()
        .subject(Subject::new(
            "dataset:anonymized_transactions",
            Digest::new(&output_hash),
        ))
        .input(input)
        .transform(transform)
        .executor(executor)
        .metadata(metadata)
        .verification(verification)
        .build()
        .expect("Failed to build transform attestation");

    // Verify structure
    let result = verify_transform_structure(&attestation);
    assert!(result.valid, "Verification failed: {:?}", result.messages);
    assert_eq!(result.level, Some(MakotoLevel::L1));

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&attestation).unwrap();
    let parsed: TransformAttestation = serde_json::from_str(&json).unwrap();
    assert_eq!(attestation, parsed);

    // Verify JSON detection
    let detected = detect_attestation_type(&json).unwrap();
    assert_eq!(detected, AttestationType::Transform);
}

#[test]
fn test_stream_window_attestation_full() {
    // Build a Merkle tree from sample records
    let tree = MerkleTree::from_leaves(&[b"r1", b"r2", b"r3", b"r4"]);
    let root = tree.root_hex().unwrap();

    let stream = StreamDescriptor::new("iot_sensors")
        .with_source("mqtt://broker.example.com:1883")
        .with_topic("sensors/temperature/#")
        .with_partitions(vec!["partition-0".to_string(), "partition-1".to_string()]);

    let window = WindowDescriptor::tumbling("PT1M")
        .with_alignment(TimeAlignment::EventTime)
        .with_allowed_lateness("PT10S");

    let merkle = MerkleTreeDescriptor::new(HashAlgorithm::Sha256, tree.leaf_count() as u64, &root)
        .with_height(tree.height() as u32);

    let chain = ChainDescriptor::genesis("stream:iot_sensors:window_20251220_100000");

    let integrity = IntegrityDescriptor::new(merkle).with_chain(chain);

    let collector = CollectorDescriptor::new("https://expanso.io/collectors/edge-001");

    let metadata = WindowMetadata {
        processing_latency: Some("PT500MS".to_string()),
        late_records: Some(5),
        dropped_records: Some(0),
        ..Default::default()
    };

    let window_digest = Digest::new(&root).with_record_count(tree.leaf_count() as u64);

    let attestation = StreamWindowAttestation::builder()
        .subject(Subject::new(
            "stream:iot_sensors:window_20251220_100000",
            window_digest,
        ))
        .stream(stream)
        .window(window)
        .integrity(integrity)
        .collector(collector)
        .metadata(metadata)
        .build()
        .expect("Failed to build stream window attestation");

    // Verify structure
    let result = verify_stream_window_structure(&attestation);
    assert!(result.valid, "Verification failed: {:?}", result.messages);
    assert_eq!(result.level, Some(MakotoLevel::L1));

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&attestation).unwrap();
    let parsed: StreamWindowAttestation = serde_json::from_str(&json).unwrap();
    assert_eq!(attestation, parsed);

    // Verify JSON detection
    let detected = detect_attestation_type(&json).unwrap();
    assert_eq!(detected, AttestationType::StreamWindow);
}

#[test]
fn test_dbom_full() {
    let dataset = DatasetInfo::new(
        "fraud-detection-training",
        "3.0.0",
        Utc::now(),
        DbomDigest::new(sha256_hex(b"final dataset")),
        MakotoLevel::L2,
    )
    .with_description("Training dataset for fraud detection ML model");

    let source1 = Source::new(
        "customer_transactions",
        "https://makoto.dev/origin/v1",
        MakotoLevel::L2,
    );

    let source2 = Source::new(
        "merchant_data",
        "https://makoto.dev/origin/v1",
        MakotoLevel::L1,
    );

    let dbom = Dbom::builder()
        .id("urn:dbom:example.com:fraud-detection-training-v3")
        .dataset(dataset)
        .source(source1)
        .source(source2)
        .build()
        .expect("Failed to build DBOM");

    // Validate
    dbom.validate().expect("DBOM validation failed");

    // Serialize and deserialize
    let json = serde_json::to_string_pretty(&dbom).unwrap();
    let parsed: Dbom = serde_json::from_str(&json).unwrap();
    assert_eq!(dbom, parsed);

    // Verify JSON detection
    let detected = detect_attestation_type(&json).unwrap();
    assert_eq!(detected, AttestationType::Dbom);
}

#[test]
fn test_signing_workflow() {
    // Generate a signer
    let signer = MakotoSigner::generate();

    // Create an attestation
    let origin = Origin::new(
        "https://api.example.com/data",
        SourceType::Api,
        CollectionMethod::Pull,
        Utc::now(),
    );

    let collector = Collector::new("https://example.com/collector/001");

    let attestation = OriginAttestation::builder()
        .subject(Subject::new("dataset:test", Digest::new("a".repeat(64))))
        .origin(origin)
        .collector(collector)
        .build()
        .unwrap();

    // Sign the attestation
    let signed = SignedAttestation::sign(&attestation, &signer).unwrap();

    // Verify structure of signed envelope
    assert_eq!(signed.payload_type, "application/vnd.in-toto+json");
    assert_eq!(signed.signatures.len(), 1);
    assert_eq!(signed.signatures[0].keyid, signer.key_id());

    // Verify signature
    let verifier = signer.verifying_key();
    assert!(signed.verify(&verifier).unwrap());

    // Decode payload
    let decoded: OriginAttestation = signed.decode_payload().unwrap();
    assert_eq!(decoded, attestation);

    // Wrong verifier should fail
    let wrong_signer = MakotoSigner::generate();
    let wrong_verifier = wrong_signer.verifying_key();
    assert!(!signed.verify(&wrong_verifier).unwrap());
}

#[test]
fn test_merkle_tree_workflow() {
    // Create some records
    let records: Vec<Vec<u8>> = (0..1000)
        .map(|i| format!("record_{:04}", i).into_bytes())
        .collect();
    let record_refs: Vec<&[u8]> = records.iter().map(|r| r.as_slice()).collect();

    // Build tree
    let tree = MerkleTree::from_leaves(&record_refs);

    // Check properties
    assert_eq!(tree.leaf_count(), 1000);
    assert!(tree.height() > 0);

    let root = tree.root_hex().unwrap();
    assert_eq!(root.len(), 64);

    // Generate and verify proofs for random records
    for idx in [0, 100, 500, 999] {
        let proof = tree.proof(idx).unwrap();
        assert!(tree.verify_proof(&proof));

        // Convert to hex and back
        let hex_proof = proof.to_hex();
        assert_eq!(hex_proof.leaf_index, idx);
    }

    // Invalid index should fail
    assert!(tree.proof(1000).is_err());
}

#[test]
fn test_verify_attestation_json() {
    // Valid origin attestation
    let origin = Origin::new(
        "https://api.example.com/data",
        SourceType::Api,
        CollectionMethod::Pull,
        Utc::now(),
    );

    let collector = Collector::new("https://example.com/collector/001");

    let attestation = OriginAttestation::builder()
        .subject(Subject::new("dataset:test", Digest::new("a".repeat(64))))
        .origin(origin)
        .collector(collector)
        .build()
        .unwrap();

    let json = serde_json::to_string(&attestation).unwrap();
    let result = verify_attestation_json(&json);
    assert!(result.valid);
    assert_eq!(result.level, Some(MakotoLevel::L1));

    // Invalid JSON
    let result = verify_attestation_json("not json");
    assert!(!result.valid);

    // Unknown type
    let result = verify_attestation_json(r#"{"foo": "bar"}"#);
    assert!(!result.valid);
}

#[test]
fn test_signer_persistence() {
    // Generate and export
    let signer = MakotoSigner::generate();
    let key_bytes = signer.to_bytes();
    let key_id = signer.key_id().to_string();

    // Restore and verify
    let restored = MakotoSigner::from_bytes(&key_bytes).unwrap();
    assert_eq!(restored.key_id(), key_id);

    // Sign with original, verify with restored
    let data = b"test data";
    let signature = signer.sign(data).unwrap();
    let verifier = restored.verifying_key();
    assert!(verifier.verify(data, &signature).unwrap());
}
