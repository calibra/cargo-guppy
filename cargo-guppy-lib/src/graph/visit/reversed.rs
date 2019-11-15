use petgraph::prelude::*;
use petgraph::visit::{
    Data, GraphBase, GraphRef, IntoEdgeReferences, IntoEdges, IntoEdgesDirected, IntoNeighbors,
    IntoNeighborsDirected, IntoNodeIdentifiers, IntoNodeReferences, Visitable,
};

/// `ReversedDirected` is a reversing adapter for directed graphs.
///
/// This is similar to `petgraph::visit::Reversed`, except with `IntoEdges` and `IntoEdgesDirected`
/// implemented as well. Unfortunately, due to an inconsistency between undirected and directed
/// graphs, these trait impls don't behave correctly for undirected graphs.
///
/// For more details about the issue, see the
/// [petgraph bugtracker](https://github.com/petgraph/petgraph/issues/292).
#[derive(Copy, Clone, Debug)]
pub struct ReversedDirected<G>(G);

impl<G> ReversedDirected<G> {
    pub fn new(graph: G) -> Self {
        ReversedDirected(graph)
    }
}

impl<G: GraphBase> GraphBase for ReversedDirected<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<G: GraphRef> GraphRef for ReversedDirected<G> {}

impl<G: Data> Data for ReversedDirected<G>
where
    G: Data,
{
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;
}

// ---
// New traits
// ---

/// Provides a way to flip source and target indexes for reversed graphs.
///
/// Some operations that are generic over forward and reverse graphs may want the original
/// direction. This trait provides that.
pub trait ReverseFlip {
    /// Node index type.
    type NodeId;

    /// Whether this graph is reversed.
    fn is_reversed() -> bool;

    /// Flip the source and target indexes if this is a reversed graph. Leave them the same if it
    /// isn't.
    fn reverse_flip(source: Self::NodeId, target: Self::NodeId) -> (Self::NodeId, Self::NodeId);
}

// TODO: implement ReverseFlip for all the other base graph types as well.

impl<'a, NW, EW, Ty, Ix> ReverseFlip for &'a Graph<NW, EW, Ty, Ix> {
    type NodeId = NodeIndex<Ix>;

    fn is_reversed() -> bool {
        false
    }

    fn reverse_flip(source: Self::NodeId, target: Self::NodeId) -> (Self::NodeId, Self::NodeId) {
        // A graph itself is in the forward direction.
        (source, target)
    }
}

impl<'a, G: 'a, N> ReverseFlip for ReversedDirected<&'a G>
where
    N: Copy + PartialEq,
    G: GraphBase<NodeId = N>,
    &'a G: ReverseFlip<NodeId = N>,
{
    type NodeId = N;

    fn is_reversed() -> bool {
        !<&G>::is_reversed()
    }

    fn reverse_flip(source: Self::NodeId, target: Self::NodeId) -> (Self::NodeId, Self::NodeId) {
        // This allows nested reversed graphs to do the right thing.
        let (target, source) = <&G>::reverse_flip(source, target);
        (source, target)
    }
}

// ---
// New impls for existing traits
// ---

impl<G> IntoEdges for ReversedDirected<G>
where
    G: IntoEdgesDirected,
{
    type Edges = ReversedEdges<G::EdgesDirected>;
    fn edges(self, a: Self::NodeId) -> Self::Edges {
        ReversedEdges {
            iter: self.0.edges_directed(a, Incoming),
        }
    }
}

impl<G> IntoEdgesDirected for ReversedDirected<G>
where
    G: IntoEdgesDirected,
{
    type EdgesDirected = ReversedEdges<G::EdgesDirected>;
    fn edges_directed(self, a: Self::NodeId, dir: Direction) -> Self::Edges {
        ReversedEdges {
            iter: self.0.edges_directed(a, dir.opposite()),
        }
    }
}

/// A reversed edges iterator.
pub struct ReversedEdges<I> {
    iter: I,
}

impl<I> Iterator for ReversedEdges<I>
where
    I: Iterator,
    I::Item: EdgeRef,
{
    type Item = ReversedEdgeReference<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(ReversedEdgeReference)
    }
}

// ---
// Other impls, copied from petgraph
// ---

impl<G> IntoNeighbors for ReversedDirected<G>
where
    G: IntoNeighborsDirected,
{
    type Neighbors = G::NeighborsDirected;
    fn neighbors(self, n: G::NodeId) -> G::NeighborsDirected {
        self.0.neighbors_directed(n, Incoming)
    }
}

impl<G> IntoNeighborsDirected for ReversedDirected<G>
where
    G: IntoNeighborsDirected,
{
    type NeighborsDirected = G::NeighborsDirected;
    fn neighbors_directed(self, n: G::NodeId, d: Direction) -> G::NeighborsDirected {
        self.0.neighbors_directed(n, d.opposite())
    }
}

impl<G: Visitable> Visitable for ReversedDirected<G> {
    type Map = G::Map;
    fn visit_map(&self) -> G::Map {
        self.0.visit_map()
    }
    fn reset_map(&self, map: &mut Self::Map) {
        self.0.reset_map(map);
    }
}

/// A reversed edge reference.
#[derive(Copy, Clone, Debug)]
pub struct ReversedEdgeReference<R>(R);

/// An edge reference
impl<R> EdgeRef for ReversedEdgeReference<R>
where
    R: EdgeRef,
{
    type NodeId = R::NodeId;
    type EdgeId = R::EdgeId;
    type Weight = R::Weight;
    fn source(&self) -> Self::NodeId {
        self.0.target()
    }
    fn target(&self) -> Self::NodeId {
        self.0.source()
    }
    fn weight(&self) -> &Self::Weight {
        self.0.weight()
    }
    fn id(&self) -> Self::EdgeId {
        self.0.id()
    }
}

impl<G> IntoEdgeReferences for ReversedDirected<G>
where
    G: IntoEdgeReferences,
{
    type EdgeRef = ReversedEdgeReference<G::EdgeRef>;
    type EdgeReferences = ReversedEdgeReferences<G::EdgeReferences>;
    fn edge_references(self) -> Self::EdgeReferences {
        ReversedEdgeReferences {
            iter: self.0.edge_references(),
        }
    }
}

/// A reversed edge references iterator.
pub struct ReversedEdgeReferences<I> {
    iter: I,
}

impl<I> Iterator for ReversedEdgeReferences<I>
where
    I: Iterator,
    I::Item: EdgeRef,
{
    type Item = ReversedEdgeReference<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(ReversedEdgeReference)
    }
}

impl<G> IntoNodeIdentifiers for ReversedDirected<G>
where
    G: IntoNodeIdentifiers,
{
    type NodeIdentifiers = G::NodeIdentifiers;
    fn node_identifiers(self) -> Self::NodeIdentifiers {
        self.0.node_identifiers()
    }
}

impl<G> IntoNodeReferences for ReversedDirected<G>
where
    G: IntoNodeReferences,
{
    type NodeRef = G::NodeRef;
    type NodeReferences = G::NodeReferences;
    fn node_references(self) -> Self::NodeReferences {
        self.0.node_references()
    }
}

// This is incomplete -- feel free to add more forwarding impls as necessary!
