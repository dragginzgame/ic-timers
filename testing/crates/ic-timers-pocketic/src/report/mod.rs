//! Host-only cohort receipts and review comparisons; never deployed in probes.

use ic_build::artifacts::WasmBuildRecord;
use ic_host_artifacts::artifact::{hash_reader, read_reader};
use ic_host_fs::durable::{PublicationMode, WriteOptions, write_bytes};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, error::Error, fs, path::Path};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const POLICY_METRICS: &[&str] = &[
    "start_instructions",
    "duplicate_instructions",
    "cancel_instructions",
    "snapshot_instructions",
    "inventory_instructions",
    "scheduler_instructions",
    "work_instructions",
    "memory_sampling_empty",
    "memory_sampling_sampled",
    "memory_sampling_overhead",
    "dispatch_cycles",
    "wasm_bytes",
];
const CALIBRATION_METRICS: &[&str] = &[
    "scheduler_instructions",
    "work_instructions",
    "dispatch_cycles",
];

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Report {
    schema: u32,
    context: BTreeMap<String, String>,
    artifacts: BTreeMap<String, BTreeMap<String, String>>,
    pub(crate) measurements: BTreeMap<String, BTreeMap<String, u128>>,
}

impl Report {
    pub(crate) fn new(root: &Path) -> Result<Self> {
        let server = std::env::var_os("POCKET_IC_BIN").ok_or("missing admitted POCKET_IC_BIN")?;
        Ok(Self {
            schema: 1,
            context: BTreeMap::from([
                ("lock_sha256".into(), digest(&root.join("Cargo.lock"))?),
                ("server_sha256".into(), digest(Path::new(&server))?),
                (
                    "host".into(),
                    format!("{}-{}", std::env::consts::ARCH, std::env::consts::OS),
                ),
                ("profile".into(), "timer-probe".into()),
            ]),
            artifacts: BTreeMap::new(),
            measurements: BTreeMap::new(),
        })
    }

    pub(crate) fn record_build(
        &mut self,
        cohort: &str,
        record: &WasmBuildRecord,
        artifact: &Path,
    ) -> Result<()> {
        let facts = record.facts();
        for (key, value) in [
            ("rustc", String::from_utf8(facts.rustc_identity.clone())?),
            ("cargo", String::from_utf8(facts.cargo_identity.clone())?),
            ("target", facts.target.clone()),
        ] {
            if let Some(previous) = self.context.insert(key.into(), value.clone())
                && previous != value
            {
                return Err(format!("cohort {key} changed during builds").into());
            }
        }
        self.artifacts.insert(
            cohort.into(),
            BTreeMap::from([
                ("wasm_sha256".into(), digest(artifact)?),
                ("build_fingerprint".into(), record.fingerprint().to_string()),
                ("input_digest".into(), record.input_digest().to_string()),
                ("feature".into(), cohort.into()),
            ]),
        );
        Ok(())
    }

    pub(crate) fn verify_context(&self, root: &Path) -> Result<()> {
        for (key, value) in Self::new(root)?.context {
            if self.context.get(&key) != Some(&value) {
                return Err(format!("qualification {key} changed during measurement").into());
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(crate) fn verify_artifact(&self, cohort: &str, wasm: &[u8]) {
        assert_eq!(
            self.artifacts[cohort]["wasm_sha256"],
            ic_host_artifacts::artifact::Sha256Digest::compute(wasm).to_string(),
            "measured artifact differs from its build receipt"
        );
    }

    /// Admit only a complete report from this invocation's child and build context.
    pub(crate) fn read_candidate(path: &Path, expected: &Self) -> Result<Self> {
        let candidate = Self::read(path)?;
        if candidate.schema != expected.schema
            || candidate.context != expected.context
            || candidate.artifacts != expected.artifacts
            || !candidate.complete_measurements()
        {
            return Err("incomplete cohort report or mismatched invocation build context".into());
        }
        Ok(candidate)
    }

    fn complete_measurements(&self) -> bool {
        self.measurements.len() == crate::build::COHORTS.len() + 2
            && self.measurements.iter().all(|(subject, metrics)| {
                let expected = if crate::build::COHORTS
                    .iter()
                    .any(|(name, _)| name == subject)
                {
                    POLICY_METRICS
                } else if matches!(
                    subject.as_str(),
                    "watchdog-calibration-minimal" | "watchdog-calibration-representative"
                ) {
                    CALIBRATION_METRICS
                } else {
                    return false;
                };
                metrics.len() == expected.len()
                    && expected.iter().all(|key| metrics.contains_key(*key))
            })
    }

    fn read(path: &Path) -> Result<Self> {
        Ok(serde_json::from_slice(&read_reader(
            fs::File::open(path)?,
            1024 * 1024,
        )?)?)
    }

    pub(crate) fn publish(&self, path: &Path) -> Result<()> {
        write_bytes(
            path,
            WriteOptions {
                mode: PublicationMode::Replace,
                permissions: 0o600,
            },
            &serde_json::to_vec_pretty(self)?,
        )?;
        println!("cohort receipt: {}", path.display());
        Ok(())
    }
}

pub(crate) fn digest(path: &Path) -> Result<String> {
    Ok(hash_reader(fs::File::open(path)?, 1024 * 1024 * 1024)?
        .sha256
        .to_string())
}

pub(crate) fn compare(current: &Path, baseline: &Path) -> Result<()> {
    let current = Report::read(current)?;
    let baseline = Report::read(baseline)?;
    let changes = current.deltas(&baseline)?;
    println!("cohort comparison (review evidence; positive deltas are increases):");
    for (subject, delta) in changes {
        println!("  {subject}: {delta:+}");
    }
    Ok(())
}

impl Report {
    fn deltas(&self, baseline: &Self) -> Result<BTreeMap<String, i128>> {
        if self.schema != 1 || baseline.schema != 1 || self.context != baseline.context {
            return Err("incomparable cohort schema/compiler/server/host/lock/profile/target; qualify a new baseline explicitly".into());
        }
        if self.artifacts.len() != 4
            || self.artifacts.keys().ne(baseline.artifacts.keys())
            || !self.complete_measurements()
            || !baseline.complete_measurements()
        {
            return Err("incomparable cohort subjects/artifacts".into());
        }
        let mut changes = BTreeMap::new();
        for (subject, metrics) in &self.measurements {
            let previous = &baseline.measurements[subject];
            for (metric, value) in metrics {
                changes.insert(
                    format!("{subject}/{metric}"),
                    i128::try_from(*value)?
                        .checked_sub(i128::try_from(previous[metric])?)
                        .ok_or("delta overflow")?,
                );
            }
        }
        Ok(changes)
    }
}

#[cfg(test)]
mod tests;
