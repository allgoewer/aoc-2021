//! Day 15: Chiton
use aoc21::{util::*, Quizzer};
use std::collections::{BinaryHeap, HashMap};

/// Todays quiz implementation
pub struct Quiz;

impl Quizzer for Quiz {
    fn part1(&self, input: &str) -> String {
        cheapest_path(&parse(input, 1))
            .unwrap_or_default()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        cheapest_path(&parse(input, 5))
            .unwrap_or_default()
            .to_string()
    }
}

/// A risk level map of a cavern
type Risks = Grid<u8>;

/// A node for use in Dijkstra's algorithm
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    /// The position on the grid
    pos: (usize, usize),
    /// The cost of this node
    cost: u64,
}

impl Node {
    /// Create a new node
    fn new(pos: (usize, usize), cost: u64) -> Self {
        Self { pos, cost }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            // In case of a tie, compare positions - this is necessary to be consistent with `PartialEq` and `Ord`
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Parse the input of todays quiz
fn parse(input: &str, repeat: u8) -> Risks {
    let mut width = 0;
    let modval = |value, offset| {
        let value = value + offset;
        if value > 9 {
            value - 9
        } else {
            value
        }
    };

    let risks: Vec<_> = input
        .lines()
        .map(str::trim)
        .map(|l| {
            let bytes = l.as_bytes();
            width = l.len() * repeat as usize;

            (0..repeat)
                .map(|offset| {
                    bytes
                        .iter()
                        .map(|b| b - b'0')
                        .map(move |v| modval(v, offset))
                })
                .flatten()
        })
        .flatten()
        .collect();

    let risks: Vec<_> = (0..repeat)
        .map(|offset| risks.iter().map(move |&v| modval(v, offset)))
        .flatten()
        .collect();

    (risks, width).try_into().expect("parsing risk map failed")
}

/// Calculate the lowest-risk paths total risk with dijkstras algorithm
fn cheapest_path(risks: &Risks) -> Option<u64> {
    let dim = risks.dim();
    let start = (0, 0);
    let end = (dim.0 - 1, dim.1 - 1);
    let mut heap = BinaryHeap::new();
    let adjacent: HashMap<_, _> = risks
        .index_iter()
        .map(|(x, y)| {
            let neighbours = [
                (x, y.wrapping_add(1)),
                (x, y.wrapping_sub(1)),
                (x.wrapping_add(1), y),
                (x.wrapping_sub(1), y),
            ];

            let neighbours: Vec<_> = neighbours
                .into_iter()
                .filter(|&npos| risks.contains(npos))
                .collect();

            ((x, y), neighbours)
        })
        .collect();

    let mut costs: Grid<u64> = (vec![u64::MAX; risks.len()], dim.0).try_into().unwrap();

    costs[start] = 0;
    heap.push(Node::new(start, 0));

    while let Some(Node { pos, cost }) = heap.pop() {
        // end found, return the total cost
        if pos == end {
            return Some(cost);
        }

        if cost > costs[pos] {
            continue;
        }

        for &npos in &adjacent[&pos] {
            let next = Node::new(npos, cost + risks[npos] as u64);

            if next.cost < costs[npos] {
                costs[npos] = next.cost;
                heap.push(next);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        1163751742
        1381373672
        2136511328
        3694931569
        7463417111
        1319128137
        1359912421
        3125421639
        1293138521
        2311944581";

    #[test]
    fn part1_examples() {
        assert_eq!(cheapest_path(&parse(EXAMPLE, 1)), Some(40));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(cheapest_path(&parse(EXAMPLE, 5)), Some(315));
    }
}
