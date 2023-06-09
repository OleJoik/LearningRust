#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.height > rectangle.height && self.width > rectangle.width
    }

    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect2 = Rectangle {
        width: 15,
        height: 60,
    };

    let rect3 = Rectangle::square(30);

    dbg!(rect1.can_hold(&rect2));
    dbg!(rect1.can_hold(&rect3));

    let rect1 = dbg!(rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:#?}", rect1);
}
