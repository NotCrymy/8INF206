#[derive(Debug, Clone)]
pub struct Node {
    pub id: i32,
    pub level: i32,
    pub string: String,
    pub x: f32,
    pub y: f32,
    pub is_dummy: bool, // flag to indicate if the node is a dummy node (added for layout purposes)
}