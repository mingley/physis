//! Content-addressed artifact DAG. Same inputs ⇒ same id. Changing one
//! node invalidates only descendants.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::collections::{BTreeMap, BTreeSet};

use physis_core::artifact::ArtifactId;
use serde::{Deserialize, Serialize};

/// Kind of stored object.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NodeKind {
    /// A theory encoding.
    Theory,
    /// A claim.
    Claim,
    /// A statement hash.
    Statement,
    /// An assumption set.
    AssumptionSet,
    /// A dataset.
    Dataset,
    /// A proof artifact.
    Proof,
    /// A calculation.
    Calculation,
    /// A source record.
    Source,
    /// An experiment.
    Experiment,
    /// A lab state snapshot.
    LabState,
    /// An agent action.
    AgentAction,
    /// A verification receipt.
    VerificationReceipt,
    /// A semantic-review record (provenance + independent encoding + audit).
    SemanticReview,
    /// A rebuilt knowledge-gap snapshot (not deserialized as authority).
    KnowledgeGap,
}

/// One DAG node.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Node {
    /// Content address of payload + kind + parent ids.
    pub id: ArtifactId,
    /// Kind.
    pub kind: NodeKind,
    /// Direct dependencies.
    pub parents: Vec<ArtifactId>,
    /// Canonical payload bytes.
    pub payload: Vec<u8>,
}

impl Node {
    /// Content-address a node.
    pub fn new(kind: NodeKind, parents: Vec<ArtifactId>, payload: impl AsRef<[u8]>) -> Self {
        let payload = payload.as_ref().to_vec();
        let mut buf = Vec::new();
        buf.extend_from_slice(format!("{kind:?}").as_bytes());
        buf.push(b'\n');
        for p in &parents {
            buf.extend_from_slice(p.to_hex().as_bytes());
            buf.push(b'\n');
        }
        buf.extend_from_slice(&payload);
        Self {
            id: ArtifactId::of(&buf),
            kind,
            parents,
            payload,
        }
    }
}

/// Content-addressed store with reverse edges for invalidation.
#[derive(Clone, Debug, Default)]
pub struct ArtifactStore {
    nodes: BTreeMap<ArtifactId, Node>,
    children: BTreeMap<ArtifactId, BTreeSet<ArtifactId>>,
}

impl ArtifactStore {
    /// Empty store.
    pub fn empty() -> Self {
        Self::default()
    }

    /// Insert a node. Returns its id.
    pub fn insert(&mut self, node: Node) -> ArtifactId {
        let id = node.id;
        for p in &node.parents {
            self.children.entry(*p).or_default().insert(id);
        }
        self.nodes.insert(id, node);
        id
    }

    /// Lookup.
    pub fn get(&self, id: ArtifactId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Transitive descendants of `id`, not including `id`.
    pub fn descendants(&self, id: ArtifactId) -> BTreeSet<ArtifactId> {
        let mut out = BTreeSet::new();
        let mut stack = vec![id];
        while let Some(n) = stack.pop() {
            if let Some(ch) = self.children.get(&n) {
                for c in ch {
                    if out.insert(*c) {
                        stack.push(*c);
                    }
                }
            }
        }
        out
    }

    /// Nodes that remain valid if `id` changes: everything that is not `id`
    /// and not a descendant of `id`.
    pub fn preserved_if_changed(&self, id: ArtifactId) -> BTreeSet<ArtifactId> {
        let hit = self.descendants(id);
        self.nodes
            .keys()
            .copied()
            .filter(|n| *n != id && !hit.contains(n))
            .collect()
    }

    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Empty?
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changing_one_input_invalidates_only_descendants() {
        let mut g = ArtifactStore::empty();
        let c = g.insert(Node::new(NodeKind::Source, vec![], b"codata"));
        let coup = g.insert(Node::new(NodeKind::Calculation, vec![c], b"coupling"));
        let rg = g.insert(Node::new(NodeKind::Calculation, vec![coup], b"rg"));
        let pred = g.insert(Node::new(NodeKind::Claim, vec![rg], b"gut-pred"));
        let other = g.insert(Node::new(NodeKind::Proof, vec![], b"unrelated-proof"));

        let hit = g.descendants(c);
        assert!(hit.contains(&coup) && hit.contains(&rg) && hit.contains(&pred));
        assert!(!hit.contains(&other));
        let kept = g.preserved_if_changed(c);
        assert!(kept.contains(&other));
        assert!(!kept.contains(&pred));
        assert_eq!(g.len(), 5);
    }

    #[test]
    fn same_payload_same_id() {
        let a = Node::new(NodeKind::Claim, vec![], b"hello");
        let b = Node::new(NodeKind::Claim, vec![], b"hello");
        assert_eq!(a.id, b.id);
    }
}
