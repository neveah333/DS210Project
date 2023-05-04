use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::dijkstra;
use petgraph::Undirected;

fn main() {
    let file = File::open("facebook_combined.txt").unwrap();
    let reader = BufReader::new(file);

    let mut graph = Graph::<i32, i32>::new(); 

    for line in reader.lines() {
        let line = line.unwrap();
        let split_line: Vec<&str> = line.split_whitespace().collect();

        let from_node = split_line[0].parse::<usize>().unwrap();
        let to_node = split_line[1].parse::<usize>().unwrap();

        let from_node_index = NodeIndex::new(from_node);
        let to_node_index = NodeIndex::new(to_node);

        graph.add_edge(from_node_index, to_node_index, 1); 
    }

    // Find the average distance between pairs of vertices in the graph using Dijkstra's algorithm
    let num_nodes = graph.node_count();
    let mut total_distance = 0.0;

    for i in 0..num_nodes {
        let start_node = NodeIndex::new(i);

        let distances: Option<HashMap<NodeIndex, f32>> = Some(dijkstra(&graph, start_node, None, |_| 1)); // use dijkstra's algorithm

        for j in i+1..num_nodes {
            let end_node = NodeIndex::new(j);

            if let Some(distance) = distances.clone().expect("REASON").get(&end_node) {
                total_distance += *distance;
            }
        }
    }

    let num_pairs = num_nodes * (num_nodes - 1) / 2;
    let average_distance = total_distance / num_pairs as f32;

    println!("Average distance: {}", average_distance);
}
