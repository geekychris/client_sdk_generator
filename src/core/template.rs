// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use anyhow::{Context, Result};
use handlebars::{Handlebars, Helper, Context as HbsContext, RenderContext, Output, HelperResult};
use serde_json::Value;
use std::path::PathBuf;

use crate::core::config::{TargetLanguage, InputType};

pub struct TemplateEngine {
    handlebars: Handlebars<'static>,
    language: TargetLanguage,
    protocol: InputType,
}

impl TemplateEngine {
    pub fn new(language: TargetLanguage, protocol: InputType) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        
        // Disable strict mode for debugging
        handlebars.set_strict_mode(false);
        
        // Register custom helpers
        handlebars.register_helper("camel_case", Box::new(camel_case_helper));
        handlebars.register_helper("camelCase", Box::new(camel_case_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
        handlebars.register_helper("upper_case", Box::new(upper_case_helper));
        handlebars.register_helper("lower_case", Box::new(lower_case_helper));
        handlebars.register_helper("type_mapping", Box::new(type_mapping_helper));
        handlebars.register_helper("method_name", Box::new(method_name_helper));
        handlebars.register_helper("class_name_transform", Box::new(class_name_helper));
        handlebars.register_helper("class_name", Box::new(class_name_helper));
        handlebars.register_helper("else", Box::new(else_helper));
        handlebars.register_helper("go_name", Box::new(pascal_case_helper));
        handlebars.register_helper("package_path", Box::new(package_path_helper));
        handlebars.register_helper("import_path", Box::new(import_path_helper));
        handlebars.register_helper("contains_date_type", Box::new(contains_date_type_helper));
        handlebars.register_helper("contains_datetime_type", Box::new(contains_datetime_type_helper));
        handlebars.register_helper("eq", Box::new(eq_helper));
        handlebars.register_helper("ne", Box::new(ne_helper));
        handlebars.register_helper("or", Box::new(or_helper));
        handlebars.register_helper("PascalCase", Box::new(pascal_case_helper));
        handlebars.register_helper("json_schema", Box::new(json_schema_helper));
        handlebars.register_helper("grpc_type_mapping", Box::new(grpc_type_mapping_helper));
        handlebars.register_helper("go_type", Box::new(go_type_helper));
        handlebars.register_helper("ts_type", Box::new(ts_type_helper));
        handlebars.register_helper("default_value", Box::new(default_value_helper));
        
        Ok(Self { handlebars, language, protocol })
    }
    
    pub fn load_templates(&mut self, template_dir: PathBuf) -> Result<()> {
        let protocol_name = match self.protocol {
            InputType::OpenApi => "rest",
            InputType::GraphQL => "graphql",
            InputType::Grpc => "grpc",
        };
        let language_protocol_dir = template_dir
            .join(self.language.to_string().to_lowercase())
            .join(protocol_name);
        
        if !language_protocol_dir.exists() {
            return Err(anyhow::anyhow!("Template directory not found: {:?}", language_protocol_dir));
        }
        
        // Load all template files
        for entry in walkdir::WalkDir::new(&language_protocol_dir) {
            let entry = entry.context("Failed to read template directory")?;
            let path = entry.path();
            
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("hbs") {
                let template_name = path
                    .strip_prefix(&language_protocol_dir)
                    .context("Failed to get relative path")?
                    .with_extension("")
                    .to_string_lossy()
                    .replace(std::path::MAIN_SEPARATOR, "/");
                
                let template_content = std::fs::read_to_string(path)
                    .context(format!("Failed to read template file: {:?}", path))?;
                
                self.handlebars.register_template_string(&template_name, template_content)
                    .context(format!("Failed to register template: {}", template_name))?;
            }
        }
        
        Ok(())
    }
    
    pub fn render_template(&self, template_name: &str, context: &Value) -> Result<String> {
        self.handlebars
            .render(template_name, context)
            .context(format!("Failed to render template: {}", template_name))
    }
    
    pub fn list_templates(&self) -> Vec<String> {
        self.handlebars.get_templates().keys().cloned().collect()
    }
}

// Helper functions for template rendering
fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Parameter required for camel_case"))?;
    
    let result = to_camel_case(value);
    out.write(&result)?;
    Ok(())
}

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Parameter required for snake_case"))?;
    
    let result = to_snake_case(value);
    out.write(&result)?;
    Ok(())
}

fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Parameter required for pascal_case"))?;
    
    let result = to_pascal_case(value);
    out.write(&result)?;
    Ok(())
}

