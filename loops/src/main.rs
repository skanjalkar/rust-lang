fn main() {
    let a = [1, 2, 3, 4, 5];
    for element in a.iter() {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

    let size: usize = a.len();
    for number in 1..size + 1 {
        println!("{}", a[number]);
    }
}
