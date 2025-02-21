pub fn di_string_match(s: String) -> Vec<i32> {
    let n: usize = s.len();
    let mut front = 0;
    let mut last: i32 = n as i32;
    let mut result: Vec<i32> = Vec::with_capacity(n + 1);
    for i in s.chars() {
        if front > last {
            return Vec::new();
        }
        match i {
            'I' => {
                result.push(front);
                front += 1;
            }
            'D' => {
                result.push(last);
                last -= 1;
            }
            _ => return Vec::new(), // Handle any other character by returning empty vector
        }
    }
    result.push(front);
    result
}

fn main() {
    let pattern = "IDID";
    let result = di_string_match(pattern.to_string());
    println!("{:?}", result);
}
