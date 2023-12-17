use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct BestState<TCurrent, TScore> {
    current: TCurrent,
    score: TScore,
}

impl<TCurrent, TScore> Ord for BestState<TCurrent, TScore>
where
    TCurrent: Ord,
    TScore: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then_with(|| self.current.cmp(&other.current))
    }
}

impl<TCurrent, TScore> PartialOrd for BestState<TCurrent, TScore>
where
    TCurrent: Ord,
    TScore: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn best_score<TCurrent, TScore, FNext, TNextSteps, FTerminates>(
    start: TCurrent,
    next: FNext,
    terminates: FTerminates,
) -> TScore
where
    TCurrent: Eq + Ord + Clone + std::hash::Hash + std::fmt::Debug,
    TScore: Default + Ord + Clone + std::fmt::Debug,
    FNext: Fn(&TCurrent, &TScore) -> TNextSteps,
    TNextSteps: Iterator<Item = (TCurrent, TScore)>,
    FTerminates: Fn(&TCurrent) -> bool,
{
    // todo: I think this can just use the generic dijkstra function
    let mut scores = HashMap::new();
    let start_score: TScore = Default::default();
    scores.insert(start.clone(), start_score.clone());

    let mut heap = BinaryHeap::new();
    heap.push(BestState {
        current: start,
        score: start_score,
    });

    // using a binary heap means we're always looking at the next best unvisited node
    while let Some(BestState { current, score }) = heap.pop() {
        if terminates(&current) {
            return score;
        }

        // if we're already above the previous best to this point, don't bother continuing
        match scores.get(&current) {
            Some(prev_score) if &score < prev_score => {
                continue;
            }
            _ => (),
        }

        for (next_step, next_score) in next(&current, &score) {
            // if this is the best score calculated so far, push it on the heap
            match scores.get(&next_step) {
                Some(prev_score) if &next_score <= prev_score => (),
                _ => {
                    heap.push(BestState {
                        current: next_step.clone(),
                        score: next_score.clone(),
                    });
                    scores.insert(next_step, next_score);
                }
            }
        }
    }

    Default::default()
}
