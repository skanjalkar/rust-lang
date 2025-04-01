use std::cmp::max;
use std::cmp::min;

pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
    let n = questions.len();
    let mut dp = vec![0i64; n+1];
    for i in (0..n).rev() {
        // you get to either choose your current question or skip it
        // if you choose it, then you can't pick the next questions[i][1]
        // so you can only choose your current question if 
        // so best if we start from the end?
        let points = questions[i][0] as i64;
        // now choose your current question
        let jump = questions[i][1] as usize;
        
        let next_idx = min(i + jump + 1, n);
        let take_current = points + dp[next_idx];
        
        let skip_current = dp[i+1];
        dp[i] = max(take_current, skip_current);

    }
    return dp[0];
}

fn main() {
    let questions = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
    let ans: i64 = most_points(questions);
    println!("{:?}", ans);
}
