use std::{collections::BTreeMap, path::PathBuf};

use jailgun_core::JailgunConfig;

#[derive(Debug, Clone)]
pub struct RunOptions {
    pub run_id: String,
    pub config: JailgunConfig,
    pub prompt_text: String,
    pub tabs_override: Option<u16>,
    pub no_deploy: bool,
    pub dry_run: bool,
    pub profile_dir: PathBuf,
    pub profile_pool: Vec<PathBuf>,
    pub tab_profile_dirs: BTreeMap<u16, PathBuf>,
    pub downloads_dir: PathBuf,
    pub artifacts_dir: PathBuf,
    pub bridge_cmd: Vec<String>,
    pub bridge_env: BTreeMap<String, String>,
    pub repo_url: String,
    pub local_archive_path: Option<PathBuf>,
    pub deploy_remote_host: Option<String>,
    pub deploy_remote_dir: Option<String>,
    pub deploy_remote_command: Option<String>,
    pub deploy_expected_top_level: Option<String>,
    pub ci_tracker_enabled: bool,
    pub ci_repo: Option<String>,
    pub ci_branch: String,
    pub ci_max_attempts: u32,
    pub ci_poll_seconds: u16,
    pub status_max_minutes: u16,
    pub max_runtime_seconds: Option<u64>,
    pub event_buffer: usize,
    pub deploy_concurrency: u16,
}

impl RunOptions {
    pub fn tabs(&self) -> u16 {
        self.tabs_override.unwrap_or(self.config.browser.tabs)
    }

    /// The spawn cardinality for the jekko-web diagram: one box per browser tab
    /// (sub-agent). Mirrors the flowgraph `spawn` node.
    pub fn spawn_cardinality(&self) -> SpawnCardinality {
        SpawnCardinality::for_tabs(self.tabs())
    }
}

/// The spawn shape of a jailgun run — how many sub-agent (browser-tab) boxes it
/// expands into. The runner parses `receipt_value()` into the flowgraph `spawn`
/// node's cardinality so the diagram renders one box per tab.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpawnCardinality {
    pub runtime: &'static str,
    pub tabs: u16,
}

impl SpawnCardinality {
    pub fn for_tabs(tabs: u16) -> Self {
        Self {
            runtime: "jailgun",
            tabs,
        }
    }

    /// Compact `key:val,…` value embedded in run metadata.
    pub fn receipt_value(&self) -> String {
        format!("runtime:{},tabs:{}", self.runtime, self.tabs)
    }
}

#[cfg(test)]
mod tests {
    use super::SpawnCardinality;

    #[test]
    fn spawn_cardinality_reports_tab_count() {
        let card = SpawnCardinality::for_tabs(5);
        assert_eq!(card.runtime, "jailgun");
        assert_eq!(card.tabs, 5);
        assert_eq!(card.receipt_value(), "runtime:jailgun,tabs:5");
    }
}
