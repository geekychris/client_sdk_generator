use crate::core::{
    config::{GeneratorConfig, AuthenticationType, AuthLocation},
    types::{ApiSpec, BaseType, TypeReference},
    template::TemplateEngine,
};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;
use std::{collections::HashMap, path::Path};

pub struct TypeScriptGenerator {
    config: GeneratorConfig,
    template_engine: TemplateEngine,
}

impl TypeScriptGenerator {
    pub fn new(config: GeneratorConfig) -> Result<Self> {
        let template_engine = TemplateEngine::new(config.target_language, config.input_type)?;
        Ok(Self { config, template_engine })
    }

    pub async fn generate(&self, api_spec: &ApiSpec, output_path: &Path) -> Result<()> {
        // Create output directory structure
        tokio::fs::create_dir_all(output_path).await?;
        let src_path = output_path.join("src");
        tokio::fs::create_dir_all(&src_path).await?;
        
        // Prepare template context
        let mut context = HashMap::new();
        context.insert("api_name", json!(api_spec.name));
        context.insert("api_version", json!(api_spec.version));
        context.insert("description", json!(api_spec.description));
        context.insert("base_url", json!(api_spec.base_url));
        context.insert("package_name", json!(self.get_package_name()));
        context.insert("client_class_name", json!(self.get_class_name(&api_spec.name)));
        
        // Add authentication context
        if let Some(auth_config) = &self.config.authentication {
            context.insert("authentication", json!(true));
            context.insert("auth_type", json!(format!("{:?}", auth_config.auth_type)));
            context.insert("auth_location", json!(format!("{:?}", auth_config.location)));
            context.insert("auth_parameter_name", json!(auth_config.parameter_name));
            context.insert("auth_scheme", json!(auth_config.scheme));
            context.insert("bearer_format", json!(auth_config.bearer_format));
            
            // Generate authentication helper methods
            context.insert("supports_bearer_auth", json!(matches!(auth_config.auth_type, AuthenticationType::Http | AuthenticationType::OAuth2)));
            context.insert("supports_api_key", json!(matches!(auth_config.auth_type, AuthenticationType::ApiKey)));
        } else {
            context.insert("authentication", json!(false));
        }

        // Add feature flags
        context.insert("async_support", json!(self.config.features.async_support));
        context.insert("retry", json!(self.config.features.retry.enabled));
        context.insert("telemetry", json!(self.config.features.telemetry.enabled));
        context.insert("caching", json!(self.config.features.caching.enabled));

        // Transform operations for TypeScript
        let operations: Vec<_> = api_spec.operations.iter()
            .map(|op| {
                json!({
                    "name": op.name,
                    "id": op.id,
                    "method": op.method.to_string().to_uppercase(),
                    "method_lower": op.method.to_string().to_lowercase(),
                    "path": op.path,
                    "description": op.description,
                    "parameters": op.parameters.iter().map(|p| {
                        json!({
                            "name": p.name,
                            "camel_name": to_camel_case(&p.name),
                            "ts_type": self.map_type_to_typescript(&p.data_type),
                            "location": format!("{:?}", p.parameter_type),
                            "required": p.required,
                            "description": p.description
                        })
                    }).collect::<Vec<_>>(),
                    "response_type": self.infer_response_type_typescript(op),
                    "has_path_params": op.parameters.iter().any(|p| matches!(p.parameter_type, crate::core::types::ParameterType::Path)),
                    "has_query_params": op.parameters.iter().any(|p| matches!(p.parameter_type, crate::core::types::ParameterType::Query)),
                    "has_body_params": op.request_body.is_some(),
                })
            })
            .collect();
        context.insert("operations", json!(operations));

        // Transform types for TypeScript
        let types: Vec<_> = api_spec.types.iter()
            .map(|type_def| {
                json!({
                    "name": type_def.name,
                    "description": type_def.description,
                    "properties": type_def.properties.iter().map(|prop| {
                        json!({
                            "name": prop.name,
                            "camel_name": to_camel_case(&prop.name),
                            "ts_type": self.map_type_to_typescript(&prop.data_type),
                            "required": prop.required,
                            "description": prop.description,
                            "optional_marker": if prop.required { "" } else { "?" }
                        })
                    }).collect::<Vec<_>>()
                })
            })
            .collect();
        context.insert("types", json!(types));

        // Generate files
        self.generate_client(&context, &src_path).await?;
        self.generate_types(&context, &src_path).await?;
        self.generate_index(&context, &src_path).await?;
        self.generate_package_json(&context, output_path).await?;
        self.generate_tsconfig(&context, output_path).await?;
        
        if self.config.output_config.include_docs {
            self.generate_readme(&context, output_path).await?;
        }

        Ok(())
    }

    async fn generate_client(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("client.ts", &context_value)?;
        let file_path = output_path.join("client.ts");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    async fn generate_types(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("types.ts", &context_value)?;
        let file_path = output_path.join("types.ts");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    async fn generate_index(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("index.ts", &context_value)?;
        let file_path = output_path.join("index.ts");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    async fn generate_package_json(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("package.json", &context_value)?;
        let file_path = output_path.join("package.json");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    async fn generate_tsconfig(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("tsconfig.json", &context_value)?;
        let file_path = output_path.join("tsconfig.json");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    async fn generate_readme(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("README.md", &context_value)?;
        let file_path = output_path.join("README.md");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    fn get_package_name(&self) -> String {
        self.config.output_config.package_name
            .as_ref()
            .map(|name| name.replace("_", "-").replace(" ", "-").to_lowercase())
            .unwrap_or_else(|| "api-client".to_string())
    }

    fn get_class_name(&self, api_name: &str) -> String {
        format!("{}Client", to_pascal_case(api_name))
    }

    fn map_type_to_typescript(&self, type_ref: &TypeReference) -> String {
        let base_type = match &type_ref.base_type {
            BaseType::String => "string",
            BaseType::Integer => "number",
            BaseType::Long => "number",
            BaseType::Float => "number",
            BaseType::Double => "number",
            BaseType::Boolean => "boolean",
            BaseType::Date => "Date",
            BaseType::DateTime => "Date",
            BaseType::Binary => "Blob | ArrayBuffer | Uint8Array",
            BaseType::Object(name) => name,
            BaseType::Any => "any",
            BaseType::Void => "void",
            BaseType::Array(inner) => return format!("{}[]", self.map_type_to_typescript(inner)),
            BaseType::Map(key, value) => return format!("Record<{}, {}>", self.map_type_to_typescript(key), self.map_type_to_typescript(value)),
        };

        let type_str = if type_ref.array {
            format!("{}[]", base_type)
        } else {
            base_type.to_string()
        };

        if type_ref.nullable {
            format!("{} | null", type_str)
        } else {
            type_str
        }
    }

    fn infer_response_type_typescript(&self, operation: &crate::core::types::Operation) -> String {
        // Look for successful response (200-299) 
        for response in &operation.responses {
            if let Some(status) = response.status_code {
                if status >= 200 && status < 300 {
                    if let Some(data_type) = &response.data_type {
                        return self.map_type_to_typescript(data_type);
                    }
                }
            }
        }
        
        // Default to any if no specific response type found
        "any".to_string()
    }
}

fn to_camel_case(name: &str) -> String {
    let words: Vec<&str> = name.split('_').collect();
    if words.is_empty() {
        return String::new();
    }
    
    let mut result = words[0].to_lowercase();
    for word in &words[1..] {
        result.push_str(&to_pascal_case(word));
    }
    result
}

fn to_pascal_case(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
            }
        })
        .collect()
}