use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::core::config::TargetLanguage;
use crate::core::types::{ApiSpec, Operation, TypeDefinition};

/// Test generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestGenerationConfig {
    pub enabled: bool,
    pub generate_unit_tests: bool,
    pub generate_integration_tests: bool,
    pub generate_mock_data: bool,
    pub test_coverage_target: Option<f32>,
    pub framework_preferences: TestFrameworkPreferences,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFrameworkPreferences {
    pub java_framework: JavaTestFramework,
    pub typescript_framework: TypeScriptTestFramework,
    pub python_framework: PythonTestFramework,
    pub go_framework: GoTestFramework,
    pub rust_framework: RustTestFramework,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JavaTestFramework {
    JUnit5,
    TestNG,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeScriptTestFramework {
    Jest,
    Vitest,
    Mocha,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PythonTestFramework {
    Pytest,
    Unittest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoTestFramework {
    StandardTesting,
    Testify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RustTestFramework {
    StandardTesting,
    Tokio,
}

/// Test suite that will be generated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestSuite {
    pub name: String,
    pub files: Vec<TestFile>,
    pub mock_data_files: Vec<MockDataFile>,
    pub config_files: Vec<ConfigFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestFile {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
    pub test_type: TestType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockDataFile {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub name: String,
    pub path: PathBuf,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    Unit,
    Integration,
    End2End,
    Performance,
}

/// Test case that will be generated for each operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub name: String,
    pub description: String,
    pub operation: Operation,
    pub test_scenarios: Vec<TestScenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestScenario {
    pub name: String,
    pub description: String,
    pub scenario_type: TestScenarioType,
    pub setup: Vec<String>,
    pub execution: Vec<String>,
    pub assertions: Vec<String>,
    pub cleanup: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestScenarioType {
    Success,
    ValidationError,
    NetworkError,
    AuthenticationError,
    AuthorizationError,
    RateLimitError,
    ServerError,
    TimeoutError,
}

/// Trait that all language-specific test generators must implement
pub trait TestGenerator {
    /// Generate all test files for the given API specification
    fn generate_test_suite(
        &self,
        api_spec: &ApiSpec,
        config: &TestGenerationConfig,
        output_path: &PathBuf,
    ) -> Result<TestSuite>;

    /// Generate unit tests for client classes
    fn generate_client_tests(
        &self,
        api_spec: &ApiSpec,
        config: &TestGenerationConfig,
    ) -> Result<Vec<TestFile>>;

    /// Generate unit tests for model classes
    fn generate_model_tests(
        &self,
        types: &[TypeDefinition],
        config: &TestGenerationConfig,
    ) -> Result<Vec<TestFile>>;

    /// Generate integration tests for API operations
    fn generate_integration_tests(
        &self,
        api_spec: &ApiSpec,
        config: &TestGenerationConfig,
    ) -> Result<Vec<TestFile>>;

    /// Generate mock data for testing
    fn generate_mock_data(
        &self,
        api_spec: &ApiSpec,
        config: &TestGenerationConfig,
    ) -> Result<Vec<MockDataFile>>;

    /// Generate test configuration files (e.g., test runners, coverage configs)
    fn generate_test_configs(
        &self,
        config: &TestGenerationConfig,
    ) -> Result<Vec<ConfigFile>>;

    /// Get the target language for this generator
    fn target_language(&self) -> TargetLanguage;
}

impl Default for TestGenerationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            generate_unit_tests: true,
            generate_integration_tests: true,
            generate_mock_data: true,
            test_coverage_target: Some(80.0),
            framework_preferences: TestFrameworkPreferences::default(),
        }
    }
}

impl Default for TestFrameworkPreferences {
    fn default() -> Self {
        Self {
            java_framework: JavaTestFramework::JUnit5,
            typescript_framework: TypeScriptTestFramework::Jest,
            python_framework: PythonTestFramework::Pytest,
            go_framework: GoTestFramework::StandardTesting,
            rust_framework: RustTestFramework::StandardTesting,
        }
    }
}

/// Helper functions for test generation
pub fn generate_test_cases_for_operation(operation: &Operation) -> Vec<TestCase> {
    let mut test_cases = vec![];
    
    // Generate success case
    let success_scenarios = vec![
        TestScenario {
            name: "should_succeed_with_valid_parameters".to_string(),
            description: format!("Test successful execution of {} operation", operation.name),
            scenario_type: TestScenarioType::Success,
            setup: vec!["// Setup valid test data".to_string()],
            execution: vec![format!("// Call {}", operation.name)],
            assertions: vec!["// Assert successful response".to_string()],
            cleanup: vec!["// Cleanup if needed".to_string()],
        }
    ];

    test_cases.push(TestCase {
        name: format!("{}Test", operation.name),
        description: format!("Test cases for {} operation", operation.name),
        operation: operation.clone(),
        test_scenarios: success_scenarios,
    });

    // Generate error cases based on parameters
    if !operation.parameters.is_empty() {
        let mut error_scenarios = vec![];
        
        // Required parameter validation
        for param in &operation.parameters {
            if param.required {
                error_scenarios.push(TestScenario {
                    name: format!("should_fail_when_{}_is_missing", param.name),
                    description: format!("Test validation error when required parameter {} is missing", param.name),
                    scenario_type: TestScenarioType::ValidationError,
                    setup: vec!["// Setup test data with missing parameter".to_string()],
                    execution: vec![format!("// Call {} with missing {}", operation.name, param.name)],
                    assertions: vec!["// Assert validation error".to_string()],
                    cleanup: vec![],
                });
            }
        }

        if !error_scenarios.is_empty() {
            test_cases.push(TestCase {
                name: format!("{}ValidationTest", operation.name),
                description: format!("Validation test cases for {} operation", operation.name),
                operation: operation.clone(),
                test_scenarios: error_scenarios,
            });
        }
    }

    // Generate network error cases
    let network_error_scenarios = vec![
        TestScenario {
            name: "should_handle_network_timeout".to_string(),
            description: format!("Test network timeout handling for {} operation", operation.name),
            scenario_type: TestScenarioType::TimeoutError,
            setup: vec!["// Setup timeout scenario".to_string()],
            execution: vec![format!("// Call {} with timeout", operation.name)],
            assertions: vec!["// Assert timeout exception".to_string()],
            cleanup: vec![],
        },
        TestScenario {
            name: "should_handle_server_error".to_string(),
            description: format!("Test server error handling for {} operation", operation.name),
            scenario_type: TestScenarioType::ServerError,
            setup: vec!["// Setup server error scenario".to_string()],
            execution: vec![format!("// Call {} with server error", operation.name)],
            assertions: vec!["// Assert server error handling".to_string()],
            cleanup: vec![],
        },
    ];

    test_cases.push(TestCase {
        name: format!("{}ErrorHandlingTest", operation.name),
        description: format!("Error handling test cases for {} operation", operation.name),
        operation: operation.clone(),
        test_scenarios: network_error_scenarios,
    });

    test_cases
}