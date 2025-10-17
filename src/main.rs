// Copyright (c) 2024 Chris Collins <chris@hitorro.com>
// Licensed under the MIT License

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber;

mod core;
mod generators;
mod parsers;

use crate::core::generator::SdkGenerator;
use crate::core::config::{GeneratorConfig, InputType, TargetLanguage};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "sdk-gen")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate SDK from OpenAPI specification
    Openapi {
        /// Path to OpenAPI spec file (JSON or YAML)
        #[arg(short, long)]
        spec: PathBuf,
        
        /// Target language for SDK generation
        #[arg(short, long)]
        language: TargetLanguage,
        
        /// Output directory for generated SDK
        #[arg(short, long)]
        output: PathBuf,
        
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Generate SDK from GraphQL schema
    Graphql {
        /// Path to GraphQL schema file
        #[arg(short, long)]
        schema: PathBuf,
        
        /// Target language for SDK generation
        #[arg(short, long)]
        language: TargetLanguage,
        
        /// Output directory for generated SDK
        #[arg(short, long)]
        output: PathBuf,
        
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Generate SDK from gRPC proto files
    Grpc {
        /// Path to proto files directory or single proto file
        #[arg(short, long)]
        proto: PathBuf,
        
        /// Target language for SDK generation
        #[arg(short, long)]
        language: TargetLanguage,
        
        /// Output directory for generated SDK
        #[arg(short, long)]
        output: PathBuf,
        
        /// Configuration file path
        #[arg(short, long)]
        config: Option<PathBuf>,
    },
    
    /// Show available generators and their capabilities
    List,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let level = if cli.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();
    
    match cli.command {
        Commands::Openapi { spec, language, output, config } => {
            info!("Generating {} SDK from OpenAPI spec: {:?}", language, spec);
            let generator_config = load_config(config, InputType::OpenApi, language)?;
        let generator = SdkGenerator::new(generator_config)?;
        generator.generate_from_openapi(spec, output).await?
        }
        
        Commands::Graphql { schema, language, output, config } => {
            info!("Generating {} SDK from GraphQL schema: {:?}", language, schema);
            let generator_config = load_config(config, InputType::GraphQL, language)?;
        let generator = SdkGenerator::new(generator_config)?;
        generator.generate_from_graphql(schema, output).await?
        }
        
        Commands::Grpc { proto, language, output, config } => {
            info!("Generating {} SDK from gRPC proto: {:?}", language, proto);
            let generator_config = load_config(config, InputType::Grpc, language)?;
        let generator = SdkGenerator::new(generator_config)?;
        generator.generate_from_grpc(proto, output).await?
        }
        
        Commands::List => {
            println!("Available SDK Generators:");
            println!("  Input Types: OpenAPI, GraphQL, gRPC");
            println!("  Target Languages: Java, Python, Rust");
            println!("  Features: Retry logic, Telemetry, Caching, Async support");
        }
    }
    
    Ok(())
}

fn load_config(
    config_path: Option<PathBuf>, 
    input_type: InputType, 
    language: TargetLanguage
) -> Result<GeneratorConfig> {
    match config_path {
        Some(path) => GeneratorConfig::from_file(path),
        None => Ok(GeneratorConfig::default_for(input_type, language)),
    }
}