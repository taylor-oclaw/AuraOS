extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn rust_start() -> ! {
    let mut runner = AIBenchmarkRunnerV2::new();
    runner.register_benchmark(String::from("Benchmark1"), Box::new(|| 42));
    runner.register_benchmark(String::from("Benchmark2"), Box::new(|| 84));
    runner.run_all_benchmarks();
    loop {}
}

pub struct AIBenchmarkRunnerV2 {
    benchmarks: Vec<(String, Box<dyn Fn() -> i32>)>,
}

impl AIBenchmarkRunnerV2 {
    pub fn new() -> Self {
        AIBenchmarkRunnerV2 {
            benchmarks: Vec::new(),
        }
    }

    pub fn register_benchmark(&mut self, name: String, benchmark: Box<dyn Fn() -> i32>) {
        self.benchmarks.push((name, benchmark));
    }

    pub fn run_all_benchmarks(&self) {
        for (name, benchmark) in &self.benchmarks {
            let result = benchmark();
            // Simulate logging the result
            println!("Benchmark {} completed with result: {}", name, result);
        }
    }

    pub fn get_benchmark_count(&self) -> usize {
        self.benchmarks.len()
    }

    pub fn get_benchmark_names(&self) -> Vec<String> {
        self.benchmarks.iter().map(|(name, _)| name.clone()).collect()
    }

    pub fn run_specific_benchmark(&self, index: usize) -> Option<i32> {
        if let Some((_, benchmark)) = self.benchmarks.get(index) {
            Some(benchmark())
        } else {
            None
        }
    }
}
