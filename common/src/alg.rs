use std::collections::{BinaryHeap, HashMap};

// General Dijkstra’s algorithm for min/maximization problems
// state should include:
//  1. current location
//  2. current output
//  3. anything that effects next steps
// cache key should omit the current output
pub fn dijkstra<TState, TCacheKey, FNext, TI, FFinal>(
    start_state: TState,
    next: FNext,
    final_predicate: FFinal,
) -> Option<TState>
where
    TState: Ord + Into<TCacheKey> + Clone,
    TCacheKey: Eq + std::hash::Hash,
    FNext: Fn(&TState) -> TI,
    TI: IntoIterator<Item = TState>,
    FFinal: Fn(&TState) -> bool,
{
    let mut cache: HashMap<TCacheKey, TState> = HashMap::new();
    let mut heap = BinaryHeap::new();
    heap.push(start_state);

    while let Some(state) = heap.pop() {
        if final_predicate(&state) {
            return Some(state);
        }

        match cache.get(&state.clone().into()) {
            Some(prev_state) if state < *prev_state => {
                continue;
            }
            _ => (),
        }

        for next in next(&state) {
            let key: TCacheKey = next.clone().into();
            match cache.get(&key) {
                Some(prev_state) if next <= *prev_state => (),
                _ => {
                    cache.insert(key, next.clone());
                    heap.push(next);
                }
            }
        }
    }

    None
}
