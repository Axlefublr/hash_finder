use ring::digest::{Context, SHA256};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use crate::HASH_LENGTH;

#[cfg(test)]
mod tests;

/// Creates a hash using sha256, from your input string
///
/// # Arguments
/// * `input` - the input string you want to get the hash of
///
/// # Returns
/// The hash of the string you inputted
fn create_sha256(input: String) -> String {
    let mut context = Context::new(&SHA256);
    context.update(input.as_bytes());
    let digest = context.finish();

    hex::encode(digest.as_ref())
}

/// Uses multiple threads to find your amount of hashes that have your amount of trailing zeros.
/// Prints them to stdout.
///
/// # Arguments
/// * `zeros_count` - the amount of consecutive trailing zeros in the hashes
/// * `max_results_count` - the amount of matching hashes to find and print
///
/// # Fails
/// If zeros_count is bigger than 64.
/// This is because the hash length is 64 symbols.
///
/// # Usage
/// ```
/// let result = multithread_hashing(zeros_count, max_results_count);
/// match result {
///     Ok(()) => ExitCode::SUCCESS,
///     Err(message) => {
///         eprintln!("{}", message);
///         ExitCode::FAILURE
///     }
/// }
/// ```
pub fn multithread_hashing(zeros_count: usize, max_results_count: u32) -> Result<(), &'static str> {
    if zeros_count > HASH_LENGTH {
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

/// Spawns a thread that hashes the number of the passed counter.
/// Every new thread increments the counter by one.
/// Any time a thread matches the passed amount of trailing zeros, `found_hashes` gets incremented, and the hash gets printed to stdout, along with the hashed counter on the left.
/// If we found `max_results_count` of hashes, the loop exits.
fn spawn_thread(
    counter: &Arc<Mutex<u32>>,
    found_hashes: &Arc<Mutex<u32>>,
    zeros_count: usize,
    max_results_count: u32,
) -> JoinHandle<()> {
    let counter = Arc::clone(counter);
    let found_hashes = Arc::clone(found_hashes);
    thread::spawn(move || loop {
        let index = {
            let mut index = counter.lock().unwrap();
            let old_index = *index;
            *index += 1;
            old_index
        };

        let hash = create_sha256(format!("{}", index));
        if hash.ends_with(&"0".repeat(zeros_count)) {
            {
                let mut num = found_hashes.lock().unwrap();
                *num += 1;
            }
            println!("{}, \"{}\"", index, hash);
        }

        if *found_hashes.lock().unwrap() >= max_results_count {
            break;
        }
    })
}
