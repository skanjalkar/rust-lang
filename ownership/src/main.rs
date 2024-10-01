fn main() {
    let mut x = String::from("Hello");
    x.push_str(", world!");
    let z = x.clone();
    println!("{}", z.len());
    let zx = String::from(&z);
    println!("{}", zx.len());

    {
        let mut y = x.clone();
        x.push_str(&y);
        y.push_str("1");
        println!("{}", y);
    }

    println!("{}", x);

    let mut vector = Vec::new();
    vector.push(5);
    vector.push(6);
    vector.push(7);

    for element in vector.iter() {
        println!("{}", element);
    }

    let s = String::from("hello");

    let s2 = takes_ownership(s);

    println!("{}", s2);
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}
