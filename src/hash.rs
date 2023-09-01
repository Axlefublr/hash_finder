use ring::digest::{Context, SHA256};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::MAX_ZEROS;

fn create_sha256(input: String) -> String {
	let mut context = Context::new(&SHA256);
	context.update(input.as_bytes());
	let digest = context.finish();

	hex::encode(digest.as_ref())
}

/// Uses multiple threads to find your amount of hashes that have your amount of trailing zeros. Prints them to stdout.
///
/// # Arguments
/// * `zeros_count` - The amount of consecutive trailing zeros in the hashes
/// * `max_results_count` - The amount of matching hashes to find and print
///
/// # Usage
/// ```
/// let result = multithread_hashing(args.zeros_count, args.max_results_count);
/// match result {
///     Ok(()) => ExitCode::SUCCESS,
///     Err(message) => {
///         eprintln!("{}", message);
///         ExitCode::FAILURE
///     }
/// }
/// ```
pub fn multithread_hashing(zeros_count: usize, max_results_count: u32) -> Result<(), &'static str> {
	if zeros_count > MAX_ZEROS {
		return Err("the hash string is 64 symbols long. it will never contain 65 trailing zeros");
	}

	let counter = Arc::new(Mutex::new(1));
	let found_hashes = Arc::new(Mutex::new(0));

	let num_threads = num_cpus::get();
	let mut handles = vec![];

	for _ in 0..num_threads {
		let handle = spawn_thread(&counter, &found_hashes, zeros_count, max_results_count);
		handles.push(handle);
	}

	for handle in handles {
		handle.join().unwrap();
	}
	Ok(())
}

fn spawn_thread(counter: &Arc<Mutex<u32>>, found_hashes: &Arc<Mutex<u32>>, zeros_count: usize, max_results_count: u32) -> JoinHandle<()> {
	let counter = Arc::clone(counter);
	let found_hashes = Arc::clone(found_hashes);
	thread::spawn(move || loop {
		let i = {
			let mut num = counter.lock().unwrap();
			let i = *num;
			*num += 1;
			i
		};

		let hash = create_sha256(format!("{}", i));
		if hash.ends_with(&"0".repeat(zeros_count)) {
			{
				let mut num = found_hashes.lock().unwrap();
				*num += 1;
			}
			println!("{}, \"{}\"", i, hash);
		}

		if *found_hashes.lock().unwrap() >= max_results_count {
			break;
		}
	})
}