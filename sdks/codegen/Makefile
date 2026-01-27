# Makoto SDK Code Generation
# Generate type-safe models from JSON Schemas

SCHEMA_DIR := ../../schemas/makoto.dev
COMMON_SCHEMA := $(SCHEMA_DIR)/common/definitions.json
ORIGIN_SCHEMA := $(SCHEMA_DIR)/origin/v1.json
TRANSFORM_SCHEMA := $(SCHEMA_DIR)/transform/v1.json
STREAM_WINDOW_SCHEMA := $(SCHEMA_DIR)/stream-window/v1.json
DBOM_SCHEMA := $(SCHEMA_DIR)/dbom/v1.json

# Output directories
PYTHON_OUT := ../python/src/makoto/models
TYPESCRIPT_OUT := ../typescript/src/models
GO_OUT := ../go/models
RUST_OUT := ../rust/src/models

.PHONY: all clean python typescript go rust check-tools

all: python typescript go rust

# ============================================================================
# Python (using datamodel-codegen)
# ============================================================================
python: $(PYTHON_OUT)/origin.py $(PYTHON_OUT)/transform.py \
        $(PYTHON_OUT)/stream_window.py $(PYTHON_OUT)/dbom.py \
        $(PYTHON_OUT)/common.py

$(PYTHON_OUT):
	mkdir -p $(PYTHON_OUT)

$(PYTHON_OUT)/common.py: $(COMMON_SCHEMA) | $(PYTHON_OUT)
	datamodel-codegen \
		--input $< \
		--input-file-type jsonschema \
		--output $@ \
		--output-model-type pydantic_v2.BaseModel \
		--use-annotated \
		--field-constraints \
		--use-double-quotes \
		--target-python-version 3.11 \
		--class-name CommonDefinitions

$(PYTHON_OUT)/origin.py: $(ORIGIN_SCHEMA) | $(PYTHON_OUT)
	datamodel-codegen \
		--input $< \
		--input-file-type jsonschema \
		--output $@ \
		--output-model-type pydantic_v2.BaseModel \
		--use-annotated \
		--field-constraints \
		--use-double-quotes \
		--target-python-version 3.11

$(PYTHON_OUT)/transform.py: $(TRANSFORM_SCHEMA) | $(PYTHON_OUT)
	datamodel-codegen \
		--input $< \
		--input-file-type jsonschema \
		--output $@ \
		--output-model-type pydantic_v2.BaseModel \
		--use-annotated \
		--field-constraints \
		--use-double-quotes \
		--target-python-version 3.11

$(PYTHON_OUT)/stream_window.py: $(STREAM_WINDOW_SCHEMA) | $(PYTHON_OUT)
	datamodel-codegen \
		--input $< \
		--input-file-type jsonschema \
		--output $@ \
		--output-model-type pydantic_v2.BaseModel \
		--use-annotated \
		--field-constraints \
		--use-double-quotes \
		--target-python-version 3.11

$(PYTHON_OUT)/dbom.py: $(DBOM_SCHEMA) | $(PYTHON_OUT)
	datamodel-codegen \
		--input $< \
		--input-file-type jsonschema \
		--output $@ \
		--output-model-type pydantic_v2.BaseModel \
		--use-annotated \
		--field-constraints \
		--use-double-quotes \
		--target-python-version 3.11

# ============================================================================
# TypeScript (using json-schema-to-typescript)
# ============================================================================
typescript: $(TYPESCRIPT_OUT)/origin.ts $(TYPESCRIPT_OUT)/transform.ts \
            $(TYPESCRIPT_OUT)/stream-window.ts $(TYPESCRIPT_OUT)/dbom.ts \
            $(TYPESCRIPT_OUT)/common.ts

$(TYPESCRIPT_OUT):
	mkdir -p $(TYPESCRIPT_OUT)

$(TYPESCRIPT_OUT)/common.ts: $(COMMON_SCHEMA) | $(TYPESCRIPT_OUT)
	json2ts --input $< --output $@ --bannerComment ""

$(TYPESCRIPT_OUT)/origin.ts: $(ORIGIN_SCHEMA) | $(TYPESCRIPT_OUT)
	json2ts --input $< --output $@ --bannerComment ""