fn kebab_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Parameter required for kebab_case"))?;
    
    let result = to_kebab_case(value);
    out.write(&result)?;
    Ok(())
}

fn upper_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Parameter required for upper_case"))?;
    
    out.write(&value.to_uppercase())?;
    Ok(())
}

fn lower_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Parameter required for lower_case"))?;
    
    out.write(&value.to_lowercase())?;
    Ok(())
}

fn type_mapping_helper(
    h: &Helper,
    _: &Handlebars,
    ctx: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let _type_ref = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Type reference required for type_mapping"))?;
    
    let _language = ctx.data()["language"].as_str()
        .ok_or_else(|| handlebars::RenderError::new("Language context required"))?;
    
    // This would need to be implemented based on the actual TypeReference structure
    // For now, returning a placeholder
    let mapped_type = "String"; // Placeholder
    out.write(mapped_type)?;
    Ok(())
}

fn method_name_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let value = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Parameter required for method_name"))?;
    
    let result = to_camel_case(value);
    out.write(&result)?;
    Ok(())
}

fn class_name_helper(
    h: &Helper,
    _: &Handlebars,
    ctx: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    // First try to get from parameter, then fall back to context
    let value = if let Some(param) = h.param(0) {
        param.value().as_str()
            .ok_or_else(|| handlebars::RenderError::new("Invalid parameter for class_name"))?
    } else {
        // Check if class_name is in the context
        ctx.data().get("class_name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| handlebars::RenderError::new("class_name not found in context and no parameter provided"))?
    };
    
    let result = to_pascal_case(value);
    out.write(&result)?;
    Ok(())
}

fn package_path_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let package = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Package name required"))?;
    
    let path = package.replace('.', "/");
    out.write(&path)?;
    Ok(())
}

fn import_path_helper(
    h: &Helper,
    _: &Handlebars,
    ctx: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let class_name = h.param(0)
        .and_then(|v| v.value().as_str())
        .ok_or_else(|| handlebars::RenderError::new("Class name required"))?;
    
    let language = ctx.data()["language"].as_str()
        .ok_or_else(|| handlebars::RenderError::new("Language context required"))?;
    
    let import_path = match language {
        "java" => format!("import {}.{};", ctx.data()["package"].as_str().unwrap_or("com.example"), class_name),
        "python" => format!("from .{} import {}", to_snake_case(class_name), to_pascal_case(class_name)),
        "rust" => format!("use crate::{};", to_snake_case(class_name)),
        _ => class_name.to_string(),
    };
    
    out.write(&import_path)?;
    Ok(())
}

// String case conversion utilities
pub fn to_camel_case(s: &str) -> String {
    let parts: Vec<&str> = s.split_whitespace()
        .flat_map(|part| part.split('_'))
        .flat_map(|part| part.split('-'))
        .collect();
    
    if parts.is_empty() {
        return String::new();
    }
    
    let mut result = parts[0].to_lowercase();
    for part in parts.iter().skip(1) {
        if !part.is_empty() {
            result.push_str(&capitalize_first(part));
        }
    }
    result
}

pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_char_was_uppercase = false;
    
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_char_was_uppercase {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_char_was_uppercase = true;
        } else if c == ' ' || c == '-' {
            result.push('_');
            prev_char_was_uppercase = false;
        } else {
            result.push(c);
            prev_char_was_uppercase = false;
        }
    }
    result
}

pub fn to_pascal_case(s: &str) -> String {
    let parts: Vec<&str> = s.split_whitespace()
        .flat_map(|part| part.split('_'))
        .flat_map(|part| part.split('-'))
        .collect();
    
    parts.iter()
        .filter(|part| !part.is_empty())
        .map(|part| capitalize_first(part))
        .collect()
}

pub fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars.as_str().to_lowercase().chars()).collect(),
    }
}

// Additional helper functions
fn contains_date_type_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    // For now, always return false as a placeholder
    // This would need proper implementation based on the actual data structure
    out.write("false")?;
    Ok(())
}

fn contains_datetime_type_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    // For now, always return false as a placeholder
    out.write("false")?;
    Ok(())
}

fn eq_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let left = h.param(0).and_then(|v| v.value().as_str());
    let right = h.param(1).and_then(|v| v.value().as_str());
    
    let result = match (left, right) {
        (Some(l), Some(r)) => l == r,
        _ => false,
    };
    
    out.write(&result.to_string())?;
    Ok(())
}

