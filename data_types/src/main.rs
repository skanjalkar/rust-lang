fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess is: {guess}");
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x, y: {x}, {y}");
    let sum = 5 + 10;
    let diff = 95.5 - 4.3;
    let div = 5 / 3;
    println!("Sum: {sum}, diff: {diff}, div: {div}");
    
    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of b is: {b}");
    let one = tup.0;
    println!("One: {one}");    
}
