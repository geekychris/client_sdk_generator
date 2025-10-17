use crate::core::{
    config::{GeneratorConfig, AuthenticationType, AuthLocation},
    types::{ApiSpec, BaseType, TypeReference},
    template::TemplateEngine,
};
use anyhow::Result;
use handlebars::Handlebars;
use serde_json::json;
use std::{collections::HashMap, path::Path};

pub struct GoGenerator {
    config: GeneratorConfig,
    template_engine: TemplateEngine,
}

impl GoGenerator {
    pub fn new(config: GeneratorConfig) -> Result<Self> {
        let template_engine = TemplateEngine::new(config.target_language, config.input_type)?;
        Ok(Self { config, template_engine })
    }

    pub async fn generate(&self, api_spec: &ApiSpec, output_path: &Path) -> Result<()> {
        // Create output directory structure
        tokio::fs::create_dir_all(output_path).await?;
        
        // Prepare template context
        let mut context = HashMap::new();
        context.insert("api_name", json!(api_spec.name));
        context.insert("api_version", json!(api_spec.version));
        context.insert("description", json!(api_spec.description));
        context.insert("base_url", json!(api_spec.base_url));
        context.insert("package_name", json!(self.get_package_name()));
        context.insert("go_module", json!(self.get_go_module()));
        context.insert("class_name", json!(to_go_name(&format!("{}Client", api_spec.name))));
        
        // Add authentication context
        if let Some(auth_config) = &self.config.authentication {
            context.insert("authentication", json!(true));
            context.insert("auth_type", json!(format!("{:?}", auth_config.auth_type)));
            context.insert("auth_location", json!(format!("{:?}", auth_config.location)));
            context.insert("auth_parameter_name", json!(auth_config.parameter_name));
            context.insert("auth_scheme", json!(auth_config.scheme));
            context.insert("bearer_format", json!(auth_config.bearer_format));
        } else {
            context.insert("authentication", json!(false));
        }

        // Add feature flags
        context.insert("async_support", json!(self.config.features.async_support));
        context.insert("retry", json!(self.config.features.retry.enabled));
        context.insert("telemetry", json!(self.config.features.telemetry.enabled));
        context.insert("caching", json!(self.config.features.caching.enabled));

        // Transform operations for Go
        let operations: Vec<_> = api_spec.operations.iter()
            .map(|op| {
                json!({
                    "name": op.name,
                    "id": op.id,
                    "method": format!("{:?}", op.method),
                    "path": op.path,
                    "description": op.description,
                    "parameters": op.parameters.iter().map(|p| {
                        json!({
                            "name": p.name,
                            "go_name": to_go_name(&p.name),
                            "go_type": self.map_type_to_go(&p.data_type),
                            "location": format!("{:?}", p.parameter_type),
                            "required": p.required,
                            "description": p.description
                        })
                    }).collect::<Vec<_>>(),
                    "response_type": self.infer_response_type_go(op),
                    "has_path_params": op.parameters.iter().any(|p| matches!(p.parameter_type, crate::core::types::ParameterType::Path)),
                    "has_query_params": op.parameters.iter().any(|p| matches!(p.parameter_type, crate::core::types::ParameterType::Query)),
                })
            })
            .collect();
        context.insert("operations", json!(operations));

        // Transform types for Go
        let types: Vec<_> = api_spec.types.iter()
            .map(|type_def| {
                json!({
                    "name": type_def.name,
                    "go_name": to_go_name(&type_def.name),
                    "description": type_def.description,
                    "properties": type_def.properties.iter().map(|prop| {
                        json!({
                            "name": prop.name,
                            "go_name": to_go_name(&prop.name),
                            "go_type": self.map_type_to_go(&prop.data_type),
                            "json_name": prop.name,
                            "required": prop.required,
                            "description": prop.description
                        })
                    }).collect::<Vec<_>>()
                })
            })
            .collect();
        context.insert("types", json!(types));

        // Generate files
        self.generate_client(&context, output_path).await?;
        self.generate_types(&context, output_path).await?;
        self.generate_go_mod(&context, output_path).await?;
        
        if self.config.output_config.include_docs {
            self.generate_readme(&context, output_path).await?;
        }

        Ok(())
    }

    async fn generate_client(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("client.go", &context_value)?;
        let file_path = output_path.join("client.go");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    async fn generate_types(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("types.go", &context_value)?;
        let file_path = output_path.join("types.go");
        tokio::fs::write(file_path, rendered).await?;
        Ok(())
    }

    async fn generate_go_mod(&self, context: &HashMap<&str, serde_json::Value>, output_path: &Path) -> Result<()> {
        let context_value = serde_json::to_value(context)?;
        let rendered = self.template_engine.render_template("go.mod", &context_value)?;
        let file_path = output_path.join("go.mod");
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
            .map(|name| name.replace("-", "_").replace(" ", "_").to_lowercase())
            .unwrap_or_else(|| "client".to_string())
    }

    fn get_go_module(&self) -> String {
        format!("github.com/example/{}", self.get_package_name())
    }

    fn map_type_to_go(&self, type_ref: &TypeReference) -> String {
        let base_type = match &type_ref.base_type {
            BaseType::String => "string",
            BaseType::Integer => "int64",
            BaseType::Long => "int64",
            BaseType::Float => "float32",
            BaseType::Double => "float64",
            BaseType::Boolean => "bool",
            BaseType::Date => "time.Time",
            BaseType::DateTime => "time.Time",
            BaseType::Binary => "[]byte",
            BaseType::Object(name) => name,
            BaseType::Any => "interface{}",
            BaseType::Void => "",
            BaseType::Array(inner) => return format!("[]{}", self.map_type_to_go(inner)),
            BaseType::Map(key, value) => return format!("map[{}]{}", self.map_type_to_go(key), self.map_type_to_go(value)),
        };

        let type_str = if type_ref.array {
            format!("[]{}", base_type)
        } else {
            base_type.to_string()
        };

        if type_ref.nullable && !type_ref.array {
            format!("*{}", type_str)
        } else {
            type_str
        }
    }

    fn infer_response_type_go(&self, operation: &crate::core::types::Operation) -> String {
        // Look for successful response (200-299) 
        for response in &operation.responses {
            if let Some(status) = response.status_code {
                if status >= 200 && status < 300 {
                    if let Some(data_type) = &response.data_type {
                        return self.map_type_to_go(data_type);
                    }
                }
            }
        }
        
        // Default to interface{} if no specific response type found
        "interface{}".to_string()
    }
}

fn to_go_name(name: &str) -> String {
    // Convert to PascalCase for Go
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