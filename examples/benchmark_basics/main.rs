use std::thread;
use std::time::Duration;

use rust_learning::benchmark::Benchmark;

fn main() {
    let mut benchmark = Benchmark::new();

    benchmark.start();
    benchmark.stop();

    println!("{}", benchmark.get_elapsed_time_message(None));
    println!("{}", benchmark.get_elapsed_time_message(Some("Benchmark")));

    benchmark.start();
    thread::sleep(Duration::from_millis(1500));
    benchmark.stop();

    println!("{}", benchmark.get_elapsed_time_message(None));
    println!("{}", benchmark.get_elapsed_time_message(Some("Benchmark")));

    benchmark.start();
    thread::sleep(Duration::from_millis(100));
    benchmark.stop();

    println!("{}", benchmark.get_elapsed_time_message(None));
    println!("{}", benchmark.get_elapsed_time_message(Some("Benchmark")));
}
