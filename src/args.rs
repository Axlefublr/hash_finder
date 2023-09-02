use clap::Parser;

#[derive(Parser)]
#[command(about, version)]
pub struct Args {
	/// The amount of consecutive zeros at the end of the hash
	#[arg(short = 'N', value_name = "ZEROS")]
	pub zeros_count: usize,
	/// The amount of results you want to print to stdout
	#[arg(short = 'F', value_name = "RESULTS")]
	pub max_results_count: u32,
}
