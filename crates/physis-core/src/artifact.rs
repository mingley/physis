//! Content-addressed artifact identifiers.
//!
//! Same canonical bytes ⇒ same [`ArtifactId`]. Changing a statement, a unit,
//! an assumption, or a constant must produce a new id. These hashes are the
//! substrate for Level-3 receipts; they are not a proof of physics.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};

/// 32-byte SHA-256 content address.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ArtifactId(pub [u8; 32]);

impl ArtifactId {
    /// Hash canonical bytes.
    pub fn of(bytes: impl AsRef<[u8]>) -> Self {
        let mut h = Sha256::new();
        h.update(bytes.as_ref());
        let digest = h.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(&digest);
        Self(out)
    }

    /// Lower-case hex.
    pub fn to_hex(self) -> String {
        self.0.iter().map(|b| format!("{b:02x}")).collect()
    }

    /// Parse 64 hex characters.
    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 64 {
            return None;
        }
        let mut out = [0u8; 32];
        for (i, chunk) in s.as_bytes().chunks(2).enumerate() {
            let a = from_hex_digit(chunk[0])?;
            let b = from_hex_digit(chunk[1])?;
            out[i] = (a << 4) | b;
        }
        Some(Self(out))
    }
}

fn from_hex_digit(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

impl fmt::Debug for ArtifactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ArtifactId({})", self.to_hex())
    }
}

impl fmt::Display for ArtifactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_hex())
    }
}

impl Serialize for ArtifactId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for ArtifactId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::from_hex(&s).ok_or_else(|| serde::de::Error::custom("expected 64 hex chars"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_deterministic_and_sensitive() {
        let a = ArtifactId::of(b"forall x");
        let b = ArtifactId::of(b"forall x");
        let c = ArtifactId::of(b"exists x");
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert_eq!(a.to_hex().len(), 64);
        assert_eq!(ArtifactId::from_hex(&a.to_hex()), Some(a));
    }
}
