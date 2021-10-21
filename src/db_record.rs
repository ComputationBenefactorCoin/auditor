use crate::db_statistics::DbStatistics;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbRecord {
    public_key: String,
    statistics: Vec<DbStatistics>,
}

impl DbRecord {
    pub fn new(public_key: String, statistics: Vec<DbStatistics>) -> Self {
        Self {
            public_key,
            statistics,
        }
    }

    pub fn public_key(&self) -> &String {
        &self.public_key
    }

    pub fn statistics(&mut self) -> &mut Vec<DbStatistics> {
        &mut self.statistics
    }
}
