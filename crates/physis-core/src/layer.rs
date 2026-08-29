//! Description layers as a trait objects can implement.

use crate::id::LayerId;

/// A mechanical stratum: state, knobs, observables.
///
/// Implementors live in `physis-model`. This trait is the *shape*
/// every layer shares so agents can walk the tower uniformly.
pub trait Layer {
    /// Which stratum this is.
    const ID: LayerId;

    /// Snapshot type an agent can print.
    type Observable;

    /// Current observable view.
    fn observe(&self) -> Self::Observable;
}
