pub fn can_make_subsequence(str1: String, str2: String) -> bool {
    let s1: Vec<char> = str1.chars().collect();
    let s2: Vec<char> = str2.chars().collect();
    let n = s1.len();
    let m = s2.len();
    let mut i = 0;
    let mut j = 0;
    while i < n && j < m {
        let c1 = s1[i];
        let c2 = s2[j];
        if c1 == c2 || (c2 as u32 - 'a' as u32 - (c1 as u32 - 'a' as u32) + 26) % 26 == 1 {
            j += 1;
        }
        i += 1;
        // println!("i: {}, j: {}", i, j);
        if j == m {
            return true;
        }
    }
    false
}

fn main() {
    let str1 = String::from("abc");
    let str2 = String::from("ad");
    println!("{}", can_make_subsequence(str1, str2));
}