fn ne_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let left = h.param(0).and_then(|v| v.value().as_str());
    let right = h.param(1).and_then(|v| v.value().as_str());
    
    let result = match (left, right) {
        (Some(l), Some(r)) => l != r,
        _ => true,
    };
    
    out.write(&result.to_string())?;
    Ok(())
}

fn or_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let left = h.param(0).and_then(|v| v.value().as_bool()).unwrap_or(false);
    let right = h.param(1).and_then(|v| v.value().as_bool()).unwrap_or(false);
    
    out.write(&(left || right).to_string())?;
    Ok(())
}

fn else_helper(
    _h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    _out: &mut dyn Output,
) -> HelperResult {
    // The else helper is primarily used for control flow in Handlebars templates
    // It doesn't need to output anything directly as it's handled by the template engine
    Ok(())
}

fn json_schema_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let _type_ref = h.param(0)
        .ok_or_else(|| handlebars::RenderError::new("Type reference required for json_schema"))?;
    
    // For now, return a placeholder JSON schema
    // This would need to be implemented based on the actual TypeReference structure
    let schema = "{ \"type\": \"object\", \"properties\": {} }";
    out.write(schema)?;
    Ok(())
}

fn grpc_type_mapping_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let type_name = if let Some(param) = h.param(0) {
        param.value().as_str().unwrap_or("string")
    } else {
        // Fallback to string if no parameter provided
        "string"
    };
    
    let ts_type = match type_name {
        "string" => "string",
        "int32" | "int64" | "uint32" | "uint64" | "sint32" | "sint64" => "number",
        "float" | "double" => "number",
        "bool" => "boolean",
        "bytes" => "Uint8Array",
        _ => type_name, // Return the type name as-is for custom types
    };
    
    out.write(ts_type)?;
    Ok(())
}

fn go_type_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let type_name = if let Some(param) = h.param(0) {
        param.value().as_str().unwrap_or("string")
    } else {
        "string" // Fallback to string if no parameter provided
    };
    
    let go_type = match type_name {
        "string" => "string",
        "int32" => "int32",
        "int64" => "int64",
        "uint32" => "uint32",
        "uint64" => "uint64",
        "float" => "float32",
        "double" => "float64",
        "bool" => "bool",
        "bytes" => "[]byte",
        _ => type_name, // Return the type name as-is for custom types
    };
    
    out.write(go_type)?;
    Ok(())
}

fn ts_type_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let type_name = if let Some(param) = h.param(0) {
        param.value().as_str().unwrap_or("string")
    } else {
        "string" // Fallback to string if no parameter provided
    };
    
    let ts_type = match type_name {
        "string" => "string",
        "integer" | "number" | "int32" | "int64" | "float" | "double" => "number",
        "boolean" | "bool" => "boolean",
        "array" => "any[]",
        "object" => "any",
        _ => type_name, // Return the type name as-is for custom types
    };
    
    out.write(ts_type)?;
    Ok(())
}

fn default_value_helper(
    h: &Helper,
    _: &Handlebars,
    _: &HbsContext,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let type_name = if let Some(param) = h.param(0) {
        param.value().as_str().unwrap_or("string")
    } else {
        "string" // Fallback to string if no parameter provided
    };
    
    let default_val = match type_name {
        "string" => "''",
        "number" | "int32" | "int64" | "float" | "double" => "0",
        "boolean" | "bool" => "false",
        "array" => "[]",
        "object" => "{}",
        _ => "undefined",
    };
    
    out.write(default_val)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_camel_case() {
        assert_eq!(to_camel_case("hello world"), "helloWorld");
        assert_eq!(to_camel_case("api_key"), "apiKey");
        assert_eq!(to_camel_case("user-profile"), "userProfile");
    }
    
    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
        assert_eq!(to_snake_case("apiKey"), "api_key");
        assert_eq!(to_snake_case("user-profile"), "user_profile");
    }
    
    #[test]
    fn test_pascal_case() {
        assert_eq!(to_pascal_case("hello world"), "HelloWorld");
        assert_eq!(to_pascal_case("api_key"), "ApiKey");
        assert_eq!(to_pascal_case("user-profile"), "UserProfile");
    }
    
    #[test]
    fn test_kebab_case() {
        assert_eq!(to_kebab_case("HelloWorld"), "hello-world");
        assert_eq!(to_kebab_case("apiKey"), "api-key");
        assert_eq!(to_kebab_case("user_profile"), "user-profile");
    }
}