# Templates

This directory contains Handlebars templates for generating client SDKs in different programming languages.

## Structure

- `java/` - Java SDK templates
- `python/` - Python SDK templates  
- `rust/` - Rust SDK templates

Each language directory contains templates for:

- `client.hbs` - Main client class
- `async_client.hbs` - Async client class (if supported)
- `config.hbs` - Configuration class
- `model.hbs` - Data model classes
- `tests.hbs` - Test files
- `pom.xml.hbs` / `setup.py.hbs` / `Cargo.toml.hbs` - Build configuration
- `README.md.hbs` - Documentation
- `api_docs.md.hbs` - API documentation

## Template Variables

The following variables are available in all templates:

- `spec` - The parsed API specification
- `config` - Generator configuration
- `language` - Target language name
- `package` - Package/module name
- `version` - SDK version
- `features` - Enabled features (retry, telemetry, caching, async)
- `feature_code` - Generated feature code (dependencies, imports, implementation)

## Helper Functions

Custom Handlebars helpers are available:

- `{{camel_case string}}` - Convert to camelCase
- `{{snake_case string}}` - Convert to snake_case
- `{{pascal_case string}}` - Convert to PascalCase
- `{{kebab_case string}}` - Convert to kebab-case
- `{{upper_case string}}` - Convert to UPPER_CASE
- `{{lower_case string}}` - Convert to lower_case
- `{{type_mapping type}}` - Map API type to language type
- `{{method_name name}}` - Format method name for language
- `{{class_name name}}` - Format class name for language
- `{{package_path package}}` - Convert package to file path
- `{{import_path class}}` - Generate import statement