use super::config::ExperimentConfig;
use super::results::ExperimentSummary;
use super::traits::StorageAdapter;

pub fn run_experiment(cfg: &ExperimentConfig, adapter: Box<dyn StorageAdapter>) -> Result<ExperimentSummary, String> {
	println!("Running control...");
	adapter.clean()?;
	let control = adapter.run_fio(&cfg.device, cfg.fio.runtime_sec)?;

	println!("{}", adapter.apply_label());
	adapter.apply()?;

	let chaos_device = adapter.chaos_device_path();

	println!("Running chaos...");
	let chaos = adapter.run_fio(&chaos_device, cfg.fio.runtime_sec)?;

	println!("{}", adapter.remove_label());
	adapter.remove()?;

	Ok(ExperimentSummary { control, chaos })
}
