use args::Args;
use clap::Parser;

mod args;
mod hash;

fn main() {
	let args = Args::parse();
	hash::multithread_hashing(args.zeros_count, args.max_results_count);
}