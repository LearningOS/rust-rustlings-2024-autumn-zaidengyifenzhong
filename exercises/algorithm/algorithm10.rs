/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph; 

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }

    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }

    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }

    fn add_node(&mut self, node: &str) -> bool {
        if !self.contains(node) {
            self.adjacency_table.insert(String::from(node), Vec::new());
            true
        } else {
            false
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;

        // Ensure both nodes exist in the graph before adding the edge
        if !self.contains(node1) {
            self.add_node(node1);
        }
        if !self.contains(node2) {
            self.add_node(node2);
        }

        // Add the edge to both nodes
        self.adjacency_table.entry(String::from(node1)).or_insert_with(Vec::new)
            .push((String::from(node2), weight));

        self.adjacency_table.entry(String::from(node2)).or_insert_with(Vec::new)
            .push((String::from(node1), weight));
    }
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool;
    fn add_edge(&mut self, edge: (&str, &str, i32));
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(String, String, i32)> {
        let mut edges = Vec::new();
        let mut seen_edges = HashSet::new();
    
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                let edge = (from_node.clone(), to_node.clone(), *weight);
                let reverse_edge = (to_node.clone(), from_node.clone(), *weight);
    
                // Ensure the edge is always stored in a consistent order
                let canonical_edge = if from_node < to_node {
                    edge
                } else {
                    reverse_edge
                };
    
                // Only add the edge if we haven't already added it
                if !seen_edges.contains(&canonical_edge) {
                    seen_edges.insert(canonical_edge.clone());
                    edges.push(canonical_edge);
                }
            }
        }
        edges
    }
}

#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;

#[test]  
fn test_add_edge() {  
    let mut graph = UndirectedGraph::new();  
    graph.add_edge(("a", "b", 5));  
    graph.add_edge(("b", "c", 10));  
    graph.add_edge(("c", "a", 7));  
  
    let expected_edges = vec![  
        (String::from("a"), String::from("b"), 5),  
        (String::from("a"), String::from("c"), 7),  // 更新这里的顺序  
        (String::from("b"), String::from("c"), 10),  
    ];  
  
    let actual_edges = graph.edges();  
    for edge in &expected_edges {  
        assert!(actual_edges.contains(edge), "Edge {:?} not found in {:?}", edge, actual_edges);  
    }  
}
}