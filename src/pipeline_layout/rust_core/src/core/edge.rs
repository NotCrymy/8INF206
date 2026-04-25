#[derive(Debug, Clone)]
pub struct Edge {
    pub original_source: i32,
    pub source: i32,
    pub target: i32,
    pub route: Vec<(f32, f32)>, // list of points for the edge route
    pub crossing: bool, // flag to indicate if the edge crosses another edge
}