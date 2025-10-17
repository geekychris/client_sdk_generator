use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BaseType {
    String,
    Integer,
    Object(String),
    Any,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeReference {
    pub base_type: BaseType,
    pub nullable: bool,
}

fn main() {
    let type_ref = TypeReference {
        base_type: BaseType::Object("User".to_string()),
        nullable: false,
    };
    
    let json = serde_json::to_string_pretty(&type_ref).unwrap();
    println!("JSON output:");
    println!("{}", json);
}