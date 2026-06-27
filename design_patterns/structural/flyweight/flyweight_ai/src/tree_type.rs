#[derive(Debug)]
pub struct TreeType {
    pub species: String,
    pub texture: String,
}

impl TreeType {
    pub fn new(species: &str, texture: &str) -> Self {
        Self {
            species: species.to_string(),
            texture: texture.to_string(),
        }
    }
}