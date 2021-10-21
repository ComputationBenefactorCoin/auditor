use crate::db_statistics::DbStatistics;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponsePostStatistics {
    data: DbStatistics,
    host_id: String,
    public_key: String,
}

impl ResponsePostStatistics {
    pub fn new(data: DbStatistics, host_id: String, public_key: String) -> Self {
        Self {
            data,
            host_id,
            public_key,
        }
    }

    pub fn host_id(&self) -> &String {
        &self.host_id
    }

    pub fn public_key(&self) -> &String {
        &self.public_key
    }
}
