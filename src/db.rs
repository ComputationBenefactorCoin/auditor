use crate::db_record::DbRecord;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    database: HashMap<String, DbRecord>,
    data_dir: String,
}

impl Db {
    pub fn new(data_dir: String) -> Self {
        Self {
            database: HashMap::new(),
            data_dir,
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&DbRecord> {
        self.database.get(key)
    }

    pub fn insert(&mut self, key: String, value: DbRecord) -> Option<DbRecord> {
        self.database.insert(key, value)
    }

    pub fn remove(&mut self, key: &str) -> Option<DbRecord> {
        self.database.remove(key)
    }

    pub fn restore(&mut self) {
        let database_file_path: String =
            format!("{dir}{file}", dir = self.data_dir, file = "db.dat");

        if Path::new(&database_file_path).exists() {
            let compressed: Vec<u8> =
                fs::read(database_file_path).expect("Failed to read from database");
            let uncompressed: Vec<u8> = decompress_size_prepended(&compressed).unwrap();
            let serialized: String =
                String::from_utf8(uncompressed).expect("Failed to read from database");

            self.database = serde_json::from_str(serialized.as_str()).unwrap();
        }
    }

    pub fn save(&self) {
        let database_file_path: String =
            format!("{dir}{file}", dir = self.data_dir, file = "db.dat");
        let serialized: Vec<u8> = serde_json::to_vec(&self.database).unwrap();
        let compressed: Vec<u8> = compress_prepend_size(&serialized);

        fs::write(database_file_path, compressed).expect("Failed to write to database");
    }
}
