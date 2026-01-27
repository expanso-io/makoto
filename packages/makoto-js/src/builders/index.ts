/**
 * Makoto SDK Builders
 *
 * Fluent builder classes for constructing Makoto attestations and DBOMs.
 */

// Origin Attestation Builder
export {
  OriginAttestationBuilder,
  createSubject,
  createOrigin,
  createCollector,
  SourceTypes,
  CollectionMethods,
  ConsentTypes,
} from "./origin.js";
export type { Subject, Origin, Collector, Schema, Metadata as OriginMetadata, DtaCompliance, Consent } from "./origin.js";

// Transform Attestation Builder
export {
  TransformAttestationBuilder,
  createTransformSubject,
  createInputReference,
  createTransformDefinition,
  createExecutor,
  createExecutionMetadata,
  MakotoLevels,
  IsolationLevels,
} from "./transform.js";
export type {
  SubjectDigest,
  InputReference,
  InputDigest,
  TransformDefinition,
  CodeReference,
  Executor,
  ExecutionMetadata,
  VerificationInfo,
} from "./transform.js";

// Stream Window Predicate Builder
export {
  StreamWindowPredicateBuilder,
  createStreamDescriptor,
  createTumblingWindow,
  createSlidingWindow,
  createSessionWindow,
  createMerkleTree,
  createIntegrity,
  createChain,
  WindowTypes,
  TimeAlignments,
  HashAlgorithms,
} from "./stream-window.js";
export type {
  MakotoStreamWindowPredicate,
  StreamDescriptor,
  WindowDescriptor,
  IntegrityDescriptor,
  MerkleTreeDescriptor,
  ChainDescriptor,
  AggregatesDescriptor,
  CollectorDescriptor,
  MetadataDescriptor,
  VerificationDescriptor,
  WindowType,
  TimeAlignment,
  HashAlgorithm,
} from "./stream-window.js";

// DBOM Builder
export {
  DBOMBuilder,
  createDataset,
  createSource,
  createTransformation,
  createDigest,
  generateDbomId,
  calculateOverallMakotoLevel,
  DBOM_VERSION,
  LineageGraphFormats,
  ComplianceStatuses,
} from "./dbom.js";
export type {
  DataBillOfMaterialsDBOM,
  Dataset,
  Source,
  Transformation,
  LineageGraph,
  Compliance,
  Verification,
  Metadata as DBOMMetadata,
  MakotoLevel,
  Digest,
  Creator,
  License,
  Contribution,
  PrivacyAssessment,
  RegulatoryStatus,
  LineageGraphFormat,
  ComplianceStatus,
} from "./dbom.js";
