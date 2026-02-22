use serde::Deserialize;

#[derive(Debug)]
pub struct FioSummary {
	pub iops: f64,
	pub bandwidth_kib: f64,
	pub latency_ms: f64,
}

pub struct ExperimentSummary {
	pub control: FioSummary,
	pub chaos: FioSummary,
}

impl ExperimentSummary {
	pub fn render_terminal(&self) -> String {
		format!(
			"\n=== RESULTS ===\n\
			CONTROL	| IOPS: {:.2}, BW: {:.2} Kib/s, Latency: {:.2} ms\n\
			CHAOS	| IOPS: {:.2}, BW: {:.2} Kib/s, Latency: {:.2} ms\n",
			self.control.iops,
			self.control.bandwidth_kib,
			self.control.latency_ms,
			self.chaos.iops,
			self.chaos.bandwidth_kib,
			self.chaos.latency_ms
		)
	}
}

#[derive(Deserialize)]
pub struct FioJson {
	pub jobs: Vec<FioJob>,
}

#[derive(Deserialize)]
pub struct FioJob {
        pub read: FioRead,
}

#[derive(Deserialize)]
pub struct FioRead {
        pub iops: f64,
	pub bw: f64,
	pub clat_ns: FioLatency,
}

#[derive(Deserialize)]
pub struct FioLatency {
        pub mean: f64,
}

impl TryFrom<FioJson> for FioSummary {
	type Error = String;

	fn try_from(value: FioJson) -> Result<Self, Self::Error> {
		let job = value.jobs.first().ok_or("No fio job output")?;

		Ok(FioSummary {
			iops: job.read.iops,
			bandwidth_kib: job.read.bw,
			latency_ms: job.read.clat_ns.mean / 1_000_000.0,
		})
	}
}
