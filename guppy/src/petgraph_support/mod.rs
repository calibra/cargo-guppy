//! Support for petgraph.
//!
//! The code in here is generic over petgraph's traits, and could be upstreamed into petgraph if
//! desirable.

use petgraph::visit::EdgeRef;

pub mod dot;
pub mod reversed;
pub mod walk;

pub fn edge_triple<ER: EdgeRef>(edge_ref: ER) -> (ER::NodeId, ER::NodeId, ER::EdgeId) {
    (edge_ref.source(), edge_ref.target(), edge_ref.id())
}
