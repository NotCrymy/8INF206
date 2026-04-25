use crate::core::layout_result::LayoutResult;
use crate::core::graph::Graph;

use pyo3::prelude::*;
use serde::{Serialize, Deserialize};

impl From<LayoutResult> for LayoutDTO {
    fn from(res: LayoutResult) -> Self {
        LayoutDTO {
            nodes_positions: res.nodes_positions,
            edges_routes: res.edges_routes,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct GraphFromJson {
    nodes: Vec<String>,
    edges: Vec<(i32, i32)>,
}

#[pyclass] // defining a Python class for the layout result to be returned to Python
pub struct LayoutDTO {
    #[pyo3(get)]
    pub nodes_positions: Vec<(i32, String, f32, f32, bool)>, // vector of tuples containing node id and its x, y positions
    #[pyo3(get)]
    pub edges_routes: Vec<(i32, Vec<(f32, f32)>, bool)>, // list of points for each edge route (sorted by source and target node ids)
}

#[pyfunction]
pub fn compute_layout_dto() -> PyResult<LayoutDTO> {
    let mut graph = Graph::new();

    let data = std::fs::read_to_string("graphs/graph.json")
        .expect("Unable to read file");
    
    let graph_from_json: GraphFromJson = 
        serde_json::from_str(&data)
        .expect("Unable to parse JSON");

    for node in graph_from_json.nodes {
        graph.add_node(node);
    }

    for (src, dest) in graph_from_json.edges {
        graph.add_edge(src, dest);
    }

    let layout_result = graph.compute_layout_result();

    Ok(LayoutDTO::from(layout_result))
}