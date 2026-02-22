use std::process::Command;
use std::fs;

use crate::domain::results::{FioSummary, FioJson};
use crate::domain::traits::StorageAdapter;

pub struct ReadOnlyAdapter {
	device: String,
	mount_point: String,
}

impl ReadOnlyAdapter {
	pub fn new(device: String) -> Self {
		Self {
			device,
			mount_point: format!("{}/block_chaos_ro", std::env::var("HOME").unwrap()),
		}
	}
}

impl StorageAdapter for ReadOnlyAdapter {
	fn clean(&self) -> Result<(), String> {
		let _ = Command::new("sudo").args(["mount", "-o", "remount,rw", &self.mount_point]).output();

		let _ = Command::new("sudo").args(["umount", &self.mount_point]).output();

		let _ = fs::remove_dir_all(&self.mount_point);

		Ok(())
	}

	fn apply(&self) -> Result<(), String> {
		fs::create_dir_all(&self.mount_point).map_err(|e| e.to_string())?;

		let mount_out = Command::new("sudo").args(["mount", &self.device, &self.mount_point])
			.output().map_err(|e| e.to_string())?;

		if !mount_out.status.success() {
			return Err(String::from_utf8_lossy(&mount_out.stderr).to_string());
		}

		let remount_out = Command::new("sudo").args(["mount", "-o", "remount,ro", &self.mount_point])
			.output().map_err(|e| e.to_string())?;

		if !remount_out.status.success() {
			let _ = Command::new("sudo").args(["umount", &self.mount_point]).output();

			return Err(String::from_utf8_lossy(&remount_out.stderr).to_string());
		}

		Ok(())
	}

	fn remove(&self) -> Result<(), String> {
		let _ = Command::new("sudo").args(["mount", "-o", "remount,rw", &self.mount_point]).output();

		let _ = Command::new("sudo").args(["umount", &self.mount_point]).output();

		Ok(())
	}

	fn run_fio(&self, directory: &str, runtime_sec: u64) -> Result<FioSummary, String> {
		let mut cmd = Command::new("sudo");
		cmd.args(["fio", "--output-format=json", "--name=job",
			&format!("--directory={}", directory), "--rw=write", "--bs=4k", "--size=512M", "--numjobs=1",
			"--time_based", &format!("--runtime={}", runtime_sec)]);

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
		"Applying read-only remount..."
	}

	fn remove_label(&self) ->&'static str {
		"Removing read-only remount..."
	}
}
