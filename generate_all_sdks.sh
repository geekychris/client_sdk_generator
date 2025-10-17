#!/bin/bash

# Generate SDK for each language and protocol combination
echo "Generating SDKs for All Language-Protocol Combinations"
echo "======================================================"

# Languages available
LANGUAGES=("java" "python" "rust" "go" "typescript")

# Protocols available  
PROTOCOLS=("rest" "graphql" "grpc")

# Specification files
OPENAPI_SPEC="examples/petstore-openapi.yaml"
GRAPHQL_SCHEMA="examples/petstore-graphql.graphql"
GRPC_PROTO="examples/petstore.proto"  # Adjust path as needed

# Output base directory
OUTPUT_BASE_DIR="output"

SUCCESS_COUNT=0
TOTAL_COUNT=0

# Ensure output directory exists
mkdir -p "$OUTPUT_BASE_DIR"

# Function to generate SDK
generate_sdk() {
    local language=$1
    local protocol=$2
    local spec_file=$3
    local output_dir="$OUTPUT_BASE_DIR/${language}_${protocol}_sdk"
    
    echo "Generating: $language $protocol SDK"
    echo "Output directory: $output_dir"
    
    TOTAL_COUNT=$((TOTAL_COUNT + 1))
    
    # Clean output directory
    rm -rf "$output_dir"
    mkdir -p "$output_dir"
    
    # Build the command based on protocol
    case $protocol in
        "rest")
            if [ ! -f "$OPENAPI_SPEC" ]; then
                echo "⚠️  WARNING: OpenAPI spec file not found: $OPENAPI_SPEC"
                echo "❌ SKIPPED: $language $protocol SDK (missing spec file)"
                echo ""
                return
            fi
            cmd="./target/release/sdk-gen openapi --spec \"$spec_file\" --language \"$language\" --output \"$output_dir\""
            ;;
        "graphql")
            if [ ! -f "$GRAPHQL_SCHEMA" ]; then
                echo "⚠️  WARNING: GraphQL schema file not found: $GRAPHQL_SCHEMA"
                echo "❌ SKIPPED: $language $protocol SDK (missing schema file)"
                echo ""
                return
            fi
            cmd="./target/release/sdk-gen graphql --schema \"$spec_file\" --language \"$language\" --output \"$output_dir\""
            ;;
        "grpc")
            if [ ! -f "$GRPC_PROTO" ]; then
                echo "⚠️  WARNING: gRPC proto file not found: $GRPC_PROTO"
                echo "❌ SKIPPED: $language $protocol SDK (missing proto file)"
                echo ""
                return
            fi
            cmd="./target/release/sdk-gen grpc --proto \"$spec_file\" --language \"$language\" --output \"$output_dir\""
            ;;
        *)
            echo "❌ ERROR: Unknown protocol: $protocol"
            echo ""
            return
            ;;
    esac
    
    # Execute the command
    echo "Running: $cmd"
    eval $cmd
    
    if [ $? -eq 0 ]; then
        echo "✅ SUCCESS: $language $protocol SDK generated"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        
        # Verify files were created
        file_count=$(find "$output_dir" -type f 2>/dev/null | wc -l)
        echo "   Generated $file_count files"
        
        # List some key generated files
        if [ $file_count -gt 0 ]; then
            echo "   Sample files:"
            find "$output_dir" -name "*.java" -o -name "*.py" -o -name "*.rs" -o -name "*.go" -o -name "*.ts" -o -name "*.js" 2>/dev/null | head -3 | sed 's/^/     /'
        fi
    else
        echo "❌ FAILED: $language $protocol SDK generation failed"
    fi
    echo ""
}

# Build the project first
echo "Building the SDK generator..."
if [ -f "Cargo.toml" ]; then
    cargo build --release
    if [ $? -ne 0 ]; then
        echo "❌ ERROR: Failed to build the project"
        exit 1
    fi
    echo "✅ Build completed successfully"
    echo ""
else
    echo "⚠️  WARNING: Cargo.toml not found, assuming binary is already built"
    echo ""
fi

# Check if the binary exists
if [ ! -f "./target/release/sdk-gen" ]; then
    echo "❌ ERROR: SDK generator binary not found at ./target/release/sdk-gen"
    echo "Make sure to build the project first with: cargo build --release"
    exit 1
fi

# Generate all combinations
echo "Starting SDK generation for all combinations..."
echo ""

for language in "${LANGUAGES[@]}"; do
    for protocol in "${PROTOCOLS[@]}"; do
        case $protocol in
            "rest")
                spec_file="$OPENAPI_SPEC"
                ;;
            "graphql")
                spec_file="$GRAPHQL_SCHEMA"
                ;;
            "grpc")
                spec_file="$GRPC_PROTO"
                ;;
        esac
        
        generate_sdk "$language" "$protocol" "$spec_file"
    done
done

# Summary
echo "======================================================"
echo "GENERATION SUMMARY"
echo "======================================================"
echo "Successfully generated: $SUCCESS_COUNT/$TOTAL_COUNT SDKs"
echo ""

if [ $SUCCESS_COUNT -eq $TOTAL_COUNT ]; then
    echo "🎉 All SDK generations completed successfully!"
    echo ""
    echo "Generated SDKs are available in:"
    find "$OUTPUT_BASE_DIR" -maxdepth 1 -type d -name "*_*_sdk" | sed 's/^/  - /'
    exit 0
else
    echo "⚠️  Some SDK generations failed or were skipped"
    echo ""
    echo "Successfully generated SDKs:"
    find "$OUTPUT_BASE_DIR" -maxdepth 1 -type d -name "*_*_sdk" | sed 's/^/  - /'
    exit 1
fi