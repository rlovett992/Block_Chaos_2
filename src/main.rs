use clap::{Parser, Subcommand};
use block_chaos::run_delay;

#[derive(Parser, Debug)]
#[command(name = "blockio")]
#[command(about = "Block I/O Chaos Controller")]

struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
	//validation for device and env safety
	Validate {
		#[arg(long)]
		device: String,
	},

	//inject dm-delay chaos
	Delay {
		#[arg(long)]
		device: String,
		#[arg(long)]
		name: String,
		#[arg(long)]
		delay: u64,
		#[arg(long)]
		runtime: u64,

	},

	//inject the full disk chaos
	Full {
		#[arg(long)]
		device: String,
		#[arg(long)]
		runtime: u64,
	},

	//inject the rad-only remount
	Readonly {
		#[arg(long)]
		device: String,
		#[arg(long)]
		runtime: u64,
	},

	//remove the chaos
	Cleanup {
		#[arg(long)]
		name: String,
	},
}

fn main() {
	let cli = Cli::parse();

	let result = match cli.command {
		Commands::Validate { device } => {
			println!("Validation not implemented yet for {device}");
			Ok(())
		}

		Commands::Delay { device, name, delay, runtime } => {
			run_delay(device, name, delay, runtime)
		}

		Commands::Full { device, runtime } => {
			block_chaos::run_full(device, runtime)
		}

		Commands::Readonly { device, runtime } => {
			block_chaos::run_readonly(device, runtime)
		}

		Commands::Cleanup { name } => { 
			println!("Manual cleanup recommended: ./scripts/cleanup.sh {name}");
			Ok(())
		}
	};

	if let Err(e) = result {
		eprintln!("\nERROR: {e}");
		std::process::exit(1);
	}
}
