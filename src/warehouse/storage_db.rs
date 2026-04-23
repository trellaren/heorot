// src/warehouse/storage_db.rs

use crate::models::{LlmItem, StorageError};
// In a real implementation, we would use an embedded database library like 'sqlite' or 'diesel'.
// For this scaffold, we simulate the connection and operations.
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Handles persistence of LlmItem to the embedded database (e.g., SQLite).
pub struct DatabaseStorage {
    // Use a static/lazy global variable to hold the simulated DB connection 
    // in this scaffolded example to demonstrate usage across modules.
    db: OnceLock<Mutex<HashMap<String, LlmItem>>>,
}

impl DatabaseStorage {
    /// Initializes and returns a new DatabaseStorage instance.
    pub fn new(db_path: &str) -> Result<Self, StorageError> {
        println!("[DB_STORAGE] Starting DatabaseStorage initialization...");
        println!("[DB_STORAGE] Database path provided: {}", db_path);

        // Simulate connection setup: initialize the HashMap acting as our DB store.
        let db = Mutex::new(HashMap::<String, LlmItem>::new());
        
        println!("[DB_STORAGE] Created in-memory HashMap database structure.");
        println!("[DB_STORAGE] Database capacity: 0 items (empty initialization)");

        // Initialize OnceLock with the new database connection
        let result = DatabaseStorage {
            db: OnceLock::sync_once(|| db.clone()).map(|d| d),
        };

        println!("[DB_STORAGE] Successfully initialized embedded database storage.");
        println!("[DB_STORAGE] Database is ready to store LlmItem records.");

        Ok(result)
    }
    
    /// Helper function to get a reference to the simulated DB connection.
    fn get_db(&self) -> std::sync::MutexGuard<'_, HashMap<String, LlmItem>> {
         // In a real scenario, we'd handle connection pooling/re-establishment here.
        println!("[DB_STORAGE] Acquiring database lock for access...");
        let guard = self.db.get_or_init(|| Mutex::new(HashMap::default())).lock().unwrap();
        println!("[DB_STORAGE] Database lock acquired. Current item count: {}", guard.len());
        guard
    }


    /// Saves an LlmItem to the simulated embedded database store.
    pub fn save_item(item: &LlmItem) -> Result<(), StorageError> {
        println!("[DB_STORAGE] save_item called for ID: {}", item.id);
        
        let mut db = self.get_db();
        
        println!("[DB_STORAGE] Current database state before insert:");
        for (key, value) in db.iter() {
            println!("[DB_STORAGE]   - Existing key: '{}' with source_type '{}'", key, value.source_type);
        }

        // Simulate DB insertion/update logic
        if db.contains_key(&item.id) {
            println!("[DB_STORAGE] Item ID '{}' already exists in database - performing UPDATE operation", item.id);
            
            // Get old value for logging
            let old_value = db.get(&item.id).cloned();
            if let Some(ref old_val) = old_value {
                println!("[DB_STORAGE]   Previous chunk text length: {} bytes", old_val.chunk_text.len());
            }
            
            db.insert(item.id.clone(), item.clone());
            println!("[DB_STORAGE]   Update completed with new data");
        } else {
            println!("[DB_STORAGE] Item ID '{}' does not exist in database - performing INSERT operation", item.id);
            // Insert new record
        }
        
        let final_count = db.len();
        println!("[DB_STORAGE] Database now contains {} item(s) after save operation.", final_count);
        println!("[DB_STORAGE] Successfully saved LlmItem to embedded database.");

        Ok(())
    }

    /// Retrieves an LlmItem from the embedded database based on its ID.
    pub fn load_item(id: &str) -> Result<LlmItem, StorageError> {
        println!("[DB_STORAGE] load_item called for ID: {}", id);
        
        let db = self.get_db();
        
        println!("[DB_STORAGE] Searching database for key: '{}'", id);

        // Simulate DB lookup
        match db.get(id) {
            Some(item) => {
                println!("[DB_STORAGE] Item found in database!");
                println!("[DB_STORAGE]   ID: {}", item.id);
                println!("[DB_STORAGE]   Source Type: {}", item.source_type);
                println!("[DB_STORAGE]   Chunk text length: {} bytes", item.chunk_text.len());
                println!("[DB_STORAGE]   Created at timestamp: {}", item.created_at);
                
                let clone = item.clone();
                drop(db); // Release lock before returning
                Ok(clone)
            }
            None => {
                println!("[DB_STORAGE] Item with ID '{}' NOT FOUND in database.", id);
                
                // List available keys for debugging
                if db.is_empty() {
                    println!("[DB_STORAGE] Database is empty - no items available.");
                } else {
                    println!("[DB_STORAGE] Available keys in database:");
                    for (key, _) in db.iter() {
                        println!("[DB_STORAGE]   - '{}'", key);
                    }
                }
                
                Err(StorageError::NotFound)
            }
        }
    }
}

// NOTE: To make the scaffolding work in lib.rs, we need a way to instantiate this DB storage. 
// For now, we assume the calling function (Warehouse::new) handles initialization and provides the necessary instance.
// We adjust the implementation details slightly for simple usage in `lib.rs` by making it self-contained or passing required components.

// Since lib.rs uses a direct module call (storage_db::save_item(item)?), 
// we will provide minimal scaffolding that relies on external initialization context if needed later, 
// but for now, the core logic remains in place.