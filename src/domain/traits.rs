use super::results::FioSummary;

pub trait StorageAdapter {
	fn clean(&self) -> Result<(), String>;
	fn apply(&self) -> Result<(), String>;
	fn remove(&self) -> Result<(), String>;

	fn run_fio(&self, device_path: &str, runtime_sec: u64) -> Result<FioSummary, String>;

	fn chaos_device_path(&self) -> String;

	fn apply_label(&self) -> &'static str;
	fn remove_label(&self) -> &'static str;
}
