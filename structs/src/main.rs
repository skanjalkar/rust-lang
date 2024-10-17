#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn volumne(&self, depth: u32) -> u32 {
        return self.width * self.height * depth;
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return self.width > rect.width && self.height > rect.height;
    }
}

fn main() {
    let width1: u32 = 30;
    let height1: u32 = 50;

    let rect1 = (30, 50);

    let rect2 = Rectangle {
        width: dbg!(30 * width1),
        height: 50,
    };

    println!("Area of rectangle is given by {}", area(width1, height1));

    println!("Area of rectangle is given by {}", area_tuple(rect1));

    println!("Area of rectangle is given by {}", area_struct(&rect2));

    println!("Rectangle struct is {:#?}", &rect2);

    dbg!(&rect2);

    println!("Area of rectangle is given by {}", rect2.area());

    println!("Volumne of rectangle is given by {}", rect2.volumne(10));

    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    let rect5 = Rectangle {
        width: 40,
        height: 60,
    };

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
    println!("Can rect2 hold rect5? {}", rect2.can_hold(&rect5));
}
