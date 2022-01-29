use std::thread;
use std::time::Duration;
use systemstat::{CPULoad, Platform, System};

#[derive(Debug)]
pub struct ClientStatistics {
    cpu_count: u16,
    cpu_idle: f32,
    cpu_interrupt: f32,
    cpu_nice: f32,
    cpu_system: f32,
    cpu_user: f32,
    mem_free: u64,
    mem_usage: f32,
    mem_total: u64,
    sys_load_average_fifteen: f32,
    sys_load_average_five: f32,
    sys_load_average_one: f32,
    sys_uptime: f64,
}

impl ClientStatistics {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        cpu_count: u16,
        cpu_idle: f32,
        cpu_interrupt: f32,
        cpu_nice: f32,
        cpu_system: f32,
        cpu_user: f32,
        mem_free: u64,
        mem_usage: f32,
        mem_total: u64,
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
            mem_free,
            mem_usage,
            mem_total,
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

    pub fn mem_free(&self) -> &u64 {
        &self.mem_free
    }

    pub fn mem_usage(&self) -> &f32 {
        &self.mem_usage
    }

    pub fn mem_total(&self) -> &u64 {
        &self.mem_total
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

pub fn get() -> ClientStatistics {
    let system = System::new();
    let cpu_count: u16 = num_cpus::get() as u16;
    let mut cpu_idle: f32 = 0.0;
    let mut cpu_interrupt: f32 = 0.0;
    let mut cpu_nice: f32 = 0.0;
    let mut cpu_system: f32 = 0.0;
    let mut cpu_user: f32 = 0.0;
    let mut mem_free: u64 = 0;
    let mut mem_usage: f32 = 0.0;
    let mut mem_total: u64 = 0;
    let mut sys_load_average_fifteen: f32 = 0.0;
    let mut sys_load_average_five: f32 = 0.0;
    let mut sys_load_average_one: f32 = 0.0;
    let mut sys_uptime: f64 = 0.0;

    match system.cpu_load_aggregate() {
        Ok(cpu) => {
            thread::sleep(Duration::from_secs(1));
            let cpu: CPULoad = cpu.done().unwrap();

            cpu_idle = cpu.idle * 100.0;
            cpu_interrupt = cpu.interrupt * 100.0;
            cpu_nice = cpu.nice * 100.0;
            cpu_system = cpu.system * 100.0;
            cpu_user = cpu.user * 100.0;
        }
        Err(x) => println!("\nCPU load: error: {}", x),
    }

    match system.memory() {
        Ok(memory) => {
            mem_free = memory.free.as_u64();
            mem_total = memory.total.as_u64();
            mem_usage = (mem_free as f64 / mem_total as f64) as f32;
        }
        Err(x) => println!("\nMemory: error: {}", x),
    }

    match system.load_average() {
        Ok(load_average) => {
            sys_load_average_fifteen = load_average.fifteen;
            sys_load_average_five = load_average.five;
            sys_load_average_one = load_average.one;
        }
        Err(x) => println!("\nLoad average: error: {}", x),
    }

    match system.uptime() {
        Ok(uptime) => {
            sys_uptime = uptime.as_secs_f64();
        }
        Err(x) => println!("\nuptime: error: {}", x),
    }

    ClientStatistics::new(
        cpu_count,
        cpu_idle,
        cpu_interrupt,
        cpu_nice,
        cpu_system,
        cpu_user,
        mem_free,
        mem_usage,
        mem_total,
        sys_load_average_fifteen,
        sys_load_average_five,
        sys_load_average_one,
        sys_uptime,
    )
}
