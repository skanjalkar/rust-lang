use std::collections::HashSet;

pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let max_ele = grid.len() * grid.len();
    let mut missing = 0;
    let mut repeated = 0;
    
    let mut seen = HashSet::new();
    for row in grid {
        for num in row {
            if seen.contains(&num) {
                repeated = num;
            } else {
                seen.insert(num);
            }
        }
    }
    
    for num in 1..=max_ele as i32 {
        if !seen.contains(&num) {
            missing = num;
            break;
        }
    }
    
    vec![repeated, missing]
}

fn main() {
    let grid = vec![vec![1, 3], vec![2, 2]];
    let result = find_missing_and_repeated_values(grid);
    assert_eq!(result, vec![2, 4]);
}
