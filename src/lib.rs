pub mod domain;
pub mod implementation;

use domain::config::{ExperimentConfig, FioConfig};
use domain::experiment::run_experiment;
use implementation::dm_delay_adapter::DmDelayAdapter;
use implementation::full_disk_adapter::FullDiskAdapter;
use implementation::read_only_adapter::ReadOnlyAdapter;

pub fn run_delay(device: String, dm_name: String, delay_ms: u64, runtime_sec: u64,) -> Result<(), String> {
	let cfg = ExperimentConfig {
		device: device.clone(),
		dm_name: dm_name.clone(),
		delay_ms,
		fio: FioConfig {
			runtime_sec,
		}
	};

	let adapter = DmDelayAdapter::new(device, dm_name, delay_ms);

	let summary = run_experiment(&cfg, Box::new(adapter))?;

	println!("{}", summary.render_terminal());

	Ok(())
}

pub fn run_full(device: String, runtime_sec: u64) -> Result<(), String> {
	let cfg = domain::config::ExperimentConfig {
		device: device.clone(),
		dm_name: "full_disk".to_string(),
		delay_ms: 0,
		fio: domain::config::FioConfig {
			runtime_sec,
		}
	};

	let adapter = FullDiskAdapter::new(device);

	let summary = domain::experiment::run_experiment(&cfg, Box::new(adapter))?;

	println!("\n=== RESULTS ===\n");
	println!("CONTROL	| IOPS: {:.2}, BW: {:.2} KIB/s, Latency: {:.2} ms",
		summary.control.iops, summary.control.bandwidth_kib, summary.control.latency_ms);

	if summary.chaos.iops == 0.0 {
		println!("CHAOS	| STATUS: WRITE FAILED (ENOSPC)");
	} else {
		println!("CHAOS		| IOPS: {:.2}, BW: {:.2} KIB/s, Latency: {:.2} ms",
			summary.control.iops, summary.control.bandwidth_kib, summary.control.latency_ms);

	}

	Ok(())
}

pub fn run_readonly(device: String, runtime_sec: u64) -> Result<(), String> {
	let cfg = domain::config::ExperimentConfig {
		device: device.clone(), dm_name: "readonly".to_string(), delay_ms: 0,
		fio: domain::config::FioConfig {
			runtime_sec
		}
	};

	let adapter = ReadOnlyAdapter::new(device);

	let summary = domain::experiment::run_experiment(&cfg, Box::new(adapter))?;

	println!("\n=== RESULTS ===\n");

	println!("CONTROL	| IOPS: {:.2}, BW: {:.2} KIB/s, Latency: {:.2} ms",
		summary.control.iops, summary.control.bandwidth_kib, summary.control.latency_ms);

	if summary.chaos.iops == 0.0 {
		println!("CHAOS	| STATUS: WRITE FAILED (READ-ONLY FILESYSTEM)");
	} else {
		println!("CHAOS	| IOPS: {:.2}, BW: {:.2} KIB/s, Latency: {:.2} ms",
			summary.chaos.iops, summary.chaos.bandwidth_kib, summary.chaos.latency_ms);
	}

	Ok(())
}
