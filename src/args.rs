use clap::Parser;

#[derive(Parser)]
#[command(author, about, next_line_help = true)]
pub struct Args {
	/// The amount of consecutive zeros at the end of the hash
	#[arg(short = 'N')]
	pub zeros_count: usize,
	/// The amount of results you want to print to stdout
	#[arg(short = 'F')]
	pub max_results_count: u32,
}