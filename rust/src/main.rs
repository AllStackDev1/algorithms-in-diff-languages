mod special_pairs;
mod tree_constructor;

fn main() {
    println!("Special Pairs - RS {:?}", special_pairs::get(18));
    println!("Tree Constructor - RS {:?}", tree_constructor::get(vec!["(1,2)", "(2,4)", "(5,7)", "(7,2)", "(9,5)"]));
    println!("Tree Constructor - RS {:?}", tree_constructor::get(vec!["(1,2)", "(3,2)", "(2,12)", "(5,2)"]));
}
