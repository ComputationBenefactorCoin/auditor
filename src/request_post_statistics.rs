use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestPostStatistics {
    cpu_count: u16,
    cpu_idle: f32,
    cpu_interrupt: f32,
    cpu_nice: f32,
    cpu_system: f32,
    cpu_user: f32,
    host_id: String,
    mem_free: u64,
    mem_usage: f32,
    mem_total: u64,
    mt_2_result: f64,
    mt_4_result: f64,
    mt_8_result: f64,
    public_key: String,
    st_result: f64,
    sys_load_average_fifteen: f32,
    sys_load_average_five: f32,
    sys_load_average_one: f32,
    sys_uptime: f64,
}

impl RequestPostStatistics {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cpu_count: u16,
        cpu_idle: f32,
        cpu_interrupt: f32,
        cpu_nice: f32,
        cpu_system: f32,
        cpu_user: f32,
        host_id: String,
        mem_free: u64,
        mem_usage: f32,
        mem_total: u64,
        mt_2_result: f64,
        mt_4_result: f64,
        mt_8_result: f64,
        public_key: String,
        st_result: f64,
        sys_load_average_fifteen: f32,
        sys_load_average_five: f32,
        sys_load_average_one: f32,
        sys_uptime: f64,
    ) -> Self {
        Self {
            cpu_count,
            cpu_idle,
            cpu_interrupt,
            cpu_nice,
            cpu_system,
            cpu_user,
            host_id,
            mem_free,
            mem_usage,
            mem_total,
            mt_2_result,
            mt_4_result,
            mt_8_result,
            public_key,
            st_result,
            sys_load_average_fifteen,
            sys_load_average_five,
            sys_load_average_one,
            sys_uptime,
        }
    }

    pub fn cpu_count(&self) -> &u16 {
        &self.cpu_count
    }

    pub fn cpu_idle(&self) -> &f32 {
        &self.cpu_idle
    }

    pub fn cpu_interrupt(&self) -> &f32 {
        &self.cpu_interrupt
    }

    pub fn cpu_nice(&self) -> &f32 {
        &self.cpu_nice
    }

    pub fn cpu_system(&self) -> &f32 {
        &self.cpu_system
    }

    pub fn cpu_user(&self) -> &f32 {
        &self.cpu_user
    }

    pub fn host_id(&self) -> &String {
        &self.host_id
    }

    pub fn mem_free(&self) -> &u64 {
        &self.mem_free
    }

    pub fn mem_usage(&self) -> &f32 {
        &self.mem_usage
    }

    pub fn mem_total(&self) -> &u64 {
        &self.mem_total
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

    pub fn public_key(&self) -> &String {
        &self.public_key
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
}
