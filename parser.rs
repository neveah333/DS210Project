use csv::ReaderBuilder;
use std::collections::HashMap;
use std::error::Error;

// parser.rs is used for reading and parsing the graph data from the provided csv file into a readable format

pub type Graph = HashMap<String, Vec<String>>;

pub fn parse_csv(file_path: &str) -> Result<Graph, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(false).from_path(file_path)?;
    let mut graph: Graph = HashMap::new();

    for result in rdr.records() {
        let record = result?;
        let source = record.get(0).unwrap().to_string();
        let target = record.get(1).unwrap().to_string();

        graph.entry(source.clone()).or_default().push(target.clone());
        graph.entry(target).or_default().push(source);
    }

    Ok(graph)
}
