// src/warehouse/lib.rs

use crate::models::{LlmItem, StorageError};
use crate::storage_db;
use crate::storage_file;

/// Public API for the LLM data storage backend.
/// Handles persistence (flat files and embedded DB) and retrieval of chunked language model context data.
pub struct Warehouse {
    // In a real implementation, this would hold connections to both file system and database.
}

impl Warehouse {
    /// Initializes the warehouse by setting up necessary resources (e.g., connecting to the embedded database).
    pub fn new(db_path: &str) -> Result<Self, StorageError> {
        println!("[WAREHOUSE] Initializing Warehouse storage backend...");
        println!("[WAREHOUSE] Database path configured: {}", db_path);
        
        // Initialize DB connection here
        match storage_db::DatabaseStorage::new(db_path) {
            Ok(_db_storage) => {
                println!("[WAREHOUSE] Embedded database connection established successfully.");
            }
            Err(e) => {
                println!("[WAREHOUSE] ERROR: Failed to establish embedded database connection: {}", e);
                return Err(e);
            }
        }

        // Verify file storage directory accessibility
        let storage_dir = "data/warehouse/";
        match std::fs::create_dir_all(storage_dir) {
            Ok(_) => {
                println!("[WAREHOUSE] File storage directory prepared at: {}", storage_dir);
            }
            Err(e) => {
                println!("[WAREHOUSE] WARNING: Could not create file storage directory '{}': {}", storage_dir, e);
            }
        }

        println!("[WAREHOUSE] Warehouse initialization complete.");
        Ok(Warehouse {})
    }

    /// Stores an LLM item into both the flat file system and the embedded database.
    /// This method is responsible for data persistence.
    pub fn store_item(&self, item: &LlmItem) -> Result<(), StorageError> {
        println!("[WAREHOUSE] Received request to store LLM item.");
        println!("[WAREHOUSE] Item ID: {}", item.id);
        println!("[WAREHOUSE] Source Type: {}", item.source_type);
        println!("[WAREHOUSE] Chunk text length: {} bytes", item.chunk_text.len());
        println!("[WAREHOUSE] Original source: {}", item.original_source);
        println!("[WAREHOUSE] Created at timestamp: {}", item.created_at);

        // 1. Persist to Embedded Database
        println!("[WAREHOUSE] Step 1/2: Attempting to persist item to embedded database...");
        match storage_db::save_item(item) {
            Ok(()) => {
                println!("[WAREHOUSE] SUCCESS: Item '{}' persisted to embedded database.", item.id);
            }
            Err(e) => {
                println!("[WAREHOUSE] ERROR: Failed to persist item '{}' to embedded database: {}", item.id, e);
                return Err(e);
            }
        }

        // 2. Persist to Flat Files (for backup/alternative retrieval methods)
        println!("[WAREHOUSE] Step 2/2: Attempting to persist item to flat file system...");
        match storage_file::save_item(item) {
            Ok(()) => {
                println!("[WAREHOUSE] SUCCESS: Item '{}' persisted to flat file system.", item.id);
            }
            Err(e) => {
                println!("[WAREHOUSE] ERROR: Failed to persist item '{}' to flat file system: {}", item.id, e);
                return Err(e);
            }
        }

        println!("[WAREHOUSE] Item '{}' has been successfully stored in both storage systems.", item.id);
        Ok(())
    }

    /// Retrieves a chunk of LLM data given a unique identifier or query criteria.
    pub fn retrieve_context(&self, item_id: &str) -> Result<LlmItem, StorageError> {
        println!("[WAREHOUSE] Received request to retrieve context for item ID: {}", item_id);
        
        // Check if the database contains the requested item
        println!("[WAREHOUSE] Querying embedded database for item '{}'", item_id);
        match storage_db::load_item(item_id) {
            Ok(retrieved_item) => {
                println!("[WAREHOUSE] SUCCESS: Retrieved context from embedded database.");
                println!("[WAREHOUSE] Retrieved item - ID: {}, Source: {}", retrieved_item.id, retrieved_item.source_type);
                println!("[WAREHOUSE] Chunk text preview (first 100 chars): {:.100}...", &retrieved_item.chunk_text);
                Ok(retrieved_item)
            }
            Err(e) => {
                println!("[WAREHOUSE] WARNING: Item '{}' not found in embedded database.", item_id);
                println!("[WAREHOUSE] Attempting fallback to flat file storage...");
                
                // Try file system as fallback
                match storage_file::load_item(item_id) {
                    Ok(fallback_item) => {
                        println!("[WAREHOUSE] SUCCESS: Retrieved context from flat file storage as fallback.");
                        Ok(fallback_item)
                    }
                    Err(file_err) => {
                        println!("[WAREHOUSE] ERROR: Failed to retrieve item '{}' from both database and file storage.", item_id);
                        Err(file_err)
                    }
                }
            }
        }
    }

    // Future methods for chunking management and complex queries will be added here.
}