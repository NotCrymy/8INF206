#[derive(Debug, Clone)]
pub struct LayoutResult {
    pub nodes_positions: Vec<(i32, String, f32, f32, bool)>, // vector of tuples containing node id, string and its x, y positions
    pub edges_routes: Vec<(i32, Vec<(f32, f32)>, bool)>, // list of points for each edge route (sorted by source and target node ids)
}