$(TYPESCRIPT_OUT)/transform.ts: $(TRANSFORM_SCHEMA) | $(TYPESCRIPT_OUT)
	json2ts --input $< --output $@ --bannerComment ""

$(TYPESCRIPT_OUT)/stream-window.ts: $(STREAM_WINDOW_SCHEMA) | $(TYPESCRIPT_OUT)
	json2ts --input $< --output $@ --bannerComment ""

$(TYPESCRIPT_OUT)/dbom.ts: $(DBOM_SCHEMA) | $(TYPESCRIPT_OUT)
	json2ts --input $< --output $@ --bannerComment ""

# ============================================================================
# Go (using go-jsonschema)
# ============================================================================
go: $(GO_OUT)/origin.go $(GO_OUT)/transform.go \
    $(GO_OUT)/stream_window.go $(GO_OUT)/dbom.go \
    $(GO_OUT)/common.go

$(GO_OUT):
	mkdir -p $(GO_OUT)

$(GO_OUT)/common.go: $(COMMON_SCHEMA) | $(GO_OUT)
	gojsonschema --package models --output $@ $<

$(GO_OUT)/origin.go: $(ORIGIN_SCHEMA) | $(GO_OUT)
	gojsonschema --package models --output $@ $<

$(GO_OUT)/transform.go: $(TRANSFORM_SCHEMA) | $(GO_OUT)
	gojsonschema --package models --output $@ $<

$(GO_OUT)/stream_window.go: $(STREAM_WINDOW_SCHEMA) | $(GO_OUT)
	gojsonschema --package models --output $@ $<

$(GO_OUT)/dbom.go: $(DBOM_SCHEMA) | $(GO_OUT)
	gojsonschema --package models --output $@ $<

# ============================================================================
# Rust (using typify)
# ============================================================================
rust: $(RUST_OUT)/origin.rs $(RUST_OUT)/transform.rs \
      $(RUST_OUT)/stream_window.rs $(RUST_OUT)/dbom.rs \
      $(RUST_OUT)/common.rs

$(RUST_OUT):
	mkdir -p $(RUST_OUT)

$(RUST_OUT)/common.rs: $(COMMON_SCHEMA) | $(RUST_OUT)
	typify $< --output $@

$(RUST_OUT)/origin.rs: $(ORIGIN_SCHEMA) | $(RUST_OUT)
	typify $< --output $@

$(RUST_OUT)/transform.rs: $(TRANSFORM_SCHEMA) | $(RUST_OUT)
	typify $< --output $@

$(RUST_OUT)/stream_window.rs: $(STREAM_WINDOW_SCHEMA) | $(RUST_OUT)
	typify $< --output $@

$(RUST_OUT)/dbom.rs: $(DBOM_SCHEMA) | $(RUST_OUT)
	typify $< --output $@

# ============================================================================
# Utilities
# ============================================================================
clean:
	rm -rf $(PYTHON_OUT)/*.py
	rm -rf $(TYPESCRIPT_OUT)/*.ts
	rm -rf $(GO_OUT)/*.go
	rm -rf $(RUST_OUT)/*.rs

check-tools:
	@echo "Checking for required tools..."
	@which datamodel-codegen > /dev/null || echo "Missing: datamodel-codegen (pip install datamodel-code-generator)"
	@which json2ts > /dev/null || echo "Missing: json2ts (npm install -g json-schema-to-typescript)"
	@which gojsonschema > /dev/null || echo "Missing: gojsonschema (go install github.com/atombender/go-jsonschema/cmd/gojsonschema@latest)"
	@which typify > /dev/null || echo "Missing: typify (cargo install typify-cli)"
	@echo "Tool check complete"

# Validate generated code against example attestations
validate: validate-python validate-typescript validate-go validate-rust

validate-python:
	@echo "Validating Python models..."
	cd ../python && python -c "from makoto.models import OriginPredicate; print('Python OK')"

validate-typescript:
	@echo "Validating TypeScript models..."
	cd ../typescript && npx tsc --noEmit && echo "TypeScript OK"

validate-go:
	@echo "Validating Go models..."
	cd ../go && go build ./... && echo "Go OK"

validate-rust:
	@echo "Validating Rust models..."
	cd ../rust && cargo check && echo "Rust OK"
