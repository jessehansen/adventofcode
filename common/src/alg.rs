use std::{
    cmp::{Eq, Ord, Ordering, PartialOrd},
    collections::{BinaryHeap, HashMap},
};

pub trait OptimizationState {
    type CacheKey: Eq + std::hash::Hash;
    type Score: Eq + Ord;

    fn cache_key(&self) -> Self::CacheKey;
    fn score(&self) -> Self::Score;
}

struct OptimizationStateWrapper<TState>(TState);

impl<TState> PartialEq for OptimizationStateWrapper<TState>
where
    TState: OptimizationState,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.score() == other.0.score()
    }
}
impl<TState> Eq for OptimizationStateWrapper<TState> where TState: OptimizationState {}

impl<TState> Ord for OptimizationStateWrapper<TState>
where
    TState: OptimizationState,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // comparing heat loss in reverse to minimize instead of maximize
        self.0.score().cmp(&other.0.score())
    }
}

impl<TState> PartialOrd for OptimizationStateWrapper<TState>
where
    TState: OptimizationState,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// General Dijkstra’s algorithm for min/maximization problems
// state should include:
//  1. current location
//  2. current output
//  3. anything that affects next steps
// cache key should omit the current output
pub fn dijkstra<TState, FNext, TI, FFinal>(
    start_state: TState,
    next: FNext,
    final_predicate: FFinal,
) -> Option<TState::Score>
where
    TState: OptimizationState,
    FNext: Fn(&TState) -> TI,
    TI: IntoIterator<Item = TState>,
    FFinal: Fn(&TState) -> bool,
{
    let mut cache: HashMap<TState::CacheKey, TState::Score> = HashMap::new();
    let mut heap: BinaryHeap<OptimizationStateWrapper<TState>> = BinaryHeap::new();
    heap.push(OptimizationStateWrapper(start_state));

    while let Some(OptimizationStateWrapper(state)) = heap.pop() {
        if final_predicate(&state) {
            return Some(state.score());
        }

        match cache.get(&state.cache_key()) {
            Some(prev_score) if state.score() < *prev_score => {
                continue;
            }
            _ => (),
        }

        for next in next(&state) {
            let key = next.cache_key();
            let score = next.score();
            match cache.get(&key) {
                Some(prev_score) if score <= *prev_score => (),
                _ => {
                    cache.insert(key, score);
                    heap.push(OptimizationStateWrapper(next));
                }
            }
        }
    }

    None
}
