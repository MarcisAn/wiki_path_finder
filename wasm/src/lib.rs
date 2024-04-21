extern crate serde;

use std::collections::HashMap;
use std::fs;

use petgraph::algo::{self, astar, dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{DiGraph, NodeIndex, UnGraph};
use petgraph::visit::NodeRef;
use petgraph::Graph;
use serde::Deserialize;
use wasm_bindgen::prelude::*;

mod DATA;
use DATA::DATA;

#[derive(Debug, Deserialize, Clone)]
struct Page {
    title: String,
    links: Vec<String>,
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn display_result(res: &str);
}

#[wasm_bindgen]
pub fn run(a: &str, b: &str) {
    let pages: Vec<Page> = serde_json::from_str(DATA).expect("JSON was not well-formatted");

    let mut graph = DiGraph::<String, i32>::new();

    let mut pages_hash: HashMap<String, usize> = HashMap::new();

    let mut counter = 0;
    for page in pages.clone() {
        let title = page.clone().title;
        pages_hash.insert(title.to_lowercase(), counter);
        graph.add_node(title.to_lowercase());
        counter += 1;
    }
    counter = 0;
    for page in pages.clone() {
        for link in page.links {
            //println!("{:?} {}", &page.title, &link);
            graph.add_edge(
                NodeIndex::new(*pages_hash.get(&page.title.to_lowercase()).unwrap()),
                NodeIndex::new(*pages_hash.get(&link.to_lowercase()).unwrap()),
                1,
            );
        }
        counter += 1;
    }
    let path = astar(
        &graph,
        NodeIndex::new(*pages_hash.get(&a.to_lowercase()).unwrap()),
        |finish| finish == NodeIndex::new(*pages_hash.get(&b.to_lowercase()).unwrap()),
        |e| *e.weight(),
        |_| 0,
    );
    let mut path_titles: Vec<String> = vec![];
    if path.is_none(){
        display_result("ERROR");
    }
    for node in path.unwrap().1.iter() {
        path_titles.push(pages[node.index()].title.clone());
    }
    display_result(&format!("{:?}", path_titles));
}

fn get_key_by_value(map: &HashMap<String, usize>, target_value: usize) -> Option<&String> {
    for (key, value) in map.iter() {
        if *value == target_value {
            return Some(key);
        }
    }
    None
}

#[test]
fn test() {
    
}
