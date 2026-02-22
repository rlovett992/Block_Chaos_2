use std::process::Command;
use serde_json;

use crate::domain::results::{FioJson, FioSummary};
use crate::domain::traits::StorageAdapter;

pub struct DmDelayAdapter {
	device: String,
	dm_name: String,
	delay_ms: u64,
}

impl DmDelayAdapter {
	pub fn new(device: String, dm_name: String, delay_ms: u64) -> Self {
		Self {
			device,
			dm_name,
			delay_ms,
		}
	}

	fn run_cmd(mut cmd: Command) -> Result<String, String> {
		let output = cmd.output().map_err(|e| e.to_string())?;
		if !output.status.success() {
			return Err(String::from_utf8_lossy(&output.stderr).to_string());
		}
		Ok(String::from_utf8_lossy(&output.stdout).to_string())
	}
}

impl StorageAdapter for DmDelayAdapter {
	fn clean(&self) -> Result<(), String> {
		let _ = Command::new("sudo").args(["dmsetup", "remove", "-f", &self.dm_name]).output();

		let _ = Command::new("sudo").args(["dmsetup", "remove", "--retry", "-f", &self.dm_name]).output();

		let _ = Command::new("sudo").args(["udevamd", "settle"]).output();

		Ok(())
	}

	fn apply(&self) -> Result<(), String> {
		let _ = Command::new("sudo").args(["dmsetup", "remove", "-f", &self.dm_name]).output();

		let _ = Command::new("sudo").args(["dmsetup", "remove", "--retry", "-f", &self.dm_name]).output();

		let _ = Command::new("sudo").args(["udevamd", "settle"]).output();

		let mut blk = Command::new("sudo");
		blk.args(["blockdev", "--getsz", &self.device]);
		let sectors = Self::run_cmd(blk)?;

		let table = format!("0 {} delay {} 0 {}", sectors.trim(), self.device, self.delay_ms);

		let output = Command::new("sudo").args(["dmsetup", "create", &self.dm_name, "--table", &table]).output().map_err(|e| e.to_string())?;

		if !output.status.success() {
			return Err(String::from_utf8_lossy(&output.stderr).to_string());
		}

		let _ = Command::new("sudo").args(["udevadm", "settle"]).output();

		Ok(())
	}

	fn remove (&self) -> Result<(), String> {
		let _ = Command::new("sudo").args(["dmsetup", "remove", "-f", &self.dm_name]).output();

		let _ = Command::new("sudo").args(["dmsetup", "remove", "--retry", "-f", &self.dm_name]).output();

		let _ = Command::new("sudo").args(["udevadm", "settle"]).output();

		Ok(())
	}

	fn run_fio(&self, device_path: &str, runtime_sec: u64) -> Result<FioSummary, String> {
		let mut cmd = Command::new("sudo");

		cmd.args(["fio", "--output-format=json", "--name=job",
			&format!("--filename={}", device_path), "--rw=randread", "--bs=4k", "--size=512M", "--numjobs=1", "--time_based",
			&format!("--runtime={}", runtime_sec),]);

		let out = Self::run_cmd(cmd)?;

		let parsed: FioJson = serde_json::from_str(&out).map_err(|e| e.to_string())?;

		FioSummary::try_from(parsed)
	}

	fn chaos_device_path(&self) -> String {
		format!("/dev/mapper/{}", self.dm_name)
	}

	fn apply_label(&self) -> &'static str {
		"Applying delay..."
	}

	fn remove_label(&self) -> &'static str {
		"Removing delay..."
	}
}
