use std::{
    cmp::Reverse,
    collections::VecDeque,
    fmt::Debug,
    ops::{Add, Deref, DerefMut},
};

use crate::{dijkstra, OptimizationState};

type NodeId = usize;

// Most problems either consider nodes as distances in an unweighted graph
// or edges have weights. We have both flavors of graph defined here for ease of use

#[derive(Debug)]
pub struct Graph<N, E> {
    nodes: Vec<N>,
    edges: Vec<Vec<(NodeId, E)>>,
}

impl<N, E> Graph<N, E> {
    pub fn add_node(&mut self, node: N) -> NodeId {
        let next_ix = self.nodes.len();
        self.nodes.push(node);
        self.edges.push(Vec::default());
        next_ix
    }

    pub fn connect(&mut self, node: NodeId, other: NodeId, edge: E) {
        self.edges[node].push((other, edge));
    }

    pub fn disconnect(&mut self, node: NodeId, other: NodeId) {
        if let Some(edges) = self.edges.get_mut(node) {
            edges.retain(|(e, _)| e != &other);
        }
    }

    pub fn disconnect_bidi(&mut self, node: NodeId, other: NodeId) {
        if let Some(edges) = self.edges.get_mut(node) {
            edges.retain(|(e, _)| e != &other);
        }
        if let Some(edges) = self.edges.get_mut(other) {
            edges.retain(|(e, _)| e != &node);
        }
    }

    pub fn node(&self, node: NodeId) -> &N {
        &self.nodes[node]
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn neighbors(&self, node: NodeId) -> Vec<(NodeId, &N, &E)> {
        self.edges[node]
            .iter()
            .map(|(id, edge)| (*id, &self.nodes[*id], edge))
            .collect()
    }
}

impl<N, E> Graph<N, E>
where
    N: Eq,
{
    pub fn insert_unique_node(&mut self, node: N) -> NodeId {
        if let Some((ix, _)) = self.nodes.iter().enumerate().find(|(_, n)| &&node == n) {
            ix
        } else {
            self.add_node(node)
        }
    }

    pub fn find_node_id(&self, node: &N) -> Option<NodeId> {
        self.nodes
            .iter()
            .enumerate()
            .find_map(|(ix, n)| if node == n { Some(ix) } else { None })
    }
}

impl<N, E> Graph<N, E>
where
    E: Copy,
{
    pub fn connect_bidi(&mut self, node: NodeId, other: NodeId, edge: E) {
        self.edges[node].push((other, edge));
        self.edges[other].push((node, edge));
    }
}

impl<N, E> Default for Graph<N, E> {
    fn default() -> Self {
        Self {
            nodes: Vec::default(),
            edges: Vec::default(),
        }
    }
}

impl<N, E> Graph<N, E>
where
    E: Copy + Default + Add<Output = E> + Ord,
{
    pub fn longest_path(&self, start: NodeId, end: NodeId) -> Option<E> {
        // DFS, this could be improved by doing a Topological sort first and only considering
        // later topological paths
        let mut todo: VecDeque<(NodeId, Vec<bool>, E)> = VecDeque::default();
        todo.push_back((start, vec![false; self.nodes.len()], Default::default()));

        let mut distance = vec![];

        while let Some((from, mut visited, path_length)) = todo.pop_back() {
            if from == end {
                distance.push(path_length);
            } else {
                visited[from] = true;
                self.edges[from].iter().for_each(|(next, length)| {
                    if !visited[*next] {
                        todo.push_back((*next, visited.clone(), path_length + *length));
                    }
                });
            }
        }

        distance.into_iter().max()
    }

    pub fn shortest_path(&self, start: NodeId, end: NodeId) -> Option<Vec<NodeId>> {
        dijkstra(
            PathState::<E> {
                node: start,
                path: vec![],
                length: Default::default(),
            },
            |state| {
                let mut path = state.path.to_vec();
                path.push(state.node);
                let length_so_far = state.length;

                self.edges[state.node]
                    .iter()
                    .filter_map(move |(next, length)| {
                        if path.contains(next) {
                            None
                        } else {
                            Some(PathState {
                                node: *next,
                                path: path.clone(),
                                length: length_so_far + *length,
                            })
                        }
                    })
            },
            |state| state.node == end,
        )
        .map(|state| state.path)
    }

    pub fn find_connected_group(&self, start: NodeId) -> Vec<NodeId> {
        let mut seen = vec![false; self.nodes.len()];
        let mut stack = vec![start];
        while let Some(next) = stack.pop() {
            for &(connected, _) in &self.edges[next] {
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
}

pub struct UnweightedGraphX<N>(Graph<N, usize>);

impl<N> Default for UnweightedGraphX<N> {
    fn default() -> Self {
        Self(Graph::default())
    }
}

impl<N> UnweightedGraphX<N>
where
    N: Eq,
{
    pub fn add_edges<I>(&mut self, a: N, others: I)
    where
        I: IntoIterator<Item = N>,
    {
        let node = self.0.insert_unique_node(a);

        for other in others {
            let other = self.0.insert_unique_node(other);
            self.0.edges[node].push((other, 1));
        }
    }

    pub fn add_edge_bidi(&mut self, node: N, other: N) {
        let node = self.0.insert_unique_node(node);
        let other = self.0.insert_unique_node(other);

        self.0.edges[node].push((other, 1));
        self.0.edges[other].push((node, 1));
    }

    pub fn add_edges_bidi<I>(&mut self, node: N, others: I)
    where
        I: IntoIterator<Item = N>,
    {
        let node = self.0.insert_unique_node(node);

        for other in others {
            let other = self.0.insert_unique_node(other);
            self.0.edges[node].push((other, 1));
            self.0.edges[other].push((node, 1));
        }
    }
}

impl<N> UnweightedGraphX<N> {
    // Here because this algorithm does not consider edge weights,
    // Doing so would make this a lot more memory intensive (IIRC it's NP-hard)
    pub fn furthest_node(&self, start: NodeId) -> Option<NodeId> {
        let mut todo = VecDeque::new();
        todo.push_back(start);

        let mut seen = vec![false; self.nodes.len()];
        seen[start] = true;

        let mut furthest = start;

        while let Some(current) = todo.pop_front() {
            furthest = current;

            for &(next, _) in &self.edges[current] {
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

impl<N> Deref for UnweightedGraphX<N> {
    type Target = Graph<N, usize>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<N> DerefMut for UnweightedGraphX<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PathState<T> {
    node: NodeId,
    path: Vec<NodeId>,
    length: T,
}

impl<T> OptimizationState for PathState<T>
where
    T: Ord + Copy,
{
    type CacheKey = NodeId;

    type Score = Reverse<T>;

    fn cache_key(&self) -> Self::CacheKey {
        self.node
    }

    fn score(&self) -> Self::Score {
        Reverse(self.length)
    }
}
