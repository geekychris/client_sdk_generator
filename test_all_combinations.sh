#!/bin/bash

# Test all language-protocol combinations
echo "Testing SDK Generation for All Language-Protocol Combinations"
echo "==========================================================="

# Languages to test
LANGUAGES=("java" "python" "rust" "go" "typescript")
# Protocols to test
PROTOCOLS=("openapi" "graphql")

# OpenAPI spec
OPENAPI_SPEC="examples/petstore-openapi.yaml"
# GraphQL schema  
GRAPHQL_SCHEMA="examples/petstore-graphql.graphql"

SUCCESS_COUNT=0
TOTAL_COUNT=0

# Function to test SDK generation
test_sdk() {
    local protocol=$1
    local language=$2
    local spec_file=$3
    local output_dir=$4
    
    echo "Testing: $language $protocol SDK"
    TOTAL_COUNT=$((TOTAL_COUNT + 1))
    
    # Clean output directory
    rm -rf "$output_dir"
    
    # Run SDK generation
    if [ "$protocol" = "openapi" ]; then
        ./target/release/sdk-gen openapi --spec "$spec_file" --language "$language" --output "$output_dir" 2>&1
    else
        ./target/release/sdk-gen graphql --schema "$spec_file" --language "$language" --output "$output_dir" 2>&1
    fi
    
    if [ $? -eq 0 ]; then
        echo "✅ SUCCESS: $language $protocol SDK generated"
        SUCCESS_COUNT=$((SUCCESS_COUNT + 1))
        
        # Verify files were created
        file_count=$(find "$output_dir" -type f | wc -l)
        echo "   Generated $file_count files"
        
        # List some key files
        echo "   Key files:"
        find "$output_dir" -name "*.java" -o -name "*.py" -o -name "*.rs" -o -name "*.go" -o -name "*.ts" | head -3 | sed 's/^/     /'
        
    else
        echo "❌ FAILED: $language $protocol SDK generation failed"
    fi
    echo ""
}

# Test all combinations
for language in "${LANGUAGES[@]}"; do
    for protocol in openapi graphql; do
        if [ "$protocol" = "openapi" ]; then
            spec_file="$OPENAPI_SPEC"
            output_dir="./generated-${language}-${protocol}-sdk"
        else
            spec_file="$GRAPHQL_SCHEMA"
            output_dir="./generated-${language}-${protocol}-sdk"
        fi
        
        test_sdk "$protocol" "$language" "$spec_file" "$output_dir"
    done
done

# Summary
echo "==========================================================="
echo "SUMMARY: $SUCCESS_COUNT/$TOTAL_COUNT tests passed"
echo "==========================================================="

if [ $SUCCESS_COUNT -eq $TOTAL_COUNT ]; then
    echo "🎉 All SDK generation tests PASSED!"
    exit 0
else
    echo "⚠️  Some SDK generation tests FAILED"
    exit 1
fi