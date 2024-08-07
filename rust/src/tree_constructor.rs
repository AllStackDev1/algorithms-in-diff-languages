use std::collections::HashMap;

pub fn get(str_arr: Vec<&str>) -> bool {
    let mut parents: HashMap<String, Vec<String>> = HashMap::new();
    let mut children: HashMap<String, String> = HashMap::new();

    for &pair in &str_arr {
        // let cleaned_pair = pair.trim_matches(|c| c == '(' || c == ')');
        let cleaned_pair = pair.replace(['(', ')'], "");
        let nodes: Vec<_> = cleaned_pair.split(',').collect();
        let child = nodes[0].to_owned();
        let parent = nodes[1].to_owned();

        // Insert the child into the parent's children list
        parents
            .entry(parent.clone())
            .or_default()
            .push(child.clone());

        // If a parent has more than two children, it's not a valid binary tree
        if parents[&parent].len() > 2 {
            return false;
        }

        // If a child already has a parent, it's not a valid binary tree
        if children.contains_key(&child) {
            return false;
        }

        // Register the parent of the child
        children.insert(child, parent);
    }

    let root_count = parents
        .keys()
        .filter(|&parent| !children.contains_key(parent))
        .count();
    root_count == 1
}
