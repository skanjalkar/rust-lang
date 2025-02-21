use std::io;

pub fn possible_string_count(word: String) -> i32 {
    let mut ans: i32 = 1;
    let chars: Vec<char> = word.chars().collect();
    let length: usize = chars.len();
    for i in 0..(length - 1) {
        let current: char = chars[i];
        let next: char = chars[i + 1];
        if current == next {
            ans += 1;
        }
    }
    return ans;
}

fn main() {
    let mut word = String::new();
    println!("Enter a string: ");
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    let result: i32 = possible_string_count(word);
    println!("Result: {}", result);
}
