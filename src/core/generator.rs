// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use anyhow::{Context, Result};
use std::path::PathBuf;
use serde_json::Value;
use tracing::{info, debug, warn};

use crate::core::config::{GeneratorConfig, TargetLanguage};
use crate::core::types::ApiSpec;
use crate::core::template::TemplateEngine;
use crate::core::features::{FeatureGenerator, FeatureCode};
use crate::core::test_generation::{TestGenerationConfig, TestGenerator};
use crate::generators::TestGeneratorFactory;
use crate::parsers::{OpenApiParser, GraphQLParser, GrpcParser};

pub struct SdkGenerator {
    config: GeneratorConfig,
    template_engine: TemplateEngine,
    feature_generator: FeatureGenerator,
    test_generator: Option<Box<dyn TestGenerator>>,
}

impl SdkGenerator {
    pub fn new(config: GeneratorConfig) -> Result<Self> {
        let template_engine = TemplateEngine::new(config.target_language, config.input_type)?;
        let feature_generator = FeatureGenerator::new(config.target_language);
        let test_generator = if config.output_config.include_tests {
            Some(TestGeneratorFactory::create_generator(config.target_language)?)
        } else {
            None
        };
        
        Ok(Self {
            config,
            template_engine,
            feature_generator,
            test_generator,
        })
    }
    
    pub async fn generate_from_openapi(
        mut self,
        spec_path: PathBuf,
        output_path: PathBuf,
    ) -> Result<()> {
        info!("Parsing OpenAPI specification from {:?}", spec_path);
        
        let mut parser = OpenApiParser::new();
        let api_spec = parser.parse(spec_path).await
            .context("Failed to parse OpenAPI specification")?;
        
        self.generate_sdk(api_spec, output_path).await
    }
    
    pub async fn generate_from_graphql(
        mut self,
        schema_path: PathBuf,
        output_path: PathBuf,
    ) -> Result<()> {
        info!("Parsing GraphQL schema from {:?}", schema_path);
        
        let parser = GraphQLParser::new();
        let api_spec = parser.parse(schema_path).await
            .context("Failed to parse GraphQL schema")?;
        
        self.generate_sdk(api_spec, output_path).await
    }
    
    pub async fn generate_from_grpc(
        mut self,
        proto_path: PathBuf,
        output_path: PathBuf,
    ) -> Result<()> {
        info!("Parsing gRPC proto files from {:?}", proto_path);
        
        let parser = GrpcParser::new();
        let api_spec = parser.parse(proto_path).await
            .context("Failed to parse gRPC proto files")?;
        
        self.generate_sdk(api_spec, output_path).await
    }
    
    async fn generate_sdk(&mut self, api_spec: ApiSpec, output_path: PathBuf) -> Result<()> {
        info!("Generating {} SDK to {:?}", self.config.target_language, output_path);
        
        // Load templates
        let template_dir = self.get_template_directory()?;
        self.template_engine.load_templates(template_dir)
            .context("Failed to load templates")?;
        
        // Generate feature code
        let feature_codes = self.generate_features()?;
        
        // Create output directory
        std::fs::create_dir_all(&output_path)
            .context("Failed to create output directory")?;
        
        // Generate SDK files
        self.generate_client_files(&api_spec, &output_path, &feature_codes).await?;
        self.generate_model_files(&api_spec, &output_path).await?;
        self.generate_build_files(&output_path, &feature_codes).await?;
        
        if self.config.output_config.include_tests {
            self.generate_test_files(&api_spec, &output_path).await?;
        }
        
        if self.config.output_config.include_docs {
            self.generate_documentation(&api_spec, &output_path).await?;
        }
        
        if self.config.output_config.format_code {
            self.format_generated_code(&output_path).await?;
        }
        
        info!("SDK generation completed successfully");
        Ok(())
    }
    
