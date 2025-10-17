pub mod java;
pub mod typescript;

pub use java::JavaTestGenerator;
pub use typescript::TypeScriptTestGenerator;

use anyhow::Result;
use std::path::PathBuf;

use crate::core::config::TargetLanguage;
use crate::core::test_generation::*;

pub struct TestGeneratorFactory;

impl TestGeneratorFactory {
    pub fn create_generator(language: TargetLanguage) -> Result<Box<dyn TestGenerator>> {
        match language {
            TargetLanguage::Java => Ok(Box::new(JavaTestGenerator::new())),
            TargetLanguage::TypeScript => Ok(Box::new(TypeScriptTestGenerator::new())),
            _ => {
                // For now, return a stub generator for unsupported languages
                Ok(Box::new(StubTestGenerator::new(language)))
            }
        }
    }
}

/// Stub test generator for languages not yet implemented
pub struct StubTestGenerator {
    target: TargetLanguage,
}

impl StubTestGenerator {
    pub fn new(target: TargetLanguage) -> Self {
        Self { target }
    }
}

impl TestGenerator for StubTestGenerator {
    fn generate_test_suite(
        &self,
        _api_spec: &crate::core::types::ApiSpec,
        _config: &TestGenerationConfig,
        _output_path: &PathBuf,
    ) -> Result<TestSuite> {
        Ok(TestSuite {
            name: "stub_test_suite".to_string(),
            files: vec![],
            mock_data_files: vec![],
            config_files: vec![],
        })
    }

    fn generate_client_tests(
        &self,
        _api_spec: &crate::core::types::ApiSpec,
        _config: &TestGenerationConfig,
    ) -> Result<Vec<TestFile>> {
        Ok(vec![])
    }

    fn generate_model_tests(
        &self,
        _types: &[crate::core::types::TypeDefinition],
        _config: &TestGenerationConfig,
    ) -> Result<Vec<TestFile>> {
        Ok(vec![])
    }

    fn generate_integration_tests(
        &self,
        _api_spec: &crate::core::types::ApiSpec,
        _config: &TestGenerationConfig,
    ) -> Result<Vec<TestFile>> {
        Ok(vec![])
    }

    fn generate_mock_data(
        &self,
        _api_spec: &crate::core::types::ApiSpec,
        _config: &TestGenerationConfig,
    ) -> Result<Vec<MockDataFile>> {
        Ok(vec![])
    }

    fn generate_test_configs(
        &self,
        _config: &TestGenerationConfig,
    ) -> Result<Vec<ConfigFile>> {
        Ok(vec![])
    }

    fn target_language(&self) -> TargetLanguage {
        self.target
    }
}