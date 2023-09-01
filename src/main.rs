use std::process::ExitCode;
use args::Args;
use clap::Parser;

mod args;
mod hash;

const MAX_ZEROS: usize = 64;

fn main() -> ExitCode {
	let args = Args::parse();
	let result = hash::multithread_hashing(args.zeros_count, args.max_results_count);
	match result {
		Ok(()) => ExitCode::SUCCESS,
		Err(message) => {
			eprintln!("{}", message);
			ExitCode::FAILURE
		}
	}
}