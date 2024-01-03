use std::{
    cmp::Reverse,
    collections::{HashMap, VecDeque},
};

use crate::{dijkstra, OptimizationState};

type NodeId = usize;

// unweighted, single directional graph
// uses a node table to keep node data and refers
// to nodes via index (NodeId)
//
// Bidirectional helper methods included (_bidi)
pub struct UnweightedGraph<TNodeData> {
    nodes: HashMap<TNodeData, NodeId>,
    edges: HashMap<NodeId, Vec<NodeId>>,
}

impl<TNodeData> UnweightedGraph<TNodeData> {
    pub fn new() -> UnweightedGraph<TNodeData> {
        UnweightedGraph::<TNodeData> {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }
}

impl<TNodeData> UnweightedGraph<TNodeData> {
    pub fn connect(&mut self, node: NodeId, other: NodeId) {
        self.edges.entry(node).or_default().push(other);
    }

    pub fn connect_bidi(&mut self, node: NodeId, other: NodeId) {
        self.edges.entry(node).or_default().push(other);
        self.edges.entry(other).or_default().push(node);
    }

    pub fn disconnect(&mut self, node: NodeId, other: NodeId) {
        if let Some(edges) = self.edges.get_mut(&node) {
            edges.retain(|e| e != &other);
        }
    }

    pub fn disconnect_bidi(&mut self, node: NodeId, other: NodeId) {
        if let Some(edges) = self.edges.get_mut(&node) {
            edges.retain(|e| e != &other);
        }
        if let Some(edges) = self.edges.get_mut(&other) {
            edges.retain(|e| e != &node);
        }
    }

    pub fn nodes(&self) -> impl Iterator<Item = &TNodeData> {
        self.nodes.keys()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn shortest_path(&self, from: NodeId, to: NodeId) -> Option<Vec<NodeId>> {
        dijkstra(
            PathState {
                node: from,
                path: vec![],
            },
            |state| {
                let mut path = state.path.to_vec();
                path.push(state.node);

                self.edges[&state.node].iter().filter_map(move |next| {
                    if path.contains(next) {
                        None
                    } else {
                        Some(PathState {
                            node: *next,
                            path: path.clone(),
                        })
                    }
                })
            },
            |state| state.node == to,
        )
        .map(|state| state.path)
    }

    pub fn find_connected_group(&self, start: NodeId) -> Vec<NodeId> {
        let mut seen = vec![false; self.nodes.len()];
        let mut stack = vec![start];
        while let Some(next) = stack.pop() {
            for &connected in &self.edges[&next] {
                if !seen[connected] {
                    stack.push(connected);
                    seen[connected] = true;
                }
            }
        }
        seen.into_iter()
            .enumerate()
            .filter_map(
                |(ix, was_encountered)| {
                    if was_encountered {
                        Some(ix)
                    } else {
                        None
                    }
                },
            )
            .collect()
    }

    // BFS furthest node implementation
    pub fn furthest_node(&self, start: NodeId) -> Option<NodeId> {
        let mut todo = VecDeque::new();
        todo.push_back(start);

        let mut seen = vec![false; self.nodes.len()];
        seen[start] = true;

        let mut furthest = start;

        while let Some(current) = todo.pop_front() {
            furthest = current;

            for &next in &self.edges[&current] {
                if !seen[next] {
                    todo.push_back(next);
                    seen[next] = true;
                }
            }
        }

        if furthest == start {
            None
        } else {
            Some(furthest)
        }
    }
}

impl<TNodeData> UnweightedGraph<TNodeData>
where
    TNodeData: Eq + std::hash::Hash,
{
    // similar to HashMap/Set insert methods, always returns an index
    pub fn insert_node(&mut self, node: TNodeData) -> NodeId {
        let next_ix = self.nodes.len();
        *self.nodes.entry(node).or_insert(next_ix)
    }

    pub fn get_node_id(&self, node: &TNodeData) -> Option<NodeId> {
        self.nodes.get(node).copied()
    }

    pub fn add_edge(&mut self, node: TNodeData, other: TNodeData) {
        let node = self.insert_node(node);
        let other = self.insert_node(other);

        self.edges.entry(node).or_default().push(other);
    }

    pub fn add_edges<I>(&mut self, a: TNodeData, others: I)
    where
        I: IntoIterator<Item = TNodeData>,
    {
        let node = self.insert_node(a);

        for other in others {
            let other = self.insert_node(other);
            self.edges.entry(node).or_default().push(other);
        }
    }

    pub fn add_edge_bidi(&mut self, node: TNodeData, other: TNodeData) {
        let node = self.insert_node(node);
        let other = self.insert_node(other);

        self.edges.entry(node).or_default().push(other);
        self.edges.entry(other).or_default().push(node);
    }

    pub fn add_edges_bidi<I>(&mut self, node: TNodeData, others: I)
    where
        I: IntoIterator<Item = TNodeData>,
    {
        let node = self.insert_node(node);

        for other in others {
            let other = self.insert_node(other);
            self.edges.entry(node).or_default().push(other);
            self.edges.entry(other).or_default().push(node);
        }
    }
}

impl<TNodeData> Default for UnweightedGraph<TNodeData> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PathState {
    node: NodeId,
    path: Vec<NodeId>,
}

impl OptimizationState for PathState {
    type CacheKey = NodeId;

    type Score = Reverse<usize>;

    fn cache_key(&self) -> Self::CacheKey {
        self.node
    }

    fn score(&self) -> Self::Score {
        Reverse(self.path.len())
    }
}

impl<T> Clone for UnweightedGraph<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
            edges: self.edges.clone(),
        }
    }
}
