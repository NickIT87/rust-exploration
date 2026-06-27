mod tree;
mod tree_factory;
mod tree_type;

use tree::Tree;
use tree_factory::TreeFactory;

fn main() {
    let mut factory = TreeFactory::new();

    let oak = factory.get_tree_type("Oak", "oak.png");
    let pine = factory.get_tree_type("Pine", "pine.png");

    let mut forest = Vec::new();

    forest.push(Tree::new(10, 20, oak.clone()));
    forest.push(Tree::new(15, 30, oak.clone()));
    forest.push(Tree::new(50, 70, oak.clone()));

    forest.push(Tree::new(100, 200, pine.clone()));
    forest.push(Tree::new(110, 210, pine.clone()));

    println!("Forest:");

    for tree in &forest {
        tree.draw();
    }

    println!();

    println!("Number of trees: {}", forest.len());
    println!("Unique TreeType objects: {}", factory.total_types());

    println!();

    println!(
        "Oak pointer 1: {:p}",
        oak.as_ref()
    );

    let another_oak = factory.get_tree_type("Oak", "oak.png");

    println!(
        "Oak pointer 2: {:p}",
        another_oak.as_ref()
    );
}