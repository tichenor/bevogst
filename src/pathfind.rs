use std::collections::{HashSet, VecDeque, BinaryHeap, HashMap};

use crate::point::Point;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pub p: Point,
    pub cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
        .then_with(|| self.p.cmp(&other.p))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn path_astar(
    start: Point,
    end: Point,
    tiles: &HashSet<Point>,
    tile_blockers: &HashSet<Point>,
) -> Option<VecDeque<Point>> {

    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();
    let mut came_from = HashMap::new();

    queue.push(Node { p: start, cost: 0 });
    visited.insert(start, 0);

    while let Some(Node { p, cost }) = queue.pop() {
        if p == end { break; }
        for dir in Point::OCTANT {
            let neighbor = p + dir;
            let new_cost = cost + 1;
            if !tiles.contains(&neighbor) { continue; }
            if tile_blockers.contains(&neighbor) && neighbor != end { continue; }
            
            match visited.get(&neighbor) {
                Some(c) if *c <= new_cost => {},
                _ => {
                    visited.insert(neighbor, new_cost);
                    queue.push(Node { p: neighbor, cost: new_cost });
                    came_from.insert(neighbor, p);
                }
            }
        }
    }

    let mut path = VecDeque::new();
    let mut cur = end;
    
    while let Some(p) = came_from.get(&cur) {
        path.push_front(cur);
        cur = *p;
        if cur == start { 
            return Some(path)
        }
    }
    None
}
