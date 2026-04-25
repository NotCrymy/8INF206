// defining the graph module which will contain the Graph struct and its associated methods

use crate::core::edge::Edge;
use crate::core::node::Node;
use crate::core::layout_result::LayoutResult;
use crate::core::grid_layout::GridLayout;

use std::collections::HashMap;

#[derive(Debug)] // defining the Graph struct which contains a vector of nodes and edges
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    next_node_id: i32, // field to keep track of the next node id to assign when adding a new node
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            next_node_id: 0, // initialize the next node id to 0, it will be incremented each time a new node is added to ensure unique ids
        }
    }

    // ----- MAIN FUNCTIONS -----

    pub fn add_node(&mut self, string: String) {
        let id = self.next_node_id; // assign the current next node id to the new node
        self.next_node_id += 1; // increment the next node id for the next node to be added

        let node = Node {
            id,
            level: 0, // level wil be compute during a topological sort of the graph
            string,
            x: 0.0, // x and y will be computed later in the layout algorithm 
            y: 0.0,
            is_dummy: false, // set the is_dummy flag to false for this node since it's a real node and not a dummy node added for layout purposes
        };
        self.nodes.push(node);
    }

    pub fn add_dummy_node(&mut self, string: String) -> i32 {
        let id = self.next_node_id; // assign the current next node id to the new dummy node
        self.next_node_id += 1; // increment the next node id for the next node to be added

        let node = Node {
            id,
            level: 0, // level wil be compute during a topological sort of the graph
            string,
            x: 0.0, // x and y will be computed later in the layout algorithm 
            y: 0.0,
            is_dummy: true, // set the is_dummy flag to true for this node
        };
        self.nodes.push(node);

        id // return the id of the newly added dummy node so it can be used when adding edges that need to connect to this dummy node
    }

    pub fn add_edge(&mut self, source: i32, target: i32) {
        let edge = Edge {
            original_source: source,
            source,
            target,
            route: Vec::new(), // initialize the route as an empty vector, it will be computed later in the layout algorithm
            crossing: false,
        };
        self.edges.push(edge);
    }

    // function to compute the layout of the graph and return a LayoutResult struct containing the positions of the nodes and the routes of the edges
    pub fn compute_layout_result(&mut self) -> LayoutResult {
        let grid = GridLayout::new();

        // KEEP THE ORDER OF THESE STEPS
        self.compute_node_levels();
        self.expand_long_edges(); 
        self.compute_horizontal_positions(&grid);
        self.route_edges_straight();
        self.crossing_edges();

        LayoutResult {
            nodes_positions: self.nodes
                .iter()
                .map(|node| (node.id, node.string.clone(), node.x, node.y, node.is_dummy))
                .collect(),

            edges_routes: self.edges
                .iter()
                .map(|edge| (edge.original_source, edge.route.clone(), edge.crossing))
                .collect(),
        }
    }

    // ----- LAYOUT ALGORITHM STEPS -----

    // fonction to expand long edges that span multiple levels by adding dummy nodes in between
    pub fn expand_long_edges(&mut self) {
        let old_edges = self.edges.clone();
        let mut new_edges = Vec::new();

        for edge in old_edges {
            // on copie les valeurs, PAS les références
            let source_level = self.nodes
                .iter()
                .find(|n| n.id == edge.source)
                .unwrap()
                .level;

            let target_level = self.nodes
                .iter()
                .find(|n| n.id == edge.target)
                .unwrap()
                .level;

            let diff = target_level - source_level;

            if diff <= 1 {
                new_edges.push(edge);
                continue;
            }

            let mut prev = edge.source;

            for i in 1..diff {
                let dummy_id = self.add_dummy_node("".to_string());

                let dummy_node = self.nodes.iter_mut().find(|n| n.id == dummy_id).unwrap();
                dummy_node.level = source_level + i;

                new_edges.push(Edge {
                    original_source: edge.original_source,
                    source: prev,
                    target: dummy_id,
                    route: Vec::new(),
                    crossing: false,
                });

                prev = dummy_id;
            }

            new_edges.push(Edge {
                original_source: edge.original_source,
                source: prev,
                target: edge.target,
                route: Vec::new(),
                crossing: false,
            });
        }

        self.edges = new_edges;
    }

    pub fn crossing_edges(&mut self) {
        for i in 0..self.edges.len() {
            for j in (i + 1)..self.edges.len() {
                let (left, right) = self.edges.split_at_mut(j);

                let edge1 = &mut left[i];
                let edge2 = &mut right[0];

                // ignore connected edges (important!)
                if edge1.source == edge2.source
                    || edge1.source == edge2.target
                    || edge1.target == edge2.source
                    || edge1.target == edge2.target
                {
                    continue;
                }

                // segments
                let segment1 = match (edge1.route.first(), edge1.route.last()) {
                    (Some(p1), Some(p2)) if p1 != p2 => (p1, p2),
                    _ => continue, // ignore edges with no route or zero-length route (should not happen with straight routing, but just in case)
                };

                let segment2 = match (edge2.route.first(), edge2.route.last()) {
                    (Some(p1), Some(p2)) if p1 != p2 => (p1, p2),
                    _ => continue, // ignore edges with no route or zero-length route (should not happen with straight routing, but just in case)
                };

                // function to compute the orientation of the triplet (p, q, r)
                fn orientation(p: &(f32, f32), q: &(f32, f32), r: &(f32, f32)) -> i32 {
                    let val = (q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1);
                    let eps = 1e-6;

                    if val > eps {
                        1
                    } else if val < -eps {
                        2
                    } else {
                        0
                    }
                }

                let o1 = orientation(segment1.0, segment1.1, segment2.0);
                let o2 = orientation(segment1.0, segment1.1, segment2.1);
                let o3 = orientation(segment2.0, segment2.1, segment1.0);
                let o4 = orientation(segment2.0, segment2.1, segment1.1);

                // ignore collinear segments (should not happen with straight routing, but just in case)
                if o1 == 0 || o2 == 0 || o3 == 0 || o4 == 0 {
                    continue;
                }

                // intersection test
                if o1 != o2 && o3 != o4 {
                    edge1.crossing = true;
                    edge2.crossing = true;
                }
            }
        }
    }

    // function to route edges as straight lines between the source and target nodes
    pub fn route_edges_straight(&mut self) {
        for edge in &mut self.edges {
            let source = self.nodes
                .iter()
                .find(|n| n.id == edge.source)
                .unwrap();

            let target = self.nodes
                .iter()
                .find(|n| n.id == edge.target)
                .unwrap();

            let (sx, sy) = (source.x, source.y);
            let (tx, ty) = (target.x, target.y);

            edge.route = vec![
                (sx, sy),
                (tx, ty),
            ];
        }
    }

    // function to compute the horizontal positions of the nodes in the graph using a barycenter method
    pub fn compute_horizontal_positions(&mut self, grid: &GridLayout) {
        // node_id -> index
        let mut node_map: HashMap<i32, usize> = HashMap::new();
        for (i, node) in self.nodes.iter().enumerate() {
            node_map.insert(node.id, i);
        }

        // group by level
        let mut layers: HashMap<i32, Vec<i32>> = HashMap::new();
        for node in &self.nodes {
            layers.entry(node.level).or_default().push(node.id);
        }

        let mut sorted_levels: Vec<i32> = layers.keys().cloned().collect();
        sorted_levels.sort();

        // barycentric ordering per layer
        for level in sorted_levels {
            let node_ids = layers.get(&level).unwrap();

            let mut scored: Vec<(i32, f32)> = node_ids
                .iter()
                .map(|&node_id| {
                    let parents: Vec<i32> = self.edges
                        .iter()
                        .filter(|e| e.target == node_id)
                        .map(|e| e.source)
                        .collect();

                    let score = if parents.is_empty() {
                        node_id as f32
                    } else {
                        let sum: f32 = parents
                            .iter()
                            .map(|p| {
                                let idx = node_map[p];
                                self.nodes[idx].x
                            })
                            .sum();

                        sum / parents.len() as f32
                    };

                    (node_id, score)
                })
                .collect();

            scored.sort_by(|a, b| {
                a.1.partial_cmp(&b.1)
                    .unwrap()
                    .then(a.0.cmp(&b.0))
            });

            // assign x positions (uniform spacing)
            for (i, (node_id, _)) in scored.iter().enumerate() {
                let idx = node_map[node_id];

                self.nodes[idx].x =
                    i as f32 * grid.node_spacing + grid.node_width / 2.0;

                self.nodes[idx].y =
                    level as f32 * grid.layer_spacing + grid.node_height / 2.0;
            }

            // centering this layer (only real visual adjustment)
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;

            let indices: Vec<usize> = scored
                .iter()
                .map(|(id, _)| node_map[id])
                .collect();

            for &i in &indices {
                let x = self.nodes[i].x;
                min_x = min_x.min(x);
                max_x = max_x.max(x);
            }

            let layer_center = (min_x + max_x) / 2.0;
            let shift = -layer_center;

            for &i in &indices {
                self.nodes[i].x += shift;
            }
        }
    }

    // function to compute the levels of each node in the graph using a topological sort + propagation approach
    pub fn compute_node_levels(&mut self) {
        let mut in_degree: HashMap<i32, i32> = HashMap::new();

        // init
        for node in &self.nodes{
            in_degree.insert(node.id, 0);
        }

        for edge in &self.edges {
            *in_degree.get_mut(&edge.target).unwrap() += 1;
        }

        // queue of sources 
        let mut queue: Vec<i32> = in_degree
            .iter()
            .filter(|&(_, &deg)| deg == 0) // filter the nodes that have an in-degree of 0, meaning they have no dependencies and can be processed first
            .map(|(&id, _)| id) // extract the node ids of the sources to be added to the queue for processing
            .collect();
        
        queue.sort(); // sort the queue to ensure a stable order of processing for nodes with the same level

        // map to stock levels
        let mut levels: HashMap<i32, i32> = HashMap::new();

        // initialize levels of sources to 0
        for &id in &queue {
            levels.insert(id, 0);
        }

        while let Some(current) = queue.pop() { // Some is used to handle the case where the queue is empty, in which case the loop will terminate
            let current_level = levels[&current]; // get the current level of the node being processed

            for edge in self.edges.iter().filter(|e| e.source == current) { // iterate over the edges that have the current node as source
                let next = edge.target; // get the target node of the edge being processed

                let entry = levels.entry(next).or_insert(0); // get the level of the target node, if it doesn't exist in the levels map, insert it with a default value of 0
                *entry = (*entry).max(current_level + 1); // update the level of the target node to be the maximum of its current level and the level of the current node + 1 (to ensure that the target node is at least one level below the current node)

                let deg = in_degree.get_mut(&next).unwrap(); // get the in-degree of the target node
                *deg -= 1; // decrement the in-degree of the target node since we have processed one of its incoming edges

                if *deg == 0 { // if the in-degree of the target node is now 0, it means that all its dependencies have been processed and it can be added to the queue for processing
                    queue.push(next); // add the target node to the queue for processing in the next iterations of the loop
                    queue.sort(); // sort the queue to ensure a stable order of processing for nodes with the same level
                }
            }
        }

        //apply to nodes
        for node in &mut self.nodes {
            node.level = *levels.get(&node.id).unwrap_or(&0);
        }
    }
}

#[cfg(test)] // only compile the tests module when running tests
mod tests {
    use super::*; // acess the parent module to use Graph, Node, and Edge

    #[test] // function to execute a test case
    fn test_add_node_and_edge() {
        let mut graph = Graph::new();

        graph.add_node("Node 1".to_string());
        graph.add_node("Node 2".to_string());
        graph.add_edge(1, 2);

        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.edges.len(), 1);
    }
}