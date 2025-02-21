pub fn smallest_number(pattern: String) -> String {
    let n: usize = pattern.len();
    let mut result: Vec<usize> = vec![0; n + 1];
    let mut stack: Vec<usize> = Vec::new();
    let mut count: usize = 1;
    let chars: Vec<char> = pattern.chars().collect();

    for i in 0..=n {
        stack.push(i);
        if i == n || chars.get(i).map_or(false, |&ch| ch == 'I') {
            while let Some(j) = stack.pop() {
                result[j] = count;
                count += 1;
            }
        }
    }

    result
        .into_iter()
        .map(|num| num.to_string())
        .collect::<String>()
}

fn main() {
    let pattern: String = String::from("DDD");
    println!("{}", smallest_number(pattern));
}
