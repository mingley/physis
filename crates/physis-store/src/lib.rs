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
    /// A live evaluator snapshot of one FormalClaim on one theory.
    /// Not a kernel proof and not a numeric certificate.
    Evaluation,
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
    /// A rebuilt evidence graph: competing encodings and evaluations of
    /// one lab slug. Not deserialized as authority; never Canonical or P4.
    Evidence,
    /// An independent Ratio parse of a `CertifiedNumeric` enclosure.
    /// Not a kernel receipt, not Canonical, and not P4. Restore rebuilds
    /// from live overlay strings; a recorded hash is not deserialized.
    NumericCertificate,
    /// An independent parse / round-trip / reconstruct of a live theory
    /// IR package. Not P3S, not a kernel receipt, not Canonical, and not
    /// P4. Restore rebuilds from the live package; a recorded hash is
    /// not deserialized.
    EncodingPackage,
    /// An independent `Judgment::from_lab` projection of live evaluator
    /// axes and receipts. Not deserialized as authority; never Canonical
    /// or P4. Restore rebuilds from live `from_lab`; a recorded hash is
    /// not deserialized. JSON cannot mint `logical proved`.
    JudgmentProjection,
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

    /// All stored nodes, ordered by id.
    pub fn iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
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

    #[test]
    fn evidence_root_invalidates_when_an_evaluation_changes() {
        let mut g = ArtifactStore::empty();
        let s = g.insert(Node::new(NodeKind::Statement, vec![], b"hash-a"));
        let e1 = g.insert(Node::new(NodeKind::Evaluation, vec![s], b"theory-a\tholds"));
        let e2 = g.insert(Node::new(NodeKind::Evaluation, vec![s], b"theory-b\tfails"));
        let mut parents = vec![e1, e2];
        parents.sort();
        let graph = g.insert(Node::new(NodeKind::Evidence, parents, b"slug"));

        assert!(g.descendants(s).contains(&e1));
        assert!(g.descendants(s).contains(&graph));

        let e1b = g.insert(Node::new(NodeKind::Evaluation, vec![s], b"theory-a\tfails"));
        assert_ne!(e1, e1b);
        let mut parents2 = vec![e1b, e2];
        parents2.sort();
        let graph2 = g.insert(Node::new(NodeKind::Evidence, parents2, b"slug-flipped"));
        assert_ne!(graph, graph2);

        assert!(g.descendants(e1).contains(&graph));
        assert!(!g.descendants(e1).contains(&graph2));
        let kept = g.preserved_if_changed(e1);
        assert!(kept.contains(&e2));
        assert!(!kept.contains(&graph));
        assert!(kept.contains(&graph2));
    }

    #[test]
    fn numeric_certificate_is_not_a_calculation_or_receipt() {
        let enclosure = Node::new(NodeKind::NumericCertificate, vec![], b"3/8\t3/8");
        let calc = Node::new(NodeKind::Calculation, vec![], b"3/8\t3/8");
        let receipt = Node::new(NodeKind::VerificationReceipt, vec![], b"3/8\t3/8");
        assert_ne!(enclosure.id, calc.id);
        assert_ne!(enclosure.id, receipt.id);
        assert_eq!(enclosure.kind, NodeKind::NumericCertificate);
    }

    #[test]
    fn encoding_package_is_not_a_theory_or_review() {
        let pkg = Node::new(NodeKind::EncodingPackage, vec![], b"id = fork\n");
        let theory = Node::new(NodeKind::Theory, vec![], b"id = fork\n");
        let review = Node::new(NodeKind::SemanticReview, vec![], b"id = fork\n");
        assert_ne!(pkg.id, theory.id);
        assert_ne!(pkg.id, review.id);
        assert_eq!(pkg.kind, NodeKind::EncodingPackage);
    }

    #[test]
    fn judgment_projection_is_not_an_evaluation_or_evidence() {
        let proj = Node::new(NodeKind::JudgmentProjection, vec![], b"heuristic failed");
        let eval = Node::new(NodeKind::Evaluation, vec![], b"heuristic failed");
        let graph = Node::new(NodeKind::Evidence, vec![], b"heuristic failed");
        assert_ne!(proj.id, eval.id);
        assert_ne!(proj.id, graph.id);
        assert_eq!(proj.kind, NodeKind::JudgmentProjection);
    }
}
