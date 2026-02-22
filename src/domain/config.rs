pub struct ExperimentConfig {
	pub device: String,
	pub dm_name: String,
	pub delay_ms: u64,
	pub fio: FioConfig,
}

pub struct FioConfig {
	pub runtime_sec: u64,
}
