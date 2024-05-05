// Test file for testing the functionality of the graph 

use your_crate_name::parser::{self, Graph};
use your_crate_name::bfs::{self};
use your_crate_name::statistics::{self};

#[test]
fn test_csv_parsing() {
    let graph = parser::parse_csv("/Users/gzs/Desktop/BU/DS210/project/euroroad.csv").unwrap();
    assert!(!graph.is_empty());
}

#[test]
fn test_bfs() {
    let graph = parser::parse_csv("/Users/gzs/Desktop/BU/DS210/project/euroroad.csv").unwrap();
    let path = bfs::bfs(&graph, "start_vertex", "end_vertex");
    assert!(path.is_some());
}

#[test]
fn test_degree_distribution() {
    let graph = parser::parse_csv("/Users/gzs/Desktop/BU/DS210/project/euroroad.csv").unwrap();
    let distribution = statistics::degree_distribution(&graph);
    assert!(!distribution.is_empty());
}
