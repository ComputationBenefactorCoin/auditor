use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DbStatistics {
    blockchain_hash: String,
    cpu_count: u16,
    cpu_idle: f32,
    cpu_interrupt: f32,
    cpu_nice: f32,
    cpu_system: f32,
    cpu_usage: f32,
    cpu_user: f32,
    id: String,
    mem_free: u64,
    mem_usage: f32,
    mem_total: u64,
    mt_2_result: f64,
    mt_4_result: f64,
    mt_8_result: f64,
    st_result: f64,
    sys_load_average_fifteen: f32,
    sys_load_average_five: f32,
    sys_load_average_one: f32,
    sys_uptime: f64,
    timestamp: u64,
    used_for_proof: bool,
}

impl DbStatistics {
    pub fn new(
        blockchain_hash: String,
        cpu_count: u16,
        cpu_idle: f32,
        cpu_interrupt: f32,
        cpu_nice: f32,
        cpu_system: f32,
        cpu_usage: f32,
        cpu_user: f32,
        id: String,
        mem_free: u64,
        mem_usage: f32,
        mem_total: u64,
        mt_2_result: f64,
        mt_4_result: f64,
        mt_8_result: f64,
        st_result: f64,
        sys_load_average_fifteen: f32,
        sys_load_average_five: f32,
        sys_load_average_one: f32,
        sys_uptime: f64,
        timestamp: u64,
        used_for_proof: bool,
    ) -> Self {
        Self {
            blockchain_hash,
            cpu_count,
            cpu_idle,
            cpu_interrupt,
            cpu_nice,
            cpu_system,
            cpu_usage,
            cpu_user,
            id,
            mem_free,
            mem_usage,
            mem_total,
            mt_2_result,
            mt_4_result,
            mt_8_result,
            st_result,
            sys_load_average_fifteen,
            sys_load_average_five,
            sys_load_average_one,
            sys_uptime,
            timestamp,
            used_for_proof,
        }
    }

    pub fn cpu_usage(&self) -> &f32 {
        &self.cpu_usage
    }

    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn mem_usage(&self) -> &f32 {
        &self.mem_usage
    }

    pub fn mt_2_result(&self) -> &f64 {
        &self.mt_2_result
    }

    pub fn mt_4_result(&self) -> &f64 {
        &self.mt_4_result
    }

    pub fn mt_8_result(&self) -> &f64 {
        &self.mt_8_result
    }

    pub fn st_result(&self) -> &f64 {
        &self.st_result
    }

    pub fn sys_load_average_fifteen(&self) -> &f32 {
        &self.sys_load_average_fifteen
    }

    pub fn sys_load_average_five(&self) -> &f32 {
        &self.sys_load_average_five
    }

    pub fn sys_load_average_one(&self) -> &f32 {
        &self.sys_load_average_one
    }

    pub fn sys_uptime(&self) -> &f64 {
        &self.sys_uptime
    }

    pub fn used_for_proof(&self) -> bool {
        self.used_for_proof
    }
}
