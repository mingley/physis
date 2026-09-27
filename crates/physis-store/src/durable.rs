//! Durable content-addressed blob store (R2 substrate).
//!
//! Each object is raw bytes filed under `ArtifactId::of(bytes)`. Writes are
//! atomic (temp file plus fsync plus rename) so a crash leaves either the
//! old state or the complete new object, never a torn blob. Reads rehash
//! and refuse on mismatch: loading bytes never restores trust in them.
//! There is no delete and no garbage collection in v1; the store is
//! append-only.

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use physis_core::artifact::ArtifactId;

/// What can go wrong in the durable store.
#[derive(Debug)]
pub enum DurableError {
    /// Filesystem failure.
    Io(std::io::Error),
    /// No object with this id.
    Missing {
        /// Requested id.
        id: ArtifactId,
    },
    /// Bytes on disk do not hash to the requested id.
    Corrupt {
        /// Requested id.
        id: ArtifactId,
    },
}

impl fmt::Display for DurableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurableError::Io(e) => write!(f, "object store io: {e}"),
            DurableError::Missing { id } => write!(f, "missing object {}", id.to_hex()),
            DurableError::Corrupt { id } => {
                write!(
                    f,
                    "corrupt object {}: content does not match id",
                    id.to_hex()
                )
            }
        }
    }
}

impl std::error::Error for DurableError {}

impl From<std::io::Error> for DurableError {
    fn from(e: std::io::Error) -> Self {
        DurableError::Io(e)
    }
}

/// Append-only directory of content-addressed blobs.
#[derive(Debug)]
pub struct DurableStore {
    dir: PathBuf,
    tmp_ctr: AtomicU64,
}

impl DurableStore {
    /// Open (creating) the objects directory.
    pub fn open(dir: impl AsRef<Path>) -> std::io::Result<Self> {
        let dir = dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&dir)?;
        Ok(Self {
            dir,
            tmp_ctr: AtomicU64::new(0),
        })
    }

    /// Store bytes, returning their content id. Same bytes give the same
    /// id; storing twice is a no-op, not an error. The write is atomic
    /// (temp file plus fsync plus rename), so a crash leaves either the
    /// old state or the complete new object. A file already sitting at
    /// the id with different bytes is corruption, not an overwrite.
    pub fn put(&self, bytes: impl AsRef<[u8]>) -> Result<ArtifactId, DurableError> {
        let bytes = bytes.as_ref();
        let id = ArtifactId::of(bytes);
        let path = self.dir.join(id.to_hex());
        if path.exists() {
            let current = std::fs::read(&path)?;
            if current == bytes {
                return Ok(id);
            }
            return Err(DurableError::Corrupt { id });
        }
        let n = self.tmp_ctr.fetch_add(1, Ordering::Relaxed);
        let tmp = self.dir.join(format!(".tmp-{}-{n}", std::process::id()));
        {
            let mut f = File::create(&tmp)?;
            f.write_all(bytes)?;
            f.sync_all()?;
        }
        std::fs::rename(&tmp, &path)?;
        File::open(&self.dir)?.sync_all()?;
        Ok(id)
    }

    /// Read and rehash-verify an object. Missing and corrupt are distinct
    /// errors; neither is ever silently tolerated.
    pub fn get(&self, id: ArtifactId) -> Result<Vec<u8>, DurableError> {
        let bytes = std::fs::read(self.dir.join(id.to_hex())).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                DurableError::Missing { id }
            } else {
                DurableError::Io(e)
            }
        })?;
        if ArtifactId::of(&bytes) != id {
            return Err(DurableError::Corrupt { id });
        }
        Ok(bytes)
    }

    /// Whether an object file exists (no integrity check; use [`DurableStore::get`]
    /// when the bytes matter).
    pub fn contains(&self, id: ArtifactId) -> bool {
        self.dir.join(id.to_hex()).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("physis-r2-store-{}-{name}", std::process::id()))
    }

    fn cleanup(dir: &Path) {
        std::fs::remove_dir_all(dir).ok();
    }

    #[test]
    fn put_get_roundtrip() {
        let dir = scratch("roundtrip");
        cleanup(&dir);
        let store = DurableStore::open(&dir).unwrap();
        let id = store.put(b"raw-evidence-bytes").unwrap();
        assert_eq!(store.get(id).unwrap(), b"raw-evidence-bytes");
        cleanup(&dir);
    }

    #[test]
    fn same_bytes_same_id() {
        let dir = scratch("sameid");
        cleanup(&dir);
        let store = DurableStore::open(&dir).unwrap();
        let a = store.put(b"dup").unwrap();
        let b = store.put(b"dup").unwrap();
        assert_eq!(a, b);
        cleanup(&dir);
    }

    #[test]
    fn missing_object_is_error() {
        let dir = scratch("missing");
        cleanup(&dir);
        let store = DurableStore::open(&dir).unwrap();
        let err = store.get(ArtifactId::of(b"never-stored")).unwrap_err();
        assert!(err.to_string().contains("missing"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn tampered_object_is_detected() {
        let dir = scratch("tamper");
        cleanup(&dir);
        let store = DurableStore::open(&dir).unwrap();
        let id = store.put(b"pristine").unwrap();
        std::fs::write(dir.join(id.to_hex()), b"forged").unwrap();
        let err = store.get(id).unwrap_err();
        assert!(err.to_string().contains("corrupt"), "{err}");
        cleanup(&dir);
    }

    #[test]
    fn empty_bytes_roundtrip() {
        let dir = scratch("empty");
        cleanup(&dir);
        let store = DurableStore::open(&dir).unwrap();
        let id = store.put(b"").unwrap();
        assert!(store.get(id).unwrap().is_empty());
        cleanup(&dir);
    }
}
