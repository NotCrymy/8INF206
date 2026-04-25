#[derive(Debug, Clone)]
pub struct GridLayout {
    pub layer_spacing: f32,
    pub node_spacing: f32,
    pub node_width: f32,
    pub node_height: f32,
}

impl GridLayout {
    pub fn new() -> Self {
        Self {
            layer_spacing: 100.0,
            node_spacing: 150.0,
            node_width: 50.0,
            node_height: 50.0,
        }
    }
}