use args::Args;
use clap::Parser;
use std::process::ExitCode;

mod args;
mod hash;

const HASH_LENGTH: usize = 64;

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
