use std::collections::HashMap;
use std::rc::Rc;

use crate::tree_type::TreeType;

pub struct TreeFactory {
    tree_types: HashMap<String, Rc<TreeType>>,
}

impl TreeFactory {
    pub fn new() -> Self {
        Self {
            tree_types: HashMap::new(),
        }
    }

    pub fn get_tree_type(
        &mut self,
        species: &str,
        texture: &str,
    ) -> Rc<TreeType> {
        self.tree_types
            .entry(species.to_string())
            .or_insert_with(|| Rc::new(TreeType::new(species, texture)))
            .clone()
    }

    pub fn total_types(&self) -> usize {
        self.tree_types.len()
    }
}