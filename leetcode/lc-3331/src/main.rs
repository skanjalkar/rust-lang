use std::iter::empty;

pub fn dfs(
    children: &mut Vec<Vec<i32>>,
    visited: &mut Vec<bool>,
    node: i32,
    subtree_sizes: &mut Vec<i32>,
) {
    if visited[node as usize] {
        return;
    }
    if children[node as usize].is_empty() {
        visited[node as usize] = true;
        subtree_sizes[node as usize] = 1;
        return;
    }
    visited[node as usize] = true;
    for &child in &children[node as usize] {
        dfs(children, visited, child, subtree_sizes);
        subtree_sizes[node as usize] += subtree_sizes[child as usize];
    }
}

pub fn find_subtree_sizes(parent: Vec<i32>, s: String) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let mut children: Vec<Vec<i32>> = Vec::new();
    let mut visited: Vec<bool> = Vec::new();

    for _ in 0..parent.len() {
        children.push(Vec::new());
        visited.push(false);
    }

    for i in 1..parent.len() {
        // push children to parent
        children[parent[i] as usize].push(i as i32);
    }

    return ans;
}

fn main() {
    println!("Hello, world!");
}
