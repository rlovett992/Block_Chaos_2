use std::process::Command;
use std::fs;

use crate::domain::results::{FioSummary, FioJson};
use crate::domain::traits::StorageAdapter;

pub struct FullDiskAdapter {
	device: String,
	mount_point: String,
}

struct MountRollBack<'a> {
	mount_point: &'a str,
	active: bool,
}

impl<'a> MountRollBack<'a> {
	fn new(mount_point: &'a str) -> Self {
		Self {mount_point, active: true }
	}

	fn disarm(&mut self) {
		self.active = false;
	}
}

impl Drop for MountRollBack<'_> {
	fn drop(&mut self) {
		if !self.active {
			return
		}

		let _ = Command::new("sudo").args(["mount", "-o", "remount,rw", self.mount_point]).output();
		let _ = Command::new("sudo").args(["umount", self.mount_point]).output();
	}
}

impl FullDiskAdapter {
	pub fn new(device: String) -> Self {
		Self {
			device,
			mount_point: format!("{}/block_chaos_full", std::env::var("HOME").unwrap()),
		}
	}
}

impl StorageAdapter for FullDiskAdapter {
	fn clean(&self) -> Result<(), String> {
		let _ = Command::new("sudo").args(["mount", "-o", "remount,rw", &self.mount_point]).output();

		let _ = Command::new("sudo").args(["umount", &self.mount_point]).output();

		let _ = fs::remove_dir_all(&self.mount_point);

		Ok(())
	}

	fn apply(&self) -> Result<(), String> {
		fs::create_dir_all(&self.mount_point).map_err(|e| e.to_string())?;

		let mut rollback = MountRollBack::new(&self.mount_point);

		let out = Command::new("sudo").args(["mount", &self.device, &self.mount_point]).output().map_err(|e| e.to_string())?;

		if !out.status.success() {
			return Err(String::from_utf8_lossy(&out.stderr).to_string());
		}

		let filler_path = format!("{}/filler.bin", self.mount_point);

		let out = Command::new("sudo").args(["dd", "if=/dev/zero", &format!("of={}", filler_path),
			"bs=1M"]).output().map_err(|e| e.to_string())?;

		let _ = out;

		rollback.disarm();
		Ok(())
	}

	fn remove(&self) -> Result<(), String> {
		let filler = format!("{}/filler.bin", self.mount_point);

		let _ = Command::new("sudo").args(["rm", "-f", &filler]).output();

		let _ = Command::new("sudo").args(["mount", "-o", "remount,rw", &self.mount_point]).output();

		let _ = Command::new("sudo").args(["umount", &self.mount_point]).output();

		Ok(())
	}

	fn run_fio(&self, directory: &str, runtime_sec: u64) -> Result<FioSummary, String> {
		let mut cmd = Command::new("sudo");

		cmd.args(["fio", "--output-format=json", "--name=job", &format!("--directory={}", directory), "--rw=write",
			"--bs=4k", "--size=512M", "--numjobs=1", "--time_based", &format!("--runtime={}", runtime_sec)]);

		let output = cmd.output().map_err(|e| e.to_string())?;

		if !output.status.success() {
			return Ok(FioSummary {
				iops: 0.0,
				bandwidth_kib: 0.0,
				latency_ms: 0.0,
			});
		}

		let parsed: FioJson = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

		let summary = FioSummary::try_from(parsed)?;

		Ok(summary)
	}

	fn chaos_device_path(&self) -> String {
		self.mount_point.clone()
	}

	fn apply_label(&self) -> &'static str {
		"Applying full disk exhaustion..."
	}

	fn remove_label(&self) -> &'static str {
		"Removing full disk condition..."
	}
}
