pub mod special_pairs;
pub mod tree_constructor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_special_pairs() {
        assert_eq!(special_pairs::get(18), [[5, 13], [7, 11]]);
        assert_eq!(special_pairs::get(2), Vec::<[i32; 0]>::new());
        assert_eq!(special_pairs::get(100).len(), 6);
    }

    #[test]
    fn ut_tree_constructor() {
        assert!(tree_constructor::get(vec![
            "(1,2)", "(2,4)", "(5,7)", "(7,2)", "(9,5)"
        ]));
        assert_ne!(
            tree_constructor::get(vec!["(1,2)", "(3,2)", "(2,12)", "(5,2)"]),
            true
        );

        assert_ne!(tree_constructor::get(vec![]), true);
    }
}