    fn generate_features(&self) -> Result<Vec<FeatureCode>> {
        let mut feature_codes = Vec::new();
        
        if self.config.features.retry.enabled {
            debug!("Generating retry feature code");
            let retry_code = self.feature_generator.generate_retry_code(&self.config.features.retry);
            feature_codes.push(retry_code);
        }
        
        if self.config.features.telemetry.enabled {
            debug!("Generating telemetry feature code");
            let telemetry_code = self.feature_generator.generate_telemetry_code(&self.config.features.telemetry);
            feature_codes.push(telemetry_code);
        }
        
        if self.config.features.caching.enabled {
            debug!("Generating caching feature code");
            let caching_code = self.feature_generator.generate_caching_code(&self.config.features.caching);
            feature_codes.push(caching_code);
        }
        
        Ok(feature_codes)
    }
    
    async fn generate_client_files(
        &self,
        api_spec: &ApiSpec,
        output_path: &PathBuf,
        feature_codes: &[FeatureCode],
    ) -> Result<()> {
        debug!("Generating client files");
        
        let context = self.create_template_context(api_spec, feature_codes)?;
        
        // Generate main client class
        let client_content = self.template_engine.render_template("client", &context)
            .context("Failed to render client template")?;
        
        let client_path = self.get_client_file_path(output_path);
        std::fs::create_dir_all(client_path.parent().unwrap())?;
        std::fs::write(&client_path, client_content)
            .context("Failed to write client file")?;
        
        // Generate async client if enabled (except for Python which includes async in main template)
        if self.config.features.async_support && self.config.target_language != crate::core::config::TargetLanguage::Python {
            let async_client_content = self.template_engine.render_template("async_client", &context)
                .context("Failed to render async client template")?;
            
            let async_client_path = self.get_async_client_file_path(output_path);
            std::fs::write(&async_client_path, async_client_content)
                .context("Failed to write async client file")?;
        }
        
        // Generate configuration class
        let config_content = self.template_engine.render_template("config", &context)
            .context("Failed to render config template")?;
        
        let config_path = self.get_config_file_path(output_path);
        std::fs::write(&config_path, config_content)
            .context("Failed to write config file")?;
        
        Ok(())
    }
    
    async fn generate_model_files(&self, api_spec: &ApiSpec, output_path: &PathBuf) -> Result<()> {
        debug!("Generating model files");
        
        let models_dir = self.get_models_directory(output_path);
        std::fs::create_dir_all(&models_dir)?;
        
        for type_def in &api_spec.types {
            let context = self.create_model_context(type_def)?;
            let model_content = self.template_engine.render_template("model", &context)
                .context("Failed to render model template")?;
            
            let model_path = self.get_model_file_path(&models_dir, &type_def.name);
            std::fs::write(&model_path, model_content)
                .context(format!("Failed to write model file for {}", type_def.name))?;
        }
        
        Ok(())
    }
    
