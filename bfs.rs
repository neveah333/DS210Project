use std::collections::{HashMap, VecDeque};
use super::parser::Graph;

// This is the Breadth-first search algorithm file
pub fn bfs(graph: &Graph, start: &str, goal: &str) -> Option<usize> {
    if !graph.contains_key(start) || !graph.contains_key(goal) {
        return None;
    }

    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start.to_string(), 0);
    queue.push_back(start.to_string());

    while let Some(current) = queue.pop_front() {
        if &current == goal {
            return visited.get(goal).cloned();
        }

        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors {
                if !visited.contains_key(neighbor) {
                    visited.insert(neighbor.to_string(), visited[&current] + 1);
                    queue.push_back(neighbor.to_string());
                }
            }
        }
    }

    None
}

