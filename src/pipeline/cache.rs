use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::pipeline::stage::{Stage, StageKind};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
struct FileFingerprint {
    path: String,
    size: u64,
    mtime: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct StageManifest {
    inputs: Vec<FileFingerprint>,
    outputs: Vec<FileFingerprint>,
}

impl StageKind {
    fn file_name(&self) -> &'static str {
        match self {
            StageKind::Ingest => "ingest",
            StageKind::Analysis => "analysis",
            StageKind::Motion => "motion",
            StageKind::Render => "render",
            StageKind::Composite => "composite",
            StageKind::Review => "review",
        }
    }
}

fn cache_path(kind: StageKind) -> String {
    format!(".cache/{}.json", kind.file_name())
}

fn should_include(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| !name.starts_with('.'))
        .unwrap_or(false)
}

fn walk_fingerprints(paths: &[String]) -> Vec<FileFingerprint> {
    let mut fingerprints: Vec<FileFingerprint> = Vec::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if !path.exists() {
            continue;
        }
        if path.is_dir() {
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file() && should_include(e))
            {
                if let Ok(metadata) = entry.metadata() {
                    fingerprints.push(FileFingerprint {
                        path: entry.path().to_string_lossy().into_owned(),
                        size: metadata.len(),
                        mtime: metadata
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                            .map(|d| d.as_secs())
                            .unwrap_or(0),
                    });
                }
            }
        } else if path.is_file()
            && should_include_simple(path)
            && let Ok(metadata) = path.metadata()
        {
            fingerprints.push(FileFingerprint {
                path: path.to_string_lossy().into_owned(),
                size: metadata.len(),
                mtime: metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
            });
        }
    }

    fingerprints.sort_by(|a, b| a.path.cmp(&b.path));
    fingerprints
}

fn should_include_simple(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| !name.starts_with('.'))
        .unwrap_or(false)
}

fn load_manifest(kind: StageKind) -> Option<StageManifest> {
    let path = cache_path(kind);
    let data = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&data).ok()
}

fn write_manifest(kind: StageKind, manifest: &StageManifest) -> anyhow::Result<()> {
    let path = cache_path(kind);
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(manifest)?;
    fs::write(&path, json)?;
    Ok(())
}

pub fn needs_run(stage: &dyn Stage) -> bool {
    let kind = stage.kind();

    let manifest = match load_manifest(kind) {
        Some(m) => m,
        None => return true,
    };

    let current_inputs = walk_fingerprints(&stage.requires());
    if current_inputs != manifest.inputs {
        return true;
    }

    for fp in &manifest.outputs {
        if !Path::new(&fp.path).exists() {
            return true;
        }
    }

    false
}

pub fn save_manifest(stage: &dyn Stage) -> anyhow::Result<()> {
    let manifest = StageManifest {
        inputs: walk_fingerprints(&stage.requires()),
        outputs: walk_fingerprints(&stage.produces()),
    };
    if manifest.outputs.is_empty() {
        return Ok(());
    }
    write_manifest(stage.kind(), &manifest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::time::Duration;
    use crate::pipeline::stage::StageKind;

    struct MockStage {
        kind: StageKind,
        requires: Vec<String>,
        produces: Vec<String>,
    }

    impl Stage for MockStage {
        fn kind(&self) -> StageKind {
            self.kind
        }
        fn requires(&self) -> Vec<String> {
            self.requires.clone()
        }
        fn produces(&self) -> Vec<String> {
            self.produces.clone()
        }
        fn run(&self) -> anyhow::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn no_manifest_needs_run() {
        let stage = MockStage {
            kind: StageKind::Ingest,
            requires: vec![],
            produces: vec![],
        };
        assert!(needs_run(&stage));
    }

    #[test]
    fn manifest_inputs_changed_needs_run() {
        let dir = "test-cache-manifest-changed";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        let inp = format!("{}/input", dir);
        let out = format!("{}/output", dir);

        fs::write(&inp, "old").unwrap();
        fs::write(&out, "output-data").unwrap();

        // save initial manifest
        let init = MockStage {
            kind: StageKind::Ingest,
            requires: vec![inp.clone()],
            produces: vec![out.clone()],
        };
        save_manifest(&init).unwrap();

        // change input — use different size to guarantee fingerprint differs
        thread::sleep(Duration::from_secs(1));
        fs::write(&inp, "new data").unwrap();

        assert!(needs_run(&init));

        let _ = fs::remove_dir_all(".cache");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn manifest_unchanged_skips() {
        let dir = "test-cache-manifest-skip";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        let inp = format!("{}/input", dir);
        let out = format!("{}/output", dir);

        fs::write(&inp, "data").unwrap();
        fs::write(&out, "output-data").unwrap();

        let stage = MockStage {
            kind: StageKind::Composite,
            requires: vec![inp.clone()],
            produces: vec![out.clone()],
        };
        save_manifest(&stage).unwrap();

        assert!(!needs_run(&stage));

        let _ = fs::remove_dir_all(".cache");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn output_missing_needs_run() {
        let dir = "test-cache-output-missing";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        let inp = format!("{}/input", dir);
        let out = format!("{}/output", dir);

        fs::write(&inp, "data").unwrap();
        fs::write(&out, "output-data").unwrap();

        let stage = MockStage {
            kind: StageKind::Motion,
            requires: vec![inp.clone()],
            produces: vec![out.clone()],
        };
        save_manifest(&stage).unwrap();

        // remove output file
        fs::remove_file(&out).unwrap();

        assert!(needs_run(&stage));

        let _ = fs::remove_dir_all(".cache");
        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn walk_skips_hidden_files() {
        let dir = "test-cache-hidden";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();
        fs::write(format!("{}/visible.txt", dir), "x").unwrap();
        fs::write(format!("{}/.hidden", dir), "x").unwrap();

        let fingerprints = walk_fingerprints(&[dir.to_string()]);
        let paths: Vec<&str> = fingerprints.iter().map(|f| f.path.as_str()).collect();

        assert!(paths.iter().any(|p| p.contains("visible.txt")));
        assert!(!paths.iter().any(|p| p.contains(".hidden")));

        let _ = fs::remove_dir_all(dir);
    }

    #[test]
    fn walk_skips_missing_paths() {
        let fingerprints = walk_fingerprints(&["nonexistent-dir".to_string()]);
        assert!(fingerprints.is_empty());
    }

    #[test]
    fn save_manifest_empty_outputs_does_nothing() {
        let dir = "test-cache-empty-save";
        let _ = fs::remove_dir_all(dir);
        fs::create_dir_all(dir).unwrap();

        let stage = MockStage {
            kind: StageKind::Render,
            requires: vec![dir.to_string()],
            produces: vec![],
        };
        save_manifest(&stage).unwrap();

        assert!(load_manifest(StageKind::Ingest).is_none());

        let _ = fs::remove_dir_all(".cache");
        let _ = fs::remove_dir_all(dir);
    }
}
