use std::time::Instant;
use std::usize;
use std::{iter, thread};

#[derive(Debug)]
pub struct ClientBenchmark {
    mt_2_result: f64,
    mt_4_result: f64,
    mt_8_result: f64,
    st_result: f64,
}

impl ClientBenchmark {
    pub fn new(mt_2_result: f64, mt_4_result: f64, mt_8_result: f64, st_result: f64) -> Self {
        Self {
            mt_2_result,
            mt_4_result,
            mt_8_result,
            st_result,
        }
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
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_loop(loops: &u32) -> u32 {
    let mut result: u32 = 0;

    for n in 0..*loops {
        result = fibonacci(n as u32);
    }

    result
}

pub fn run() -> ClientBenchmark {
    // 44
    let loops: u32 = 32;
    let mt_2_microseconds: u128;
    let mt_2_seconds: f64;
    let mt_2_result: f64;
    let mt_2_threads: usize = 2;
    let mt_2_items = iter::repeat(0);
    let mt_4_microseconds: u128;
    let mt_4_seconds: f64;
    let mt_4_result: f64;
    let mt_4_threads: usize = 4;
    let mt_4_items = iter::repeat(0);
    let mt_8_microseconds: u128;
    let mt_8_seconds: f64;
    let mt_8_result: f64;
    let mt_8_threads: usize = 8;
    let mt_8_items = iter::repeat(0);
    let st_microseconds: u128;
    let st_seconds: f64;
    let st_result: f64;

    let st_now: Instant = Instant::now();
    fibonacci_loop(&loops);
    st_microseconds = st_now.elapsed().as_micros();
    st_seconds = st_microseconds as f64 / 1000000.0;
    st_result = st_seconds / loops as f64;

    let mt_2_now: Instant = Instant::now();
    let threads: Vec<_> = mt_2_items
        .take(mt_2_threads)
        .into_iter()
        .map(|_| {
            thread::spawn(move || {
                fibonacci_loop(&loops);
            })
        })
        .collect();
    for handle in threads {
        handle.join().unwrap()
    }

    mt_2_microseconds = mt_2_now.elapsed().as_micros();
    mt_2_seconds = mt_2_microseconds as f64 / 1000000.0;
    mt_2_result = mt_2_seconds / (loops * 2) as f64;

    let mt_4_now: Instant = Instant::now();
    let threads: Vec<_> = mt_4_items
        .take(mt_4_threads)
        .into_iter()
        .map(|_| {
            thread::spawn(move || {
                fibonacci_loop(&loops);
            })
        })
        .collect();
    for handle in threads {
        handle.join().unwrap()
    }

    mt_4_microseconds = mt_4_now.elapsed().as_micros();
    mt_4_seconds = mt_4_microseconds as f64 / 1000000.0;
    mt_4_result = mt_4_seconds / (loops * 4) as f64;

    let mt_8_now: Instant = Instant::now();
    let threads: Vec<_> = mt_8_items
        .take(mt_8_threads)
        .into_iter()
        .map(|_| {
            thread::spawn(move || {
                fibonacci_loop(&loops);
            })
        })
        .collect();
    for handle in threads {
        handle.join().unwrap()
    }

    mt_8_microseconds = mt_8_now.elapsed().as_micros();
    mt_8_seconds = mt_8_microseconds as f64 / 1000000.0;
    mt_8_result = mt_8_seconds / (loops * 8) as f64;

    ClientBenchmark::new(mt_2_result, mt_4_result, mt_8_result, st_result)
}
