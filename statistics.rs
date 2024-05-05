use std::collections::{HashMap, HashSet, VecDeque};
use super::parser::Graph;

pub fn degree_distribution(graph: &Graph) -> HashMap<String, usize> {
    let mut degrees = HashMap::new();

    for (node, neighbors) in graph {
        degrees.insert(node.clone(), neighbors.len());
    }

    degrees
}

pub fn mean(degrees: &HashMap<String, usize>) -> f64 {
    let sum: usize = degrees.values().sum();
    sum as f64 / degrees.len() as f64
}

pub fn variance(degrees: &HashMap<String, usize>, mean: f64) -> f64 {
    degrees.values().map(|&val| {
        let diff = val as f64 - mean;
        diff * diff
    }).sum::<f64>() / degrees.len() as f64
}

pub fn standard_deviation(degrees: &HashMap<String, usize>, mean: f64) -> f64 {
    variance(degrees, mean).sqrt()
}

pub fn mode(degrees: &HashMap<String, usize>) -> Vec<usize> {
    let mut frequency_map = HashMap::new();
    let mut max_freq = 0;

    for &degree in degrees.values() {
        let count = frequency_map.entry(degree).or_insert(0);
        *count += 1;
        max_freq = max_freq.max(*count);
    }

    frequency_map.into_iter()
        .filter(|&(_, count)| count == max_freq)
        .map(|(val, _)| val)
        .collect()
}

pub fn count_connected_components(graph: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut count = 0;

    for node in graph.keys() {
        if !visited.contains(node) {
            bfs_visit(node, graph, &mut visited);
            count += 1;
        }
    }

    count
}

fn bfs_visit(start: &str, graph: &Graph, visited: &mut HashSet<String>) {
    let mut queue = VecDeque::new();
    queue.push_back(start.to_string());

    while let Some(current) = queue.pop_front() {
        if visited.insert(current.clone()) {
            if let Some(neighbors) = graph.get(&current) {
                for neighbor in neighbors {
                    queue.push_back(neighbor.to_string());
                }
            }
        }
    }
}

pub fn calculate_density(graph: &Graph) -> f64 {
    let num_edges = graph.iter().map(|(_, neighbors)| neighbors.len()).sum::<usize>() / 2;
    let num_nodes = graph.len();
    num_edges as f64 / (num_nodes * (num_nodes - 1) / 2) as f64
}

