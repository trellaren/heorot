// src/warehouse/models.rs

use std::fmt;

/// Custom error type for storage operations.
#[derive(Debug)]
pub enum StorageError {
    NotFound,
    DatabaseError(String),
    FileIOError(std::io::Error),
    InvalidFormat(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StorageError::NotFound => write!(f, "Data item not found."),
            StorageError::DatabaseError(e) => write!(f, "Database error: {}", e),
            StorageError::FileIOError(e) => write!(f, "File I/O error: {}", e),
            StorageError::InvalidFormat(s) => write!(f, "Invalid data format provided: {}", s),
        }
    }
}

// Implement From traits for easier error handling (optional but good practice).
impl std::error::Error for StorageError {}
impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        println!("[MODEL] Converting std::io::Error to StorageError::FileIOError");
        StorageError::FileIOError(err)
    }
}


/// Represents a chunk of Language Model data. This is the core entity managed by the warehouse.
#[derive(Debug, Clone)]
pub struct LlmItem {
    // Unique identifier for retrieval
    pub id: String,

    // Metadata about the source (e.g., "GPT-4", "Claude3", "Custom Format")
    pub source_type: String,
    
    // The chunked text content itself
    pub chunk_text: String,
    
    // Original source reference (e.g., file path or document ID)
    pub original_source: String,

    // Timestamp of creation/last update
    pub created_at: u64,
}


/// Defines supported formats for LLM models and data sources.
#[derive(Debug)]
pub enum SourceType {
    LlmModel(String), // e.g., "GPT-3.5", "Mistral"
    CustomFormat,     // For user-defined or non-standard formats
    Other(String),    // Fallback for unknown sources
}

impl From<&str> for SourceType {
    fn from(s: &str) -> Self {
        println!("[MODEL] Converting source string to SourceType enum...");
        println!("[MODEL] Input source string: '{}'", s);
        
        let lowercase_source = s.to_lowercase();
        println!("[MODEL] Lowercase version of source: '{}'", lowercase_source);
        
        match lowercase_source.as_str() {
            "gpt-4" => {
                println!("[MODEL] Matched 'GPT-4' model - creating LlmModel variant");
                SourceType::LlmModel("GPT-4".into())
            }
            "claude3" => {
                println!("[MODEL] Matched 'Claude 3' model - creating LlmModel variant");
                SourceType::LlmModel("Claude 3".into())
            }
            // Add other specific model mappings here...
            _ => {
                println!("[MODEL] No direct match found - creating Other variant with original string");
                SourceType::Other(s.to_string())
            }
        }
    }
}

impl LlmItem {
    /// Helper constructor for creating a new item.
    pub fn new(id: String, source_type: SourceType, chunk_text: String, original_source: String) -> Self {
        println!("[MODEL] Creating new LlmItem instance...");
        println!("[MODEL] Item ID: {}", id);
        println!("[MODEL] Source Type variant: {:?}", source_type);
        println!("[MODEL] Chunk text length: {} bytes", chunk_text.len());
        println!("[MODEL] Original source: {}", original_source);

        let created_at = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        println!("[MODEL] Generated timestamp for created_at: {}", created_at);
        
        let item = LlmItem {
            id,
            source_type: format!("{:?}", source_type), // Use debug representation for simplicity now
            chunk_text,
            original_source,
            created_at,
        };

        println!("[MODEL] LlmItem instance creation complete.");
        println!("[MODEL] Final item structure - ID: {}, SourceType: {}", item.id, item.source_type);
        
        item
    }
}