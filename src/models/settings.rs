use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct Path {
    pub public_files: String,
    pub views: String,
}