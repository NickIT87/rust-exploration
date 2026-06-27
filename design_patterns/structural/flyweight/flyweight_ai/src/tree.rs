use std::rc::Rc;

use crate::tree_type::TreeType;

pub struct Tree {
    x: i32,
    y: i32,
    tree_type: Rc<TreeType>,
}

impl Tree {
    pub fn new(
        x: i32,
        y: i32,
        tree_type: Rc<TreeType>,
    ) -> Self {
        Self {
            x,
            y,
            tree_type,
        }
    }

    pub fn draw(&self) {
        println!(
            "{} at ({}, {}) uses texture {}",
            self.tree_type.species,
            self.x,
            self.y,
            self.tree_type.texture
        );
    }
}