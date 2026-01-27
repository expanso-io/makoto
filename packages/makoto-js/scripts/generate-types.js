#!/usr/bin/env node
/**
 * Generates TypeScript types and schema modules from Makoto JSON schemas.
 *
 * This script:
 * 1. Reads JSON Schema files from ../../schemas/
 * 2. Generates TypeScript interfaces in src/generated/
 * 3. Generates TypeScript schema modules in src/schemas/
 */

import { compileFromFile } from "json-schema-to-typescript";
import { writeFile, mkdir, readFile } from "fs/promises";
import { dirname, join, resolve } from "path";
import { fileURLToPath } from "url";

const __dirname = dirname(fileURLToPath(import.meta.url));
const SCHEMAS_DIR = resolve(__dirname, "../../../schemas");
const TYPES_OUTPUT_DIR = resolve(__dirname, "../src/generated");
const SCHEMAS_OUTPUT_DIR = resolve(__dirname, "../src/schemas");

const SCHEMAS = [
  { input: "origin-v1.json", output: "origin.ts", schemaOutput: "origin-v1.ts", name: "OriginAttestation" },
  { input: "transform-v1.json", output: "transform.ts", schemaOutput: "transform-v1.ts", name: "TransformAttestation" },
  { input: "stream-window-v1.json", output: "stream-window.ts", schemaOutput: "stream-window-v1.ts", name: "StreamWindowPredicate" },
  { input: "dbom-v1.json", output: "dbom.ts", schemaOutput: "dbom-v1.ts", name: "DBOM" },
];

async function generateTypes() {
  // Ensure output directories exist
  await mkdir(TYPES_OUTPUT_DIR, { recursive: true });
  await mkdir(SCHEMAS_OUTPUT_DIR, { recursive: true });

  console.log("Generating TypeScript types and schemas from JSON schemas...\n");

  // Generate types and schema modules for each schema
  for (const schema of SCHEMAS) {
    const inputPath = join(SCHEMAS_DIR, schema.input);
    const typesOutputPath = join(TYPES_OUTPUT_DIR, schema.output);
    const schemaOutputPath = join(SCHEMAS_OUTPUT_DIR, schema.schemaOutput);

    try {
      // Generate TypeScript types
      const ts = await compileFromFile(inputPath, {
        bannerComment:
          "/* eslint-disable */\n/**\n * AUTO-GENERATED FILE - DO NOT EDIT\n * Generated from: " +
          schema.input +
          "\n */",
        style: {
          semi: true,
          singleQuote: false,
        },
        additionalProperties: false,
        strictIndexSignatures: true,
      });

      await writeFile(typesOutputPath, ts);
      console.log(`  + types/${schema.output} (from ${schema.input})`);

      // Generate schema module
      const schemaJson = await readFile(inputPath, "utf-8");
      // Parse and re-stringify to ensure clean JSON without trailing whitespace
      const cleanJson = JSON.stringify(JSON.parse(schemaJson), null, 2);
      const schemaModule = `/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * JSON Schema exported as TypeScript module.
 * Source: ${schema.input}
 */

const schema = ${cleanJson} as const;

export default schema;
`;

      await writeFile(schemaOutputPath, schemaModule);
      console.log(`  + schemas/${schema.schemaOutput}`);
    } catch (error) {
      console.error(`  ! Error generating ${schema.output}:`, error.message);
      process.exit(1);
    }
  }

  // Generate types index file - use namespace exports to avoid conflicts
  const typesIndexContent = `/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * Re-exports all generated types using namespaces to avoid naming conflicts.
 *
 * For direct type imports, import from the specific module:
 * import type { MakotoOriginAttestation, Subject } from "@makoto/sdk/generated/origin";
 */

// Origin attestation types
export type { MakotoOriginAttestation } from "./origin.js";
export * as origin from "./origin.js";

// Transform attestation types
export type { MakotoTransformAttestation } from "./transform.js";
export * as transform from "./transform.js";

// Stream window predicate types
export type { MakotoStreamWindowPredicate } from "./stream-window.js";
export * as streamWindow from "./stream-window.js";

// DBOM types
export type { DataBillOfMaterialsDBOM, MakotoLevel } from "./dbom.js";
export * as dbom from "./dbom.js";
`;

  await writeFile(join(TYPES_OUTPUT_DIR, "index.ts"), typesIndexContent);
  console.log(`  + types/index.ts (re-exports)`);

  // Generate schemas index file
  const schemasIndexContent = `/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * Re-exports all JSON schemas as TypeScript modules.
 */

export { default as originSchema } from "./origin-v1.js";
export { default as transformSchema } from "./transform-v1.js";
export { default as streamWindowSchema } from "./stream-window-v1.js";
export { default as dbomSchema } from "./dbom-v1.js";
`;

  await writeFile(join(SCHEMAS_OUTPUT_DIR, "index.ts"), schemasIndexContent);
  console.log(`  + schemas/index.ts (re-exports)\n`);

  console.log("Generation complete!");
}

generateTypes().catch((err) => {
  console.error("Failed to generate:", err);
  process.exit(1);
});
