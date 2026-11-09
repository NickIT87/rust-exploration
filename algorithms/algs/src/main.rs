mod binary_search;

fn main() {
    let numbers = [1, 3, 5, 7, 9];
    println!("{:?}", binary_search::binary_search(&numbers, &5));
}