    async fn generate_build_files(
        &self,
        output_path: &PathBuf,
        feature_codes: &[FeatureCode],
    ) -> Result<()> {
        debug!("Generating build files");
        
        let merged_features = FeatureCode::merge(feature_codes.to_vec());
        let context = self.create_build_context(&merged_features)?;
        
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                let pom_content = self.template_engine.render_template("pom.xml", &context)
                    .context("Failed to render pom.xml template")?;
                std::fs::write(output_path.join("pom.xml"), pom_content)?;
            }
            crate::core::config::TargetLanguage::Python => {
                let setup_content = self.template_engine.render_template("setup.py", &context)
                    .context("Failed to render setup.py template")?;
                std::fs::write(output_path.join("setup.py"), setup_content)?;
                
                let requirements_content = merged_features.dependencies.join("\n");
                std::fs::write(output_path.join("requirements.txt"), requirements_content)?;
            }
            crate::core::config::TargetLanguage::Rust => {
                let cargo_content = self.template_engine.render_template("Cargo.toml", &context)
                    .context("Failed to render Cargo.toml template")?;
                std::fs::write(output_path.join("Cargo.toml"), cargo_content)?;
            }
            crate::core::config::TargetLanguage::Go => {
                let go_mod_content = self.template_engine.render_template("go.mod", &context)
                    .context("Failed to render go.mod template")?;
                std::fs::write(output_path.join("go.mod"), go_mod_content)?;
            }
            crate::core::config::TargetLanguage::TypeScript => {
                let package_content = self.template_engine.render_template("package.json", &context)
                    .context("Failed to render package.json template")?;
                std::fs::write(output_path.join("package.json"), package_content)?;
                
                let tsconfig_content = self.template_engine.render_template("tsconfig.json", &context)
                    .context("Failed to render tsconfig.json template")?;
                std::fs::write(output_path.join("tsconfig.json"), tsconfig_content)?;
            }
        }
        
        Ok(())
    }
    
    async fn generate_test_files(&self, api_spec: &ApiSpec, output_path: &PathBuf) -> Result<()> {
        debug!("Generating comprehensive test files");
        
        if let Some(test_generator) = &self.test_generator {
            let test_config = TestGenerationConfig::default();
            let test_suite = test_generator.generate_test_suite(api_spec, &test_config, output_path)?;
            
            info!("Generated {} test files, {} mock data files, {} config files", 
                  test_suite.files.len(), 
                  test_suite.mock_data_files.len(), 
                  test_suite.config_files.len());
            
            // Write test files
            for test_file in &test_suite.files {
                let full_path = output_path.join(&test_file.path);
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)
                        .context(format!("Failed to create directory for {}", test_file.name))?;
                }
                std::fs::write(&full_path, &test_file.content)
                    .context(format!("Failed to write test file {}", test_file.name))?;
                debug!("Generated test file: {:?}", full_path);
            }
            
            // Write mock data files
            for mock_file in &test_suite.mock_data_files {
                let full_path = output_path.join(&mock_file.path);
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)
                        .context(format!("Failed to create directory for {}", mock_file.name))?;
                }
                std::fs::write(&full_path, &mock_file.content)
                    .context(format!("Failed to write mock data file {}", mock_file.name))?;
                debug!("Generated mock data file: {:?}", full_path);
            }
            
            // Write test config files
            for config_file in &test_suite.config_files {
                let full_path = output_path.join(&config_file.path);
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)
                        .context(format!("Failed to create directory for {}", config_file.name))?;
                }
                std::fs::write(&full_path, &config_file.content)
                    .context(format!("Failed to write test config file {}", config_file.name))?;
                debug!("Generated test config file: {:?}", full_path);
            }
        } else {
            warn!("No test generator available for target language");
        }
        
        Ok(())
    }
    
    async fn generate_documentation(&self, api_spec: &ApiSpec, output_path: &PathBuf) -> Result<()> {
        debug!("Generating documentation");
        
        let docs_dir = output_path.join("docs");
        std::fs::create_dir_all(&docs_dir)?;
        
        let context = self.create_template_context(api_spec, &[])?;
        
        // Generate README
        let readme_content = self.template_engine.render_template("README.md", &context)
            .context("Failed to render README template")?;
        std::fs::write(docs_dir.join("README.md"), readme_content)?;
        
        // Generate API documentation
        let api_docs_content = self.template_engine.render_template("api_docs.md", &context)
            .context("Failed to render API docs template")?;
        std::fs::write(docs_dir.join("api.md"), api_docs_content)?;
        
        Ok(())
    }
    
    async fn format_generated_code(&self, output_path: &PathBuf) -> Result<()> {
        debug!("Formatting generated code");
        
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                // Try to format with Google Java Format or similar
                if let Err(e) = tokio::process::Command::new("google-java-format")
                    .args(&["--replace", "--recursive"])
                    .arg(output_path)
                    .output()
                    .await
                {
                    warn!("Failed to format Java code: {}", e);
                }
            }
            crate::core::config::TargetLanguage::Python => {
                // Try to format with black
                if let Err(e) = tokio::process::Command::new("black")
                    .arg(output_path)
                    .output()
                    .await
                {
                    warn!("Failed to format Python code: {}", e);
                }
            }
            crate::core::config::TargetLanguage::Rust => {
                // Try to format with rustfmt
                if let Err(e) = tokio::process::Command::new("cargo")
                    .args(&["fmt", "--all"])
                    .current_dir(output_path)
                    .output()
                    .await
                {
                    warn!("Failed to format Rust code: {}", e);
                }
            }
            crate::core::config::TargetLanguage::Go => {
                // Try to format with gofmt
                if let Err(e) = tokio::process::Command::new("gofmt")
                    .args(&["-w", "."])
                    .current_dir(output_path)
                    .output()
                    .await
                {
                    warn!("Failed to format Go code: {}", e);
                }
            }
            crate::core::config::TargetLanguage::TypeScript => {
                // Try to format with prettier
                if let Err(e) = tokio::process::Command::new("prettier")
                    .args(&["--write", "**/*.ts"])
                    .current_dir(output_path)
                    .output()
                    .await
                {
                    warn!("Failed to format TypeScript code: {}", e);
                }
            }
        }
        
        Ok(())
    }
    
    // Helper methods for file paths and contexts
    
    fn get_template_directory(&self) -> Result<PathBuf> {
        if let Some(override_path) = &self.config.template_overrides {
            Ok(override_path.clone())
        } else {
            // Try to find templates directory relative to the project root
            let current_dir = std::env::current_dir()
                .context("Failed to get current directory")?;
            
            // Look for templates directory in current directory
            let templates_dir = current_dir.join("templates");
            if templates_dir.exists() {
                return Ok(templates_dir);
            }
            
            // Fallback to executable directory (for installed binary)
            let exe_path = std::env::current_exe()
                .context("Failed to get current executable path")?;
            let exe_dir = exe_path.parent()
                .context("Failed to get executable directory")?;
            Ok(exe_dir.join("templates"))
        }
    }
    
    fn create_template_context(&self, api_spec: &ApiSpec, feature_codes: &[FeatureCode]) -> Result<Value> {
        let merged_features = FeatureCode::merge(feature_codes.to_vec());
        
        // Create class name from API spec name
        let class_name = format!("{}Client", api_spec.name);
        
        let context = serde_json::json!({
            "spec": api_spec,
            "config": self.config,
            "language": self.config.target_language.to_string().to_lowercase(),
            "package": self.config.output_config.package_name.as_ref().unwrap_or(&"com.example.client".to_string()),
            "package_name": self.config.output_config.package_name.as_ref().unwrap_or(&"client".to_string()),
            "version": self.config.output_config.version,
            "api_name": api_spec.name,
            "api_version": api_spec.version,
            "description": api_spec.description,
            "base_url": api_spec.base_url,
            "class_name": class_name,
            "go_module": format!("github.com/example/{}", self.config.output_config.package_name.as_ref().unwrap_or(&"client".to_string()).replace("-", "_").replace(" ", "_").to_lowercase()),
            "operations": api_spec.operations,
            "types": api_spec.types,
            "features": {
                "retry": self.config.features.retry.enabled,
                "telemetry": self.config.features.telemetry.enabled,
                "caching": self.config.features.caching.enabled,
                "async": self.config.features.async_support,
            },
            "feature_code": {
                "dependencies": merged_features.dependencies,
                "imports": merged_features.imports,
                "code": merged_features.code,
            }
        });
        
        Ok(context)
    }
    
    fn create_model_context(&self, type_def: &crate::core::types::TypeDefinition) -> Result<Value> {
        Ok(serde_json::json!({
            "type": type_def,
            "config": self.config,
            "language": self.config.target_language.to_string().to_lowercase(),
            "package": self.config.output_config.package_name.as_ref().unwrap_or(&"com.example.client".to_string()),
        }))
    }
    
    fn create_build_context(&self, features: &FeatureCode) -> Result<Value> {
        Ok(serde_json::json!({
            "config": self.config,
            "dependencies": features.dependencies,
            "package": self.config.output_config.package_name.as_ref().unwrap_or(&"client-sdk".to_string()),
            "version": self.config.output_config.version,
            "author": self.config.output_config.author,
            "license": self.config.output_config.license,
        }))
    }
    
    fn get_client_file_path(&self, output_path: &PathBuf) -> PathBuf {
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                let package_path = self.config.output_config.package_name
                    .as_ref()
                    .unwrap_or(&"com.example.client".to_string())
                    .replace('.', "/");
                output_path.join("src").join("main").join("java").join(package_path).join("Client.java")
            }
            crate::core::config::TargetLanguage::Python => {
                output_path.join("client_sdk").join("client.py")
            }
            crate::core::config::TargetLanguage::Rust => {
                output_path.join("src").join("client.rs")
            },
            TargetLanguage::Go => output_path.join("client_sdk").join("client.go"),
            TargetLanguage::TypeScript => output_path.join("client_sdk").join("client.ts"),
        }
    }
    
    fn get_async_client_file_path(&self, output_path: &PathBuf) -> PathBuf {
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                let package_path = self.config.output_config.package_name
                    .as_ref()
                    .unwrap_or(&"com.example.client".to_string())
                    .replace('.', "/");
                output_path.join("src").join("main").join("java").join(package_path).join("AsyncClient.java")
            }
            crate::core::config::TargetLanguage::Python => {
                output_path.join("client_sdk").join("async_client.py")
            }
            crate::core::config::TargetLanguage::Rust => {
                output_path.join("src").join("async_client.rs")
            },
            TargetLanguage::Go => output_path.join("client_sdk").join("async_client.go"),
            TargetLanguage::TypeScript => output_path.join("client_sdk").join("async_client.ts"),
        }
    }
    
    fn get_config_file_path(&self, output_path: &PathBuf) -> PathBuf {
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                let package_path = self.config.output_config.package_name
                    .as_ref()
                    .unwrap_or(&"com.example.client".to_string())
                    .replace('.', "/");
                output_path.join("src").join("main").join("java").join(package_path).join("ClientConfig.java")
            }
            crate::core::config::TargetLanguage::Python => {
                output_path.join("client_sdk").join("config.py")
            }
            crate::core::config::TargetLanguage::Rust => {
                output_path.join("src").join("config.rs")
            },
            TargetLanguage::Go => output_path.join("client_sdk").join("config.go"),
            TargetLanguage::TypeScript => output_path.join("client_sdk").join("config.ts"),
        }
    }
    
    fn get_models_directory(&self, output_path: &PathBuf) -> PathBuf {
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                let package_path = self.config.output_config.package_name
                    .as_ref()
                    .unwrap_or(&"com.example.client".to_string())
                    .replace('.', "/");
                output_path.join("src").join("main").join("java").join(package_path).join("models")
            }
            crate::core::config::TargetLanguage::Python => {
                output_path.join("client_sdk").join("models")
            }
            crate::core::config::TargetLanguage::Rust => {
                output_path.join("src").join("models")
            },
            TargetLanguage::Go => output_path.join("client_sdk").join("models"),
            TargetLanguage::TypeScript => output_path.join("client_sdk").join("models"),
        }
    }
    
    fn get_model_file_path(&self, models_dir: &PathBuf, model_name: &str) -> PathBuf {
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                models_dir.join(format!("{}.java", model_name))
            }
            crate::core::config::TargetLanguage::Python => {
                models_dir.join(format!("{}.py", crate::core::template::to_snake_case(model_name)))
            }
            crate::core::config::TargetLanguage::Rust => {
                models_dir.join(format!("{}.rs", crate::core::template::to_snake_case(model_name)))
            },
            TargetLanguage::Go => models_dir.join(format!("{}.go", crate::core::template::to_snake_case(model_name))),
            TargetLanguage::TypeScript => models_dir.join(format!("{}.ts", crate::core::template::to_snake_case(model_name))),
        }
    }
    
    fn get_tests_directory(&self, output_path: &PathBuf) -> PathBuf {
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                output_path.join("src").join("test").join("java")
            }
            crate::core::config::TargetLanguage::Python => {
                output_path.join("tests")
            }
            crate::core::config::TargetLanguage::Rust => {
                output_path.join("tests")
            },
            TargetLanguage::Go => output_path.join("tests"),
            TargetLanguage::TypeScript => output_path.join("tests"),
        }
    }
    
    fn get_test_file_path(&self, tests_dir: &PathBuf) -> PathBuf {
        match self.config.target_language {
            crate::core::config::TargetLanguage::Java => {
                tests_dir.join("ClientTest.java")
            }
            crate::core::config::TargetLanguage::Python => {
                tests_dir.join("test_client.py")
            }
            crate::core::config::TargetLanguage::Rust => {
                tests_dir.join("integration_tests.rs")
            },
            TargetLanguage::Go => tests_dir.join("client_test.go"),
            TargetLanguage::TypeScript => tests_dir.join("client_test.ts"),
        }
    }
